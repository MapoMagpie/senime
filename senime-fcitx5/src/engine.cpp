#include "engine.h"
#include "fcitx-utils/keysym.h"
#include <fcitx-utils/key.h>

#include <fcitx-utils/log.h>
#include <fcitx/event.h>
#include <fcitx/inputpanel.h>
#include <fcitx/statusarea.h>
#include <fcitx/userinterface.h>
#include <fcitx/userinterfacemanager.h>
// #include <chrono>
#include <functional>
#include <memory>

namespace fcitx {

namespace {

// Fcitx5 日志桥接：Rust 层通过回调将日志转发到 fcitx5 日志系统
void senimeFcitxLog(int level, const char *msg) {
    switch (level) {
    case 0: FCITX_INFO() << "[senime] " << msg; break;
    case 1: FCITX_WARN() << "[senime] " << msg; break;
    case 2: FCITX_ERROR() << "[senime] " << msg; break;
    default: FCITX_INFO() << "[senime] " << msg; break;
    }
}

class SenimeCandidateWord : public CandidateWord {
public:
    SenimeCandidateWord(std::string text, std::string code, std::string selectKey, InputContext *ic,
                        std::function<void()> resetCallback)
        : CandidateWord(Text(text)), ic_(ic), text_(std::move(text)),
          resetCallback_(std::move(resetCallback)) {
            if (!code.empty()) {
                setComment(Text(std::move(code)));
            }
            if (!selectKey.empty()) {
                setCustomLabel(Text(std::move(selectKey)));
            }
        }

    void select(InputContext *) const override {
        ic_->commitString(text_);
        if (resetCallback_) {
            resetCallback_();
        }
    }

private:
    InputContext *ic_;
    std::string text_;
    std::function<void()> resetCallback_;
};

} // namespace

// ── SenimeState ──────────────────────────────────────────────────────────

SenimeState::SenimeState(SenimeEngine *engine, InputContext *ic)
    : engine_(engine), ic_(ic),
      state_(senime_state_new(engine->engine()), senime_state_free) {}

SenimeState::~SenimeState() = default;

void SenimeState::processKeyEvent(KeyEvent &event) {
    const auto &key = event.key();

    // Convert Fcitx5 key event to flat C struct
    SenimeKeyEvent rustKey;
    rustKey.sym = static_cast<uint32_t>(key.sym());
    rustKey.states = static_cast<uint32_t>(key.states());
    rustKey.is_release = event.isRelease();

    // auto start = std::chrono::steady_clock::now();
    SenimeKeyEventResult *result = senime_engine_key_event(
        engine_->engine(), state_.get(), &rustKey);
    // auto end = std::chrono::steady_clock::now();

    // auto us = std::chrono::duration_cast<std::chrono::microseconds>(end - start).count();
    // FCITX_INFO() << "Senime process_key: " << us << "us";

    if (!result) {
        return;
    }

    if (result->accepted) {
        event.filterAndAccept();
    }

    executeCommands(result, ic_);
    senime_key_event_result_free(result);
}

void SenimeState::reset(bool reset_mode) {
    if (!state_) return;

    // 直接清空 inputContext 的预编辑和候选框
    ic_->inputPanel().reset();
    ic_->updatePreedit();
    ic_->updateUserInterface(UserInterfaceComponent::InputPanel);

    // 重置 Rust 侧状态
    if (reset_mode) {
        senime_state_reset(state_.get());
    } else {
        senime_state_reset_input(state_.get());
    }
}

void SenimeState::reloadEngine() {
    // 销毁旧的 Rust 状态，用新引擎重新创建
    state_.reset(senime_state_new(engine_->engine()));
}

bool SenimeState::chineseMode() const {
    return state_ ? senime_state_chinese_mode(state_.get()) : false;
}

void SenimeState::setChineseMode(bool chinese) {
    if (!state_) return;
    senime_state_set_chinese_mode(state_.get(), chinese);
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
            ic->updatePreedit();
            break;
        }

        case SENIME_CMD_SET_CANDIDATES: {
            auto candidates = std::make_unique<CommonCandidateList>();
            candidates->setPageSize(
                engine_->instance()->globalConfig().defaultPageSize());
            candidates->setCursorPositionAfterPaging(
                CursorPositionAfterPaging::ResetToFirst);
            // 手动点击候选项后，重置输入状态但保留中英模式
            auto resetCallback = [this]() { this->reset(false); };
            for (size_t j = 0; j < cmd.candidate_count; j++) {
                const auto &cand = cmd.candidates[j];
                candidates->append<SenimeCandidateWord>(
                    std::string(cand.text ? cand.text : ""),
                    cand.code ? cand.code : "",
                    cand.select_key
                        ? std::string(1, static_cast<char>(cand.select_key)) + ": "
                        : std::string(),
                    ic, resetCallback);
            }
            candidates->setGlobalCursorIndex(0);
            ic->inputPanel().setCandidateList(std::move(candidates));
            break;
        }

        case SENIME_CMD_RESET_INPUT_PANEL:
            ic->inputPanel().reset();
            break;

        case SENIME_CMD_UPDATE_UI:
            ic->updateUserInterface(UserInterfaceComponent::InputPanel);
            break;

        case SENIME_CMD_UPDATE_STATUS_AREA:
            ic->updateUserInterface(UserInterfaceComponent::StatusArea);
            break;
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
    return st->chineseMode() ? "中" : "EN";
}

