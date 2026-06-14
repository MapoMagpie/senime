#ifndef SENIME_FCITX5_ENGINE_H_
#define SENIME_FCITX5_ENGINE_H_

#include "senime_fcitx5.h"

#include <memory>
#include <string>
#include <vector>
#include <fcitx-config/configuration.h>
#include <fcitx-config/iniparser.h>
#include <fcitx-config/option.h>
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
    SenimeConfig,
    Option<std::string> tablePath{this, "TablePath", _("Table Path"), ""};)

class SenimeEngine;

class SenimeState : public InputContextProperty {
public:
    SenimeState(SenimeEngine *engine, InputContext *ic);

    void keyEvent(KeyEvent &event);
    void reset();
    void commit();

private:
    void update();

    SenimeEngine *engine_;
    InputContext *ic_;
    std::string input_;
};

class SenimeEngine : public InputMethodEngine {
public:
    explicit SenimeEngine(Instance *instance);

    void keyEvent(const InputMethodEntry &, KeyEvent &event) override;
    void reset(const InputMethodEntry &, InputContextEvent &event) override;
    void deactivate(const InputMethodEntry &entry, InputContextEvent &event) override;
    void reloadConfig() override;
    const Configuration *getConfig() const override { return &config_; }
    void setConfig(const RawConfig &rawConfig) override;

    SenimeState *state(InputContext *ic);
    ::SenimeEngine *engine() const { return engine_.get(); }
    Instance *instance() const { return instance_; }
    const SenimeConfig &config() const { return config_; }
    void reloadEngine();

private:
    using EnginePtr = std::unique_ptr<::SenimeEngine, decltype(&senime_engine_free)>;

    Instance *instance_;
    SenimeConfig config_;
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
