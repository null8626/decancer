#ifndef __DECANCER_H__
#define __DECANCER_H__

#include <stdbool.h>
#include <stdint.h>

#ifdef _WIN32
#define _DECANCER_EXPORT __declspec(dllimport)
#else
#define _DECANCER_EXPORT
#endif

typedef void * decancer_cured_t;

#ifdef __cplusplus
extern "C" {
#endif

_DECANCER_EXPORT decancer_cured_t decancer_cure(uint8_t * input_str, const size_t input_size);
_DECANCER_EXPORT bool decancer_equals(decancer_cured_t cured, uint8_t * other_str, const size_t other_size);
_DECANCER_EXPORT bool decancer_contains(decancer_cured_t cured, uint8_t * other_str, const size_t other_size);
_DECANCER_EXPORT bool decancer_starts_with(decancer_cured_t cured, uint8_t * other_str, const size_t other_size);
_DECANCER_EXPORT bool decancer_ends_with(decancer_cured_t cured, uint8_t * other_str, const size_t other_size);
_DECANCER_EXPORT size_t decancer_cured_string(decancer_cured_t cured, uint8_t ** output);
_DECANCER_EXPORT void decancer_free(decancer_cured_t cured);

#ifdef __cplusplus
}
#endif
#undef _DECANCER_EXPORT
#endif