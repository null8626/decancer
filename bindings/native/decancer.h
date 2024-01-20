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
extern "C"
{
#endif

#define DECANCER_TRANSLATION_KIND_CHARACTER 0
#define DECANCER_TRANSLATION_KIND_STRING 1
#define DECANCER_TRANSLATION_KIND_NONE 2

#define DECANCER_ERROR_LEVEL_EXPLICIT_OVERFLOW 0
#define DECANCER_ERROR_LEVEL_IMPLICIT_OVERFLOW 1
#define DECANCER_ERROR_LEVEL_MODIFICATION_UNDERFLOW 2
#define DECANCER_ERROR_LEVEL_MODIFICATION_OVERFLOW 3

    typedef struct
    {
        uint8_t kind;
        union {
            uint32_t character;
            struct
            {
                uint8_t *contents;
                size_t length;
            } string;
        } contents;
    } decancer_translation_t;

    typedef uint8_t decancer_error_t;
    typedef void *decancer_cured_t;

    _DECANCER_EXPORT bool decancer_contains(decancer_cured_t cured, uint8_t *other_str, const size_t other_size);
    _DECANCER_EXPORT decancer_cured_t decancer_cure(uint8_t *input_str, const size_t input_size, decancer_error_t *error);
    _DECANCER_EXPORT const uint8_t *decancer_error(decancer_error_t error, uint8_t *string_size);
    _DECANCER_EXPORT void decancer_cure_char(uint32_t input, decancer_translation_t *translation);
    _DECANCER_EXPORT bool decancer_ends_with(decancer_cured_t cured, uint8_t *other_str, const size_t other_size);
    _DECANCER_EXPORT bool decancer_equals(decancer_cured_t cured, uint8_t *other_str, const size_t other_size);
    _DECANCER_EXPORT void decancer_free(decancer_cured_t cured);
    _DECANCER_EXPORT const uint8_t *decancer_raw(decancer_cured_t cured, size_t *output_size);
    _DECANCER_EXPORT bool decancer_starts_with(decancer_cured_t cured, uint8_t *other_str, const size_t other_size);

#ifdef __cplusplus
}
#endif
#endif
