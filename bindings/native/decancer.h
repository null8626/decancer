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

#define DECANCER_OPTION_DEFAULT 0
#define DECANCER_OPTION_RETAIN_CAPITALIZATION (1 << 0)
#define DECANCER_OPTION_DISABLE_BIDI (1 << 1)
#define DECANCER_OPTION_RETAIN_DIACRITICS (1 << 2)
#define DECANCER_OPTION_RETAIN_GREEK (1 << 3)
#define DECANCER_OPTION_RETAIN_CYRILLIC (1 << 4)
#define DECANCER_OPTION_RETAIN_HEBREW (1 << 5)
#define DECANCER_OPTION_RETAIN_ARABIC (1 << 6)
#define DECANCER_OPTION_RETAIN_DEVANAGARI (1 << 7)
#define DECANCER_OPTION_RETAIN_BENGALI (1 << 8)
#define DECANCER_OPTION_RETAIN_ARMENIAN (1 << 9)
#define DECANCER_OPTION_RETAIN_GUJARATI (1 << 10)
#define DECANCER_OPTION_RETAIN_TAMIL (1 << 11)
#define DECANCER_OPTION_RETAIN_THAI (1 << 12)
#define DECANCER_OPTION_RETAIN_LAO (1 << 13)
#define DECANCER_OPTION_RETAIN_BURMESE (1 << 14)
#define DECANCER_OPTION_RETAIN_KHMER (1 << 15)
#define DECANCER_OPTION_RETAIN_MONGOLIAN (1 << 16)
#define DECANCER_OPTION_RETAIN_CHINESE (1 << 17)
#define DECANCER_OPTION_RETAIN_JAPANESE (1 << 18)
#define DECANCER_OPTION_RETAIN_KOREAN (1 << 19)
#define DECANCER_OPTION_RETAIN_BRAILLE (1 << 20)
#define DECANCER_OPTION_RETAIN_EMOJIS (1 << 21)
#define DECANCER_OPTION_FORMATTER ((1 << 22) - 1)
#define DECANCER_OPTION_PURE_HOMOGLYPH (((1 << 22) - 1) ^ 0x200003)

    typedef struct
    {
        uint8_t kind;
        union {
            uint32_t character;
            struct
            {
                uint8_t *contents;
                size_t length;
                void *__heap;
            } string;
        } contents;
    } decancer_translation_t;

    typedef struct
    {
        size_t start;
        size_t end;
    } decancer_match_t;

    typedef uint32_t decancer_options_t;
    typedef uint8_t decancer_error_t;
    typedef void *decancer_matcher_t;
    typedef void *decancer_cured_t;

    _DECANCER_EXPORT bool decancer_contains(decancer_cured_t cured, uint8_t *other_str, const size_t other_size);
    _DECANCER_EXPORT decancer_cured_t decancer_cure(uint8_t *input_str, const size_t input_size,
                                                    const decancer_options_t options, decancer_error_t *error);
    _DECANCER_EXPORT const uint8_t *decancer_error(decancer_error_t error, uint8_t *string_size);
    _DECANCER_EXPORT void decancer_cure_char(uint32_t input, decancer_translation_t *translation);
    _DECANCER_EXPORT decancer_matcher_t decancer_find(decancer_cured_t cured, uint8_t *other_str, const size_t other_size);
    _DECANCER_EXPORT bool decancer_matcher_next(decancer_cured_t cured, decancer_match_t *match);
    _DECANCER_EXPORT bool decancer_ends_with(decancer_cured_t cured, uint8_t *other_str, const size_t other_size);
    _DECANCER_EXPORT bool decancer_equals(decancer_cured_t cured, uint8_t *other_str, const size_t other_size);
    _DECANCER_EXPORT void decancer_matcher_free(decancer_matcher_t matcher);
    _DECANCER_EXPORT void decancer_translation_free(decancer_translation_t* translation);
    _DECANCER_EXPORT void decancer_free(decancer_cured_t cured);
    _DECANCER_EXPORT const uint8_t *decancer_raw(decancer_cured_t cured, size_t *output_size);
    _DECANCER_EXPORT bool decancer_starts_with(decancer_cured_t cured, uint8_t *other_str, const size_t other_size);

#ifdef __cplusplus
}
#endif
#endif
