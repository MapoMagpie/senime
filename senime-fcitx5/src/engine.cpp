#include "engine.h"

#include <algorithm>
#include <cctype>
#include <fcitx-utils/capabilityflags.h>
#include <fcitx-utils/log.h>
#include <fcitx/event.h>
#include <fcitx/inputpanel.h>
#include <fcitx/userinterface.h>

namespace fcitx {

namespace {

std::string lastError() {
    const char *error = senime_last_error();
    return error ? std::string(error) : std::string();
}

bool isAsciiInput(uint32_t c) {
    return c >= 0x20 && c <= 0x7e;
}

void eraseLastUtf8(std::string &value) {
    if (value.empty()) {
        return;
    }
    auto pos = value.size() - 1;
    while (pos > 0 && (static_cast<unsigned char>(value[pos]) & 0xc0) == 0x80) {
        --pos;
    }
    value.erase(pos);
}

class SenimeCandidateWord : public CandidateWord {
public:
    SenimeCandidateWord(SenimeEngine *engine, size_t index, std::string text,
                        std::string code)
        : CandidateWord(Text(std::move(text))), engine_(engine), index_(index) {
        if (!code.empty()) {
            setComment(Text(std::move(code)));
        }
    }

    void select(InputContext *inputContext) const override {
        engine_->state(inputContext)->select(index_);
    }

private:
    SenimeEngine *engine_;
    size_t index_;
};

} // namespace

SenimeState::SenimeState(SenimeEngine *engine, InputContext *ic)
    : engine_(engine), ic_(ic) {}

SenimeState::~SenimeState() { clearAnalysis(); }

void SenimeState::clearAnalysis() {
    if (analysis_) {
        senime_analysis_free(analysis_);
        analysis_ = nullptr;
    }
}

KeyList SenimeState::selectionKeyList() const {
    KeyList keys;
    size_t len = 0;
    const char *sk = senime_engine_selection_keys(engine_->engine(), &len);
    if (sk) {
        for (size_t i = 0; i < len; i++) {
            keys.emplace_back(static_cast<KeySym>(sk[i]));
        }
    }
    return keys;
}

bool SenimeState::isSelectionKey(const Key &key, size_t *index) const {
    auto keys = selectionKeyList();
    auto idx = key.keyListIndex(keys);
    if (idx < 0) {
        return false;
    }
    if (index) {
        *index = static_cast<size_t>(idx);
    }
    return true;
}

bool SenimeState::appendKey(KeySym sym) {
    auto utf8 = Key::keySymToUTF8(sym);
    if (utf8.empty()) {
        return false;
    }
    auto unicode = Key::keySymToUnicode(sym);
    if (!isAsciiInput(unicode)) {
        return false;
    }
    input_ += utf8;
    return true;
}

void SenimeState::keyEvent(KeyEvent &event) {
    if (event.isRelease()) {
        return;
    }
    if (!engine_->engine()) {
        return;
    }

    const auto &key = event.key();

    // Selection keys must be checked before modifier handling,
    // so that Shift+number keys work for candidate selection.
    size_t selection = 0;
    if (!input_.empty() && isSelectionKey(key, &selection)) {
        select(selection);
        event.filterAndAccept();
        return;
    }

    if (key.hasModifier() && !key.states().test(KeyState::Shift)) {
        if (!input_.empty()) {
            commit();
            event.filterAndAccept();
        }
        return;
    }

    if (key.check(FcitxKey_Escape)) {
        reset();
        event.filterAndAccept();
        return;
    }

    if (key.check(FcitxKey_BackSpace)) {
        if (!input_.empty()) {
            eraseLastUtf8(input_);
            update();
            event.filterAndAccept();
        }
        return;
    }

    if (key.check(FcitxKey_Return) || key.check(FcitxKey_space)) {
        if (!input_.empty()) {
            commit();
            event.filterAndAccept();
        }
        return;
    }

    if (appendKey(key.sym())) {
        update();
        event.filterAndAccept();
    }
}

void SenimeState::reset() {
    input_.clear();
    clearAnalysis();
    updatePreedit();
}

void SenimeState::commit() {
    if (input_.empty() || !engine_->engine()) {
        reset();
        return;
    }
    // Use stored analysis if available, otherwise analyze now.
    if (!analysis_) {
        analysis_ = senime_engine_analyze(engine_->engine(), input_.c_str());
    }
    if (analysis_ && analysis_->text && analysis_->text[0] != '\0') {
        ic_->commitString(analysis_->text);
    } else {
        ic_->commitString(input_);
    }
    input_.clear();
    clearAnalysis();
    updatePreedit();
}

void SenimeState::select(size_t index) {
    if (input_.empty()) {
        return;
    }
    // Ensure we have an analysis result.
    if (!analysis_ && engine_->engine()) {
        analysis_ = senime_engine_analyze(engine_->engine(), input_.c_str());
    }
    if (!analysis_ || index >= analysis_->candidate_count) {
        return;
    }
    const auto &candidate = analysis_->candidates[index];
    if (candidate.text && candidate.text[0] != '\0') {
        ic_->commitString(candidate.text);
    }
    input_.clear();
    clearAnalysis();
    updatePreedit();
}

void SenimeState::updatePreedit() {
    ic_->inputPanel().reset();
    Text preedit(input_);
    preedit.setCursor(input_.size());
    if (ic_->capabilityFlags().test(CapabilityFlag::Preedit)) {
        ic_->inputPanel().setClientPreedit(preedit);
    } else {
        ic_->inputPanel().setPreedit(preedit);
    }
    ic_->updatePreedit();
    ic_->updateUserInterface(UserInterfaceComponent::InputPanel);
}

void SenimeState::update() {
    ic_->inputPanel().reset();
    clearAnalysis();

    if (input_.empty() || !engine_->engine()) {
        updatePreedit();
        return;
    }

    analysis_ = senime_engine_analyze(engine_->engine(), input_.c_str());
    if (!analysis_) {
        FCITX_WARN() << "Senime analyze failed: " << lastError();
        updatePreedit();
        return;
    }

    const char *text = analysis_->text ? analysis_->text : "";

    // All codes resolved to unique matches (no candidates), auto-commit.
    if (analysis_->candidate_count == 0 && input_ != text) {
        ic_->commitString(text);
        input_.clear();
        clearAnalysis();
        updatePreedit();
        return;
    }

    Text preedit(text);
    preedit.setCursor(preedit.toString().size());
    if (ic_->capabilityFlags().test(CapabilityFlag::Preedit)) {
        ic_->inputPanel().setClientPreedit(preedit);
    } else {
        ic_->inputPanel().setPreedit(preedit);
    }

    if (analysis_->candidate_count > 0) {
        auto candidates = std::make_unique<CommonCandidateList>();
        candidates->setSelectionKey(selectionKeyList());
        candidates->setPageSize(engine_->instance()->globalConfig().defaultPageSize());
        candidates->setCursorPositionAfterPaging(CursorPositionAfterPaging::ResetToFirst);
        for (size_t i = 0; i < analysis_->candidate_count; i++) {
            const auto &candidate = analysis_->candidates[i];
            candidates->append<SenimeCandidateWord>(
                engine_, i, candidate.text ? candidate.text : "",
                candidate.code ? candidate.code : "");
        }
        candidates->setGlobalCursorIndex(0);
        ic_->inputPanel().setCandidateList(std::move(candidates));
    }

    ic_->updatePreedit();
    ic_->updateUserInterface(UserInterfaceComponent::InputPanel);
}

SenimeEngine::SenimeEngine(Instance *instance)
    : instance_(instance),
      factory_([this](InputContext &ic) { return new SenimeState(this, &ic); }) {
    reloadConfig();
    reloadEngine();
    instance_->inputContextManager().registerProperty("senimeState", &factory_);
}

void SenimeEngine::reloadEngine() {
    engine_.reset();
    if (config_.tablePath->empty()) {
        FCITX_WARN() << "Senime table path is empty.";
        return;
    }
    engine_.reset(senime_engine_new(config_.tablePath->c_str()));
    if (!engine_) {
        FCITX_WARN() << "Failed to load Senime table: " << lastError();
    }
}

void SenimeEngine::keyEvent(const InputMethodEntry &, KeyEvent &event) {
    auto *state = event.inputContext()->propertyFor(&factory_);
    state->keyEvent(event);
}

void SenimeEngine::reset(const InputMethodEntry &, InputContextEvent &event) {
    auto *state = event.inputContext()->propertyFor(&factory_);
    state->reset();
}

void SenimeEngine::deactivate(const InputMethodEntry &entry,
                              InputContextEvent &event) {
    auto *state = event.inputContext()->propertyFor(&factory_);
    state->commit();
    reset(entry, event);
}

void SenimeEngine::reloadConfig() { readAsIni(config_, "conf/senime.conf"); }

void SenimeEngine::setConfig(const RawConfig &rawConfig) {
    config_.load(rawConfig, true);
    safeSaveAsIni(config_, "conf/senime.conf");
    reloadEngine();
    instance_->inputContextManager().foreach([this](InputContext *ic) {
        state(ic)->reset();
        return true;
    });
}

SenimeState *SenimeEngine::state(InputContext *ic) {
    return ic->propertyFor(&factory_);
}

} // namespace fcitx

FCITX_ADDON_FACTORY_V2(senime, fcitx::SenimeEngineFactory);