SenimeEngine::SenimeEngine(Instance *instance)
    : instance_(instance),
      factory_([this](InputContext &ic) { return new SenimeState(this, &ic); }) {
    // 设置 Rust 层日志回调，将日志转发到 fcitx5 日志系统
    static bool log_initialized = false;
    if (!log_initialized) {
        senime_set_log_callback(senimeFcitxLog);
        log_initialized = true;
    }
    reloadConfig();
    config_ = convertConfig(configDef_);
    globalChineseMode_.store(config_.default_chinese_mode);
    reloadEngine();
    instance_->inputContextManager().registerProperty("senimeState", &factory_);

    // 注册托盘菜单项
    toggleChineseAction_.connect<SimpleAction::Activated>(
        [this](InputContext *ic) {
            bool newMode = !globalChineseMode_.load();
            globalChineseMode_.store(newMode);
            instance_->inputContextManager().foreach([this, newMode](InputContext *ic) {
                state(ic)->setChineseMode(newMode);
                return true;
            });
            updateAction(ic);
        });
    instance_->userInterfaceManager().registerAction(
        "senime-toggle-chinese", &toggleChineseAction_);
}

// # 单`Shift_L`在~/.config/fcitx5/conf/senime.conf中的情况
// Shift+Shift_L
// # 实际按下左Shift后的key值
// key event: sym: [65505], state: [0]

// # 单`Shift_R`在~/.config/fcitx5/conf/senime.conf中的情况
// Shift+Shift_R
// # 实际按下右Shift后的key值
// key event: sym: [65506], state: [0]

// # 组合键+字母键在~/.config/fcitx5/conf/senime.conf中的情况
// Alt+J
// # 实际按下Alt+j后的key值
// key event: sym: [74], state: [8]

// # 双组合键在~/.config/fcitx5/conf/senime.conf中的情况
// Control+Shift+J
// # 实际按下Ctrl+Shift+J后的key值
// key event: sym: [74], state: [5]

// # 以下是其他按键对应的SenimeKeyEvent值
// Shift_L+j
// key event: sym: [74], state: [0]
// Shift_R+j
// key event: sym: [74], state: [0]
// 单独的j
// key event: sym: [106], state: [0]
// Ctrl+Shift_L
// key event: sym: [65505], state: [4]
// Shift_L+Return
// key event: sym: [65293], state: [1]
// Shift_R+Return
// key event: sym: [65293], state: [1]
SenimeConfig SenimeEngine::convertConfig(const SenimeFcitxConfig &cfg) {
    SenimeConfig kc{};
    auto extract = [](const KeyList &list, uint32_t &sym, uint32_t &states) {
        if (!list.empty()) {
            auto key = list[0];
            sym = static_cast<uint32_t>(key.sym());
            states = static_cast<uint32_t>(key.states());
        }
    };
    auto extract_char = [](const std::string &s, uint32_t &sym) {
        if (!s.empty()) {
            sym = static_cast<uint32_t>(static_cast<unsigned char>(s[0]));
        }
    };
    extract(*cfg.toggleMode, kc.toggle_sym, kc.toggle_states);
    extract_char(*cfg.triggerTempChineseStart, kc.trigger_start_char);
    extract_char(*cfg.triggerTempChineseEnd, kc.trigger_end_char);
    kc.table_path = cfg.tablePath->c_str();
    kc.default_chinese_mode = *cfg.defaultChineseMode;
    kc.sentence_flow = *cfg.sentenceFlow;
    return kc;
}

void SenimeEngine::reloadEngine() {
    engine_.reset();
    engine_.reset(senime_engine_new(&config_));
}

void SenimeEngine::activate(const InputMethodEntry &,
                            InputContextEvent &event) {
    event.inputContext()->statusArea().addAction(StatusGroup::InputMethod, &toggleChineseAction_);
    updateAction(event.inputContext());
}

void SenimeEngine::updateAction(InputContext *ic) {
    bool chinese = globalChineseMode_.load();
    toggleChineseAction_.setIcon(chinese ? "fcitx-senime-cn" : "fcitx-senime-en");
    toggleChineseAction_.setShortText(chinese ? _("Switch to English")
                                              : _("Switch to Chinese"));
    toggleChineseAction_.update(ic);
}

void SenimeEngine::keyEvent(const InputMethodEntry &, KeyEvent &event) {
    auto *st = event.inputContext()->propertyFor(&factory_);
    st->processKeyEvent(event);
}

void SenimeEngine::reset(const InputMethodEntry &, InputContextEvent &event) {
    auto *state = event.inputContext()->propertyFor(&factory_);
    state->reset();
}

void SenimeEngine::deactivate(const InputMethodEntry &entry, InputContextEvent &event) {
    reset(entry, event);
}

void SenimeEngine::reloadConfig() { readAsIni(configDef_, "conf/senime.conf"); }

void SenimeEngine::setConfig(const RawConfig &rawConfig) {
    configDef_.load(rawConfig, true);
    safeSaveAsIni(configDef_, "conf/senime.conf");
    config_ = convertConfig(configDef_);
    reloadEngine();
    instance_->inputContextManager().foreach([this](InputContext *ic) {
        state(ic)->reloadEngine();
        return true;
    });
}

SenimeState *SenimeEngine::state(InputContext *ic) {
    return ic->propertyFor(&factory_);
}

} // namespace fcitx

FCITX_ADDON_FACTORY_V2(senime, fcitx::SenimeEngineFactory);
