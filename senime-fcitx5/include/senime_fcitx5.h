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

// ── Command types for key event processing ───────────────────────────────

typedef enum SenimeCommandType {
    SENIME_CMD_COMMIT_TEXT = 0,
    SENIME_CMD_SET_PREEDIT = 1,
    SENIME_CMD_SET_CANDIDATES = 2,
    SENIME_CMD_RESET_INPUT_PANEL = 3,
    SENIME_CMD_UPDATE_UI = 4,
    SENIME_CMD_UPDATE_STATUS_AREA = 5,
} SenimeCommandType;

typedef struct SenimeCandidateData {
    char *text;
    char *code;
    uint32_t select_key;
} SenimeCandidateData;

typedef struct SenimeCommand {
    SenimeCommandType type;
    char *text;
    SenimeCandidateData *candidates;
    size_t candidate_count;
} SenimeCommand;

typedef struct SenimeKeyEvent {
    uint32_t sym;
    uint32_t states;
    bool is_release;
} SenimeKeyEvent;

typedef struct SenimeConfig {
    uint32_t toggle_sym;       // 中英切换键 keysym
    uint32_t toggle_states;    // 中英切换键修饰符
    uint32_t trigger_sym;      // 临时中文触发键 keysym (0 = 禁用)
    uint32_t trigger_states;   // 临时中文触发键修饰符
    const char *table_path;
    bool default_chinese_mode; // 新建状态时默认使用中文模式
} SenimeConfig ;

typedef struct SenimeKeyEventResult {
    bool accepted;
    SenimeCommand *commands;
    size_t command_count;
} SenimeKeyEventResult;

// ── Engine lifecycle ─────────────────────────────────────────────────────

SenimeEngine *senime_engine_new(const SenimeConfig *config);
void senime_engine_free(SenimeEngine *engine);

// ── State lifecycle ──────────────────────────────────────────────────────

SenimeState *senime_state_new(const SenimeEngine *engine);
void senime_state_free(SenimeState *state);
void senime_state_reset(SenimeState *state);
void senime_state_set_chinese_mode(SenimeState *state, bool chinese);
bool senime_state_chinese_mode(const SenimeState *state);

// ── Key event processing ─────────────────────────────────────────────────

SenimeKeyEventResult *senime_engine_key_event(const SenimeEngine *engine,
                                                SenimeState *state,
                                                const SenimeKeyEvent *key);

// ── Result cleanup ───────────────────────────────────────────────────────

void senime_key_event_result_free(SenimeKeyEventResult *result);

// ── Utilities ────────────────────────────────────────────────────────────

const char *senime_last_error(void);
void senime_string_free(char *value);

#ifdef __cplusplus
}
#endif

#endif // SENIME_FCITX5_H_
