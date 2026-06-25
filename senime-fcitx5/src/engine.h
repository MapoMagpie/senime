#ifndef SENIME_FCITX5_ENGINE_H_
#define SENIME_FCITX5_ENGINE_H_

#include "senime_fcitx5.h"

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
#include <fcitx/text.h>
#include <fcitx-utils/i18n.h>

namespace fcitx {

FCITX_CONFIGURATION(
    SenimeConfigDef,
    Option<std::string> tablePath{this, "TablePath", _("Table Path"), ""};
    Option<KeyList, ListConstrain<KeyConstrain>> toggleMode{
        this, "ToggleMode", _("Toggle Chinese/English"),
        KeyList{Key("Shift+Shift_L")},
        KeyListConstrain(KeyConstrainFlag::AllowModifierOnly)};
    Option<KeyList, ListConstrain<KeyConstrain>> triggerTempChinese{
        this, "TriggerTempChinese", _("Trigger Temporary Chinese"),
        KeyList{Key("grave")},
        KeyListConstrain(KeyConstrainFlag::AllowModifierLess)};)

class SenimeEngine;

class SenimeState : public InputContextProperty {
public:
    SenimeState(SenimeEngine *engine, InputContext *ic);
    ~SenimeState();

    void processKeyEvent(KeyEvent &event);
    void reset();
    void deactivate();
    void reloadEngine();
    bool chineseMode() const;

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

    void keyEvent(const InputMethodEntry &, KeyEvent &event) override;
    void reset(const InputMethodEntry &, InputContextEvent &event) override;
    void deactivate(const InputMethodEntry &entry, InputContextEvent &event) override;
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
    const SenimeConfigDef &configDef() const { return configDef_; }
    const SenimeConfig &config() const { return config_; }
    void reloadEngine();
private:
    using EnginePtr = std::unique_ptr<::SenimeEngine, decltype(&senime_engine_free)>;

    static SenimeConfig convertConfig(const SenimeConfigDef &cfg);

    Instance *instance_;
    SenimeConfigDef configDef_;
    SenimeConfig config_{};
    FactoryFor<SenimeState> factory_;
    EnginePtr engine_{nullptr, senime_engine_free};
};

class SenimeEngineFactory : public AddonFactory {
public:
    AddonInstance *create(AddonManager *manager) override {
        return new SenimeEngine(manager->instance());
    }
};

} // namespace fcitx

#endif // SENIME_FCITX5_ENGINE_H_
