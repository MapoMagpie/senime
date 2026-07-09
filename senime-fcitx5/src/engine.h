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
    OptionWithAnnotation<std::string, ToolTipAnnotation> tablePath{
        this, "TablePath", _("Table Path"), "",
        NoConstrain<std::string>{}, DefaultMarshaller<std::string>{},
        ToolTipAnnotation(_("Path to the dictionary or config file. When empty, "
                            "defaults to XDG_DATA_CONFIG/senime/config.toml. "
                            "The directory must be writable for binary cache generation."))};
    KeyListOptionWithAnnotation<ToolTipAnnotation> toggleMode{
        this, "ToggleMode", _("Toggle Chinese/English"),
        KeyList{Key("Shift+Shift_L")},
        KeyListConstrain(KeyConstrainFlag::AllowModifierOnly),
        DefaultMarshaller<KeyList>{},
        ToolTipAnnotation(_("Key combination to toggle between Chinese and English mode."))};
    OptionWithAnnotation<std::string, ToolTipAnnotation> triggerTempChineseStart{
        this, "TriggerTempChineseStart", _("Trigger Temporary Chinese Start"), "",
        NoConstrain<std::string>{}, DefaultMarshaller<std::string>{},
        ToolTipAnnotation(_("In English mode, typing this character temporarily "
                            "enables Chinese mode. Leave empty to disable."))};
    OptionWithAnnotation<std::string, ToolTipAnnotation> triggerTempChineseEnd{
        this, "TriggerTempChineseEnd", _("Trigger Temporary Chinese End"), "",
        NoConstrain<std::string>{}, DefaultMarshaller<std::string>{},
        ToolTipAnnotation(_("Typing this character ends temporary Chinese mode "
                            "and commits text. Leave empty to use the same "
                            "character as Start."))};
    OptionWithAnnotation<bool, ToolTipAnnotation> defaultChineseMode{
        this, "DefaultChineseMode", _("Default Chinese Mode"), false,
        NoConstrain<bool>{}, DefaultMarshaller<bool>{},
        ToolTipAnnotation(_("Whether to enable Chinese mode by default."))};
    OptionWithAnnotation<bool, ToolTipAnnotation> sentenceFlow{
        this, "SentenceFlow", _("Sentence Flow"), false,
        NoConstrain<bool>{}, DefaultMarshaller<bool>{},
        ToolTipAnnotation(_("In continuous typing, text is not committed until "
                            "punctuation. This is an experimental feature."))};
    OptionWithAnnotation<bool, ToolTipAnnotation> resetStateOnFocusIn{
        this, "ResetStateOnFocusIn", _("Reset State on Focus In"), false,
        NoConstrain<bool>{}, DefaultMarshaller<bool>{},
        ToolTipAnnotation(_("When switching input fields, reset Chinese/English "
                            "state according to Default Chinese Mode."))};
    OptionWithAnnotation<bool, ToolTipAnnotation> enableTextPreedit{
        this, "EnableTextPreedit", _("Enable Text Preedit"), true,
        NoConstrain<bool>{}, DefaultMarshaller<bool>{},
        ToolTipAnnotation(_("Show the Chinese character (IME result) in the "
                            "preedit area."))};
    OptionWithAnnotation<bool, ToolTipAnnotation> enableInputPreedit{
        this, "EnableInputPreedit", _("Enable Input Preedit"), false,
        NoConstrain<bool>{}, DefaultMarshaller<bool>{},
        ToolTipAnnotation(_("Show the actual typing codes in the preedit area. "
                            "If both preedit options are enabled, the result and "
                            "codes are concatenated."))};)
// Table Path
//   码表或配置文件地址，为空时，将默认加载XDG_DATA_CONFIG/senime/config.toml。
//   需要保证码表文件所在的目录有写入权限，以方便生成二进制缓存
// Toggle Chinese/English
//   用于切换中英文模式的按键
// Trigger Temporary Chinese Start
//   在英文模式下，输入此字符后，可临时启用中文模式，输出汉字
//   可为空，为空表示不启用临时中文模式
// Trigger Temporary Chinese End
//   输入此字符可结束临时中文模式，并上屏
//   可为空，为空时将被设置为与Trigger Temporary Chinese Start一样的字符
// Default Chinese Mode
//   是否默认启用中文模式
// SentenceFlow
//   语句流，在连续打一句话时，不会上屏，输入标点符号时才上屏
//   此为实验性特性
// Reset State On Focus In
//   切换应用(输入框)后，是否重置中英文状态，重置时以Default Chinese Mode为准
// Enable Text Preedit
//   预编辑(未上屏)将展示汉字(输入法结果)
// Enable Input Preedit
//   预编辑(未上屏)将展示用户的实际输入
//   同时启用Enable Text Preedit和Enable Input Preedit，则是将输入法结果与用户实际输入拼接起来展示


class SenimeEngine;

class SenimeState : public InputContextProperty {
public:
    SenimeState(SenimeEngine *engine, InputContext *ic);
    ~SenimeState();

    void processKeyEvent(KeyEvent &event);
    void reset_input();
    void reset_chinese_mode();
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
