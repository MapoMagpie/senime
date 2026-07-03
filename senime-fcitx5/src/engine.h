#ifndef SENIME_FCITX5_ENGINE_H_
#define SENIME_FCITX5_ENGINE_H_

#include "senime_fcitx5.h"

#include <atomic>
#include <memory>
#include <string>
#include <fcitx-config/configuration.h>
#include <fcitx-config/iniparser.h>
#include <fcitx-config/option.h>
#include <fcitx-utils/key.h>
#include <fcitx/addonfactory.h>
#include <fcitx/candidatelist.h>
#include <fcitx/inputcontext.h>
#include <fcitx/inputcontextmanager.h>
#include <fcitx/inputcontextproperty.h>
#include <fcitx/inputmethodengine.h>
#include <fcitx/instance.h>
#include <fcitx/addonmanager.h>
#include <fcitx/action.h>
#include <fcitx/menu.h>
#include <fcitx/statusarea.h>
#include <fcitx/text.h>
#include <fcitx-utils/i18n.h>

namespace fcitx {

FCITX_CONFIGURATION(
    SenimeFcitxConfig,
    Option<std::string> tablePath{this, "TablePath", _("Table Path"), ""};
    Option<KeyList, ListConstrain<KeyConstrain>> toggleMode{
        this, "ToggleMode", _("Toggle Chinese/English"),
        KeyList{Key("Shift+Shift_L")},
        KeyListConstrain(KeyConstrainFlag::AllowModifierOnly)};
    Option<std::string> triggerTempChineseStart{this, "TriggerTempChineseStart",
                                                _("Trigger Temporary Chinese Start"), ""};
    Option<std::string> triggerTempChineseEnd{this, "TriggerTempChineseEnd",
                                              _("Trigger Temporary Chinese End"), ""};
    Option<bool> defaultChineseMode{this, "DefaultChineseMode", _("Default Chinese Mode"),
                                    false};)

class SenimeEngine;

class SenimeState : public InputContextProperty {
public:
    SenimeState(SenimeEngine *engine, InputContext *ic);
    ~SenimeState();

    void processKeyEvent(KeyEvent &event);
    void reset(bool reset_mode = true);
    void deactivate();
    void reloadEngine();
    bool chineseMode() const;
    void setChineseMode(bool chinese);

private:
    using StatePtr = std::unique_ptr<::SenimeState, decltype(&senime_state_free)>;

    void executeCommands(SenimeKeyEventResult *result, InputContext *ic);

    SenimeEngine *engine_;
    InputContext *ic_;
    StatePtr state_;
};

class SenimeEngine : public InputMethodEngineV2 {
public:
    explicit SenimeEngine(Instance *instance);

    void activate(const InputMethodEntry &entry,
                  InputContextEvent &event) override;
    void deactivate(const InputMethodEntry &entry,
                    InputContextEvent &event) override;
    void keyEvent(const InputMethodEntry &, KeyEvent &event) override;
    void reset(const InputMethodEntry &, InputContextEvent &event) override;
    void reloadConfig() override;
    const Configuration *getConfig() const override { return &configDef_; }
    void setConfig(const RawConfig &rawConfig) override;

    std::string subModeIconImpl(const InputMethodEntry &entry,
                                InputContext &ic) override;
    std::string subModeLabelImpl(const InputMethodEntry &entry,
                                 InputContext &ic) override;

    SenimeState *state(InputContext *ic);
    ::SenimeEngine *engine() const { return engine_.get(); }
    Instance *instance() const { return instance_; }
    const SenimeFcitxConfig &configDef() const { return configDef_; }
    const SenimeConfig &config() const { return config_; }
    void reloadEngine();

private:
    using EnginePtr = std::unique_ptr<::SenimeEngine, decltype(&senime_engine_free)>;

    static SenimeConfig convertConfig(const SenimeFcitxConfig &cfg);
    void updateAction(InputContext *ic);

    Instance *instance_;
    SenimeFcitxConfig configDef_;
    SenimeConfig config_{};
    FactoryFor<SenimeState> factory_;
    EnginePtr engine_{nullptr, senime_engine_free};

    // 全局中英模式状态
    std::atomic<bool> globalChineseMode_{false};

    // 托盘菜单动作
    SimpleAction toggleChineseAction_;
};

class SenimeEngineFactory : public AddonFactory {
public:
    AddonInstance *create(AddonManager *manager) override {
        return new SenimeEngine(manager->instance());
    }
};

} // namespace fcitx

#endif // SENIME_FCITX5_ENGINE_H_
