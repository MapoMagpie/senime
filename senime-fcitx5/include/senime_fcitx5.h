#ifndef SENIME_FCITX5_H_
#define SENIME_FCITX5_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SenimeEngine SenimeEngine;

typedef struct SenimeCandidate {
    char *text;
    char *code;
    uint32_t select_key;
    size_t order;
    bool unique;
} SenimeCandidate;

typedef struct SenimeAnalysis {
    char *text;
    SenimeCandidate *candidates;
    size_t candidate_count;
} SenimeAnalysis;

SenimeEngine *senime_engine_new(const char *table_path);
void senime_engine_free(SenimeEngine *engine);

SenimeAnalysis *senime_engine_analyze(const SenimeEngine *engine,
                                      const char *input);
void senime_analysis_free(SenimeAnalysis *analysis);

const char *senime_last_error(void);
void senime_string_free(char *value);

#ifdef __cplusplus
}
#endif

#endif // SENIME_FCITX5_H_
