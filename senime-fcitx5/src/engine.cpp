#include "engine.h"
#include "fcitx-utils/keysym.h"

#include <fcitx-utils/log.h>
#include <fcitx/event.h>
#include <fcitx/inputpanel.h>
#include <fcitx/userinterface.h>
#include <chrono>
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

// ── SenimeState ──────────────────────────────────────────────────────────

SenimeState::SenimeState(SenimeEngine *engine, InputContext *ic)
    : engine_(engine), ic_(ic),
      state_(senime_state_new(engine->engine()), senime_state_free) {}

SenimeState::~SenimeState() = default;

void SenimeState::processKeyEvent(KeyEvent &event) {
    if (event.isRelease() || !engine_->engine()) {
        return;
    }

    const auto &key = event.key();

    // Convert Fcitx5 key event to flat C struct
    SenimeKeyEvent rustKey;
    rustKey.sym = static_cast<uint32_t>(key.sym());
    rustKey.states = static_cast<uint32_t>(key.states());
    rustKey.is_release = event.isRelease();

    auto start = std::chrono::steady_clock::now();
    SenimeKeyEventResult *result = senime_engine_process_key(
        engine_->engine(), state_.get(), &rustKey);
    auto end = std::chrono::steady_clock::now();

    auto us = std::chrono::duration_cast<std::chrono::microseconds>(end - start).count();
    FCITX_INFO() << "Senime process_key: " << us << "us";

    if (!result) {
        FCITX_WARN() << "Senime process_key failed: " << lastError();
        return;
    }

    if (result->accepted) {
        event.filterAndAccept();
    }

    executeCommands(result, ic_);
    senime_key_event_result_free(result);
}

void SenimeState::reset() {
    if (!state_) return;

    // Send Escape key to trigger reset logic in Rust
    SenimeKeyEvent escKey;
    escKey.sym = FcitxKey_Escape;
    escKey.states = 0;
    escKey.is_release = false;

    SenimeKeyEventResult *result = senime_engine_process_key(
        engine_->engine(), state_.get(), &escKey);

    if (result) {
        executeCommands(result, ic_);
        senime_key_event_result_free(result);
    }
}

void SenimeState::deactivate() {
    if (!state_) return;

    // Send Return key to commit pending input, then Escape to reset
    SenimeKeyEvent retKey;
    retKey.sym = FcitxKey_Return;
    retKey.states = 0;
    retKey.is_release = false;

    SenimeKeyEventResult *result = senime_engine_process_key(
        engine_->engine(), state_.get(), &retKey);

    if (result) {
        executeCommands(result, ic_);
        senime_key_event_result_free(result);
    }

    reset();
}

bool SenimeState::chineseMode() const {
    return state_ ? senime_state_chinese_mode(state_.get()) : false;
}

void SenimeState::executeCommands(SenimeKeyEventResult *result, InputContext *ic) {
    if (!result || !result->commands || result->command_count == 0) {
        return;
    }

    for (size_t i = 0; i < result->command_count; i++) {
        const SenimeCommand &cmd = result->commands[i];
        switch (cmd.type) {
        case SENIME_CMD_COMMIT_TEXT:
            if (cmd.text) {
                ic->commitString(cmd.text);
            }
            break;

        case SENIME_CMD_SET_PREEDIT: {
            Text preedit(cmd.text ? cmd.text : "");
            preedit.setCursor(preedit.toString().size());
            if (ic->capabilityFlags().test(CapabilityFlag::Preedit)) {
                ic->inputPanel().setClientPreedit(preedit);
            } else {
                ic->inputPanel().setPreedit(preedit);
            }
            break;
        }

        case SENIME_CMD_SET_CANDIDATES: {
            auto candidates = std::make_unique<CommonCandidateList>();
            candidates->setPageSize(
                engine_->instance()->globalConfig().defaultPageSize());
            candidates->setCursorPositionAfterPaging(
                CursorPositionAfterPaging::ResetToFirst);
            for (size_t j = 0; j < cmd.candidate_count; j++) {
                const auto &cand = cmd.candidates[j];
                candidates->append<SenimeCandidateWord>(
                    std::string(cand.text ? cand.text : ""),
                    cand.code ? cand.code : "",
                    cand.select_key
                        ? std::string(1, static_cast<char>(cand.select_key)) + ": "
                        : std::string(),
                    ic);
            }
            candidates->setGlobalCursorIndex(0);
            ic->inputPanel().setCandidateList(std::move(candidates));
            break;
        }

        case SENIME_CMD_CLEAR_INPUT_PANEL:
            ic->inputPanel().reset();
            break;

        case SENIME_CMD_UPDATE_PREEDIT:
            ic->updatePreedit();
            break;

        case SENIME_CMD_UPDATE_UI:
            ic->updateUserInterface(UserInterfaceComponent::InputPanel);
            break;

        case SENIME_CMD_UPDATE_STATUS_AREA:
            ic->updateUserInterface(UserInterfaceComponent::StatusArea);
            break;

        // case SENIME_CMD_FILTER_AND_ACCEPT:
        //     // Already handled via event.filterAndAccept() in processKeyEvent
        //     break;
        }
    }
}

// ── SenimeEngine ─────────────────────────────────────────────────────────

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
    state->processKeyEvent(event);
}

void SenimeEngine::reset(const InputMethodEntry &, InputContextEvent &event) {
    auto *state = event.inputContext()->propertyFor(&factory_);
    state->reset();
}

void SenimeEngine::deactivate(const InputMethodEntry &entry,
                              InputContextEvent &event) {
    auto *state = event.inputContext()->propertyFor(&factory_);
    state->deactivate();
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
