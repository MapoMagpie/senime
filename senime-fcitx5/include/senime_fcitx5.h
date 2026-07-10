#ifndef SENIME_FCITX5_H_
#define SENIME_FCITX5_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SenimeEngine SenimeEngine;
typedef struct SenimeState SenimeState;

// ── Display state for key event processing ───────────────────────────────
//
// 每轮 key_event 后，Rust 侧填充此结构体，描述输入面板的期望状态。
// C++ 侧按固定顺序 apply：
//   1. commit_str 非空 → commitString
//   2. 重置 InputPanel
//   3. preedit_str 非空 → setPreedit
//   4. candidates 非空 → setCandidateList
//   5. aux_up_str / aux_down_str 非空 → setAuxUp / setAuxDown
//   6. updatePreedit + updateUserInterface
//   7. update_status_area 为真 → updateUserInterface(StatusArea)
//
// 所有字符串指针指向 SenimeState 内部内存，有效期至下一次 key_event。
// C++ 侧只读，不需要释放。

typedef struct SenimeCandidateData {
    char *text;
    char *code;
    uint32_t select_key;
} SenimeCandidateData;

typedef struct SenimeInnerState {
    char *commit_str;
    char *preedit_str;
    char *aux_up_str;
    char *aux_down_str;
    SenimeCandidateData *candidates;
    size_t candidate_count;
    bool update_status_area;
} SenimeInnerState;

typedef struct SenimeKeyEvent {
    uint32_t sym;
    uint32_t states;
    bool is_release;
} SenimeKeyEvent;

typedef struct SenimeConfig {
    uint32_t toggle_sym;             // 中英切换键 keysym
    uint32_t toggle_states;          // 中英切换键修饰符
    uint32_t trigger_start_char;     // 临时中文开始字符 keysym (0 = 禁用)
    uint32_t trigger_end_char;       // 临时中文结束字符 keysym (0 = 使用start)
    const char *table_path;
    bool default_chinese_mode;       // 新建状态时默认使用中文模式
    bool sentence_flow;              // 语句流模式：输入持续在preedit，遇标点或双空格才提交
    bool enable_text_preedit;        // 在预编辑中显示码表查询出的汉字
    bool enable_input_preedit;       // 在预编辑中显示用户原始输入编码
} SenimeConfig ;

typedef struct SenimeKeyEventResult {
    bool accepted;
    const SenimeInnerState *inner_state;
} SenimeKeyEventResult;

// ── Engine lifecycle ─────────────────────────────────────────────────────

SenimeEngine *senime_engine_new(const SenimeConfig *config);
void senime_engine_free(SenimeEngine *engine);

// ── State lifecycle ──────────────────────────────────────────────────────

SenimeState *senime_state_new(const SenimeEngine *engine);
void senime_state_free(SenimeState *state);
void senime_state_reset(SenimeState *state);
void senime_state_reset_input(SenimeState *state);
void senime_state_set_chinese_mode(SenimeState *state, bool chinese);
bool senime_state_chinese_mode(const SenimeState *state);

// ── Key event processing ─────────────────────────────────────────────────

/// 处理键盘事件，返回结果结构（含 InnerState 指针，指向 state 内部内存）。
SenimeKeyEventResult *senime_engine_key_event(const SenimeEngine *engine,
                                                SenimeState *state,
                                                const SenimeKeyEvent *key);

// ── Result cleanup ───────────────────────────────────────────────────────

/// 释放 senime_engine_key_event 返回的结果结构体。
/// 注意：InnerState 及其字符串归 SenimeState 所有，不在此释放。
void senime_key_event_result_free(SenimeKeyEventResult *result);

/// 手动清空 InnerState（通常在 reset 时调用）。
void senime_inner_state_clear(SenimeState *state);

// ── Utilities ────────────────────────────────────────────────────────────

void senime_string_free(char *value);

// ── Logging ─────────────────────────────────

/// 日志回调函数类型: (level, message)
/// level: 0=INFO, 1=WARN, 2=ERROR
typedef void (*SenimeLogCallback)(int level, const char *message);

/// 设置日志回调，应在 senime_engine_new 之前调用
void senime_set_log_callback(SenimeLogCallback callback);

#ifdef __cplusplus
}
#endif

#endif // SENIME_FCITX5_H_
