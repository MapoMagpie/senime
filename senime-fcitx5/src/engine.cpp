#include "engine.h"
#include "fcitx-utils/keysym.h"

#include <fcitx-utils/log.h>
#include <fcitx/event.h>
#include <fcitx/inputpanel.h>
#include <fcitx/userinterface.h>
#include <memory>

namespace fcitx {

namespace {

std::string lastError() {
    const char *error = senime_last_error();
    return error ? std::string(error) : std::string();
}

class SenimeCandidateWord : public CandidateWord {
public:
    SenimeCandidateWord(std::string text, std::string code, std::string selectKey, InputContext *ic)
        : CandidateWord(Text(text)), ic_(ic), text_(std::move(text)) {
            if (!code.empty()) {
                setComment(Text(std::move(code)));
            }
            if (!selectKey.empty()) {
                setCustomLabel(Text(std::move(selectKey)));
            }
        }

    void select(InputContext *) const override {
        ic_->commitString(text_);
    }

private:
    InputContext *ic_;
    std::string text_;
};

} // namespace

SenimeState::SenimeState(SenimeEngine *engine, InputContext *ic)
    : engine_(engine), ic_(ic) {}

void SenimeState::keyEvent(KeyEvent &event) {
    if (event.isRelease() || !engine_->engine()) {
        return;
    }

    const auto &key = event.key();
    // FCITX_INFO() << "Senime keyEvent: sym=" << key.sym()
    //              << " states=" << key.states()
    //              << " isRelease=" << event.isRelease()
    //              << " chineseMode=" << chineseMode();

    // Alt+J: 切换中文模式
    if (key.check(FcitxKey_J, KeyState::Alt)) {
    // if (key.check(FcitxKey_Shift_R, KeyState::NoState)) {
        if (chineseMode()) {
            commit();
            setChineseMode(false);
            // FCITX_INFO() << "Senime: Alt+I pressed, chineseMode -> OFF";
        } else {
            setChineseMode(true);
            // FCITX_INFO() << "Senime: Alt+I pressed, chineseMode -> ON";
            Text preedit(":(中)");
            // preedit.setCursor(preedit.toString().size());
            if (ic_->capabilityFlags().test(CapabilityFlag::Preedit)) {
                ic_->inputPanel().setClientPreedit(preedit);
            } else {
                ic_->inputPanel().setPreedit(preedit);
            }
            ic_->updatePreedit();
            ic_->updateUserInterface(UserInterfaceComponent::InputPanel);
        }
        ic_->updateUserInterface(UserInterfaceComponent::StatusArea);
        event.filterAndAccept();
        return;
    }
    // 英文模式下直接透传
    if (!chineseMode()) {
        return;
    }

    // Non-Shift modifier (Ctrl, Alt, Super...) → commit pending input, forward key to application.
    auto nonShiftMods = key.states() & ~(KeyStates(KeyState::Shift) | KeyState::CapsLock | KeyState::NumLock);
    if (nonShiftMods) {
        if (!input_.empty()) {
            commit();
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
            // Remove last UTF-8 character.
            auto pos = input_.size() - 1;
            while (pos > 0 && (static_cast<unsigned char>(input_[pos]) & 0xc0) == 0x80) {
                --pos;
            }
            input_.erase(pos);
            update();
            event.filterAndAccept();
        }
        return;
    }

    if (key.check(FcitxKey_Return)) {
        if (!input_.empty()) {
            commit();
        }
        if (key.states().test(KeyState::Shift)) {
            event.filterAndAccept();
        }
        return;
    }

    // When input is empty, space should be committed directly.
    if (key.check(FcitxKey_space) && input_.empty()) {
        ic_->commitString(" ");
        event.filterAndAccept();
        return;
    }

    // Let the engine handle everything else (letters, numbers, selection keys,
    // space, punctuation, etc.).
    auto utf8 = Key::keySymToUTF8(key.sym());
    if (!utf8.empty()) {
        input_ += utf8;
        update();
        event.filterAndAccept();
    }
}

void SenimeState::reset() {
    commit();
    input_.clear();
    chineseMode_ = false;
    ic_->inputPanel().reset();
    ic_->updatePreedit();
    ic_->updateUserInterface(UserInterfaceComponent::InputPanel);
    ic_->updateUserInterface(UserInterfaceComponent::StatusArea);
}

void SenimeState::commit() {
    if (input_.empty()) {
        return;
    }
    ic_->commitString(input_);
    input_.clear();
    ic_->inputPanel().reset();
    ic_->updatePreedit();
    ic_->updateUserInterface(UserInterfaceComponent::InputPanel);
}

void SenimeState::update() {
    ic_->inputPanel().reset();

    if (input_.empty()) {
        ic_->updatePreedit();
        ic_->updateUserInterface(UserInterfaceComponent::InputPanel);
        return;
    }

    SenimeAnalysis *analysis = senime_engine_analyze(engine_->engine(), input_.c_str());
    if (!analysis) {
        FCITX_WARN() << "Senime analyze failed: " << lastError();
        ic_->updatePreedit();
        ic_->updateUserInterface(UserInterfaceComponent::InputPanel);
        return;
    }

    const char *text = analysis->text ? analysis->text : "";

    // No candidates — everything resolved (unique codes, punctuation, etc.), commit.
    if (analysis->candidate_count == 0) {
        if (input_ != text) {
            ic_->commitString(text);
            input_.clear();
        } else {
            // Show input as preedit.
            Text preedit(text);
            preedit.setCursor(preedit.toString().size());
            if (ic_->capabilityFlags().test(CapabilityFlag::Preedit)) {
                ic_->inputPanel().setClientPreedit(preedit);
            } else {
                ic_->inputPanel().setPreedit(preedit);
            }
        }
        senime_analysis_free(analysis);
        ic_->updatePreedit();
        ic_->updateUserInterface(UserInterfaceComponent::InputPanel);
        return;
    }

    // Show preedit and candidate list.
    Text preedit(text);
    preedit.setCursor(preedit.toString().size());
    if (ic_->capabilityFlags().test(CapabilityFlag::Preedit)) {
        ic_->inputPanel().setClientPreedit(preedit);
    } else {
        ic_->inputPanel().setPreedit(preedit);
    }
    auto candidates = std::make_unique<CommonCandidateList>();
    candidates->setPageSize(engine_->instance()->globalConfig().defaultPageSize());
    candidates->setCursorPositionAfterPaging(CursorPositionAfterPaging::ResetToFirst);
    for (size_t i = 0; i < analysis->candidate_count; i++) {
        const auto &candidate = analysis->candidates[i];
        candidates->append<SenimeCandidateWord>(
            std::string(candidate.text ? candidate.text : ""),
            candidate.code ? candidate.code : "",
            candidate.select_key ? std::string(1, static_cast<char>(candidate.select_key)) + ": " : std::string(),
            ic_);
    }
    candidates->setGlobalCursorIndex(0);
    ic_->inputPanel().setCandidateList(std::move(candidates));

    senime_analysis_free(analysis);
    ic_->updatePreedit();
    ic_->updateUserInterface(UserInterfaceComponent::InputPanel);
}

std::string SenimeEngine::subModeIconImpl(const InputMethodEntry &,
                                          InputContext &ic) {
    auto *st = ic.propertyFor(&factory_);
    return st->chineseMode() ? "fcitx-senime-cn" : "fcitx-senime-en";
}

std::string SenimeEngine::subModeLabelImpl(const InputMethodEntry &,
                                           InputContext &ic) {
    auto *st = ic.propertyFor(&factory_);
    return st->chineseMode() ? "中" : "En";
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
