#ifndef __DECANCER_H__
#define __DECANCER_H__

#include <stdbool.h>
#include <stdint.h>

#ifdef _WIN32
#define _DECANCER_EXPORT __declspec(dllimport)
#else
#define _DECANCER_EXPORT
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef void * decancer_cured_t;
_DECANCER_EXPORT void decancer_free(decancer_cured_t cured);

_DECANCER_EXPORT decancer_cured_t decancer_cure(uint8_t * input_str, const size_t input_size);
_DECANCER_EXPORT bool decancer_equals(decancer_cured_t cured, uint8_t * other_str, const size_t other_size);
_DECANCER_EXPORT bool decancer_contains(decancer_cured_t cured, uint8_t * other_str, const size_t other_size);
_DECANCER_EXPORT bool decancer_starts_with(decancer_cured_t cured, uint8_t * other_str, const size_t other_size);
_DECANCER_EXPORT bool decancer_ends_with(decancer_cured_t cured, uint8_t * other_str, const size_t other_size);
_DECANCER_EXPORT const uint8_t * decancer_raw(decancer_cured_t cured, size_t * output_size);

typedef void * wdecancer_raw_cured_t;

_DECANCER_EXPORT decancer_cured_t wdecancer_cure(uint16_t * input_str, const size_t input_size);
_DECANCER_EXPORT bool wdecancer_equals(decancer_cured_t cured, uint16_t * other_str, const size_t other_size);
_DECANCER_EXPORT bool wdecancer_contains(decancer_cured_t cured, uint16_t * other_str, const size_t other_size);
_DECANCER_EXPORT bool wdecancer_starts_with(decancer_cured_t cured, uint16_t * other_str, const size_t other_size);
_DECANCER_EXPORT bool wdecancer_ends_with(decancer_cured_t cured, uint16_t * other_str, const size_t other_size);
_DECANCER_EXPORT wdecancer_raw_cured_t wdecancer_raw(decancer_cured_t cured, size_t * output_size);
_DECANCER_EXPORT const uint16_t * wdecancer_raw_ptr(decancer_raw_cured_t cured_raw);
_DECANCER_EXPORT void wdecancer_raw_free(decancer_raw_cured_t cured_raw);

#ifdef __cplusplus
}
#endif
#endif