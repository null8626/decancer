#ifndef __DECANCER_H__
#define __DECANCER_H__

#include <stdbool.h>
#include <stdint.h>

#if defined(_WIN32) && !defined(__DECANCER_CXX__) && !defined(__DECANCER_CXX_BUILDING__)
#define DECANCER_EXPORT __declspec(dllimport)
#else
#define DECANCER_EXPORT
#endif

#ifndef __DECANCER_CXX__
#define DECANCER_TRANSLATION_KIND_CHARACTER 0
#define DECANCER_TRANSLATION_KIND_STRING 1
#define DECANCER_TRANSLATION_KIND_NONE 2
#endif

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
#define DECANCER_OPTION_ASCII_ONLY (1 << 22)
#define DECANCER_OPTION_ALPHANUMERIC_ONLY (1 << 23)
#define DECANCER_OPTION_ALL 0xffffff
#define DECANCER_OPTION_PURE_HOMOGLYPH 0x1ffffc

#if defined(__DECANCER_CXX__) || defined(__DECANCER_CXX_BUILDING__)
#define DECANCER_EXPORT_NAME(name) name
#else
#define DECANCER_EXPORT_NAME(name) decancer_##name
#endif

#ifndef __DECANCER_CXX__
  typedef struct {
    const char* message;
    uint8_t message_length;
  } DECANCER_EXPORT_NAME(error_t);

  typedef struct {
    const uint8_t* string;
    size_t length;
  } DECANCER_EXPORT_NAME(keyword_t);

  typedef struct {
    const uint16_t* string;
    size_t length;
  } DECANCER_EXPORT_NAME(keyword_wide_t);

  typedef void* DECANCER_EXPORT_NAME(cured_raw_wide_t);
  typedef void* DECANCER_EXPORT_NAME(matcher_t);
  typedef void* DECANCER_EXPORT_NAME(matches_t);
#endif

  typedef struct {
    uint8_t kind;
    union {
      uint32_t character;
      struct {
        const uint8_t* contents;
        size_t length;
        void* __heap;
      } string;
    } contents;
  } DECANCER_EXPORT_NAME(translation_t);
  
  typedef void* DECANCER_EXPORT_NAME(cured_t);
  
  typedef struct {
    size_t start;
    size_t end;
  } DECANCER_EXPORT_NAME(match_t);
  
  typedef uint32_t DECANCER_EXPORT_NAME(options_t);

#ifdef __cplusplus
#if defined(__DECANCER_CXX__) || defined(__DECANCER_CXX_BUILDING__)
}; // namespace decancer
#endif

extern "C" {
#endif

#ifndef __DECANCER_CXX__
#ifdef __DECANCER_CXX_BUILDING__
#undef DECANCER_EXPORT_NAME
#define DECANCER_EXPORT_NAME(name) decancer::name
#endif

  DECANCER_EXPORT DECANCER_EXPORT_NAME(cured_t) decancer_cure(const uint8_t* input_str, const size_t input_size,
                                                              const DECANCER_EXPORT_NAME(options_t) options, DECANCER_EXPORT_NAME(error_t)* error);
  DECANCER_EXPORT DECANCER_EXPORT_NAME(cured_t) decancer_cure_wide(const uint16_t* input_str, const size_t input_size,
                                                                   const DECANCER_EXPORT_NAME(options_t) options, DECANCER_EXPORT_NAME(error_t)* error);
  DECANCER_EXPORT void decancer_cure_char(uint32_t input, const DECANCER_EXPORT_NAME(options_t) options,
                                          DECANCER_EXPORT_NAME(translation_t)* translation);

  DECANCER_EXPORT const uint8_t* decancer_cured_raw(DECANCER_EXPORT_NAME(cured_t) cured, size_t* output_length);
  DECANCER_EXPORT DECANCER_EXPORT_NAME(cured_raw_wide_t) decancer_cured_raw_wide(DECANCER_EXPORT_NAME(cured_t) cured, uint16_t** output_ptr,
                                       size_t* output_length);

  DECANCER_EXPORT const DECANCER_EXPORT_NAME(match_t)* decancer_matches_raw(DECANCER_EXPORT_NAME(matches_t) matches, size_t* output_length);

  DECANCER_EXPORT DECANCER_EXPORT_NAME(matcher_t) decancer_find(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str,
                                                                const size_t other_size);
  DECANCER_EXPORT DECANCER_EXPORT_NAME(matcher_t) decancer_find_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str,
                                                                     const size_t other_size);
  DECANCER_EXPORT DECANCER_EXPORT_NAME(matches_t) decancer_find_multiple(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_t)* other,
                                                                         const size_t other_length);
  DECANCER_EXPORT DECANCER_EXPORT_NAME(matches_t) decancer_find_multiple_wide(DECANCER_EXPORT_NAME(cured_t) cured,
                                                                              const DECANCER_EXPORT_NAME(keyword_wide_t)* other,
                                                                              const size_t other_length);
  DECANCER_EXPORT bool decancer_matcher_next(DECANCER_EXPORT_NAME(matcher_t) matcher, DECANCER_EXPORT_NAME(match_t)* match);

  DECANCER_EXPORT bool decancer_censor(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size,
                                       const uint32_t with_char);
  DECANCER_EXPORT bool decancer_censor_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size,
                                            const uint32_t with_char);
  DECANCER_EXPORT bool decancer_replace(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size,
                                        const uint8_t* with_str, const size_t with_size);
  DECANCER_EXPORT bool decancer_replace_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size,
                                             const uint16_t* with_str, const size_t with_size);
  DECANCER_EXPORT bool decancer_censor_multiple(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_t)* other,
                                                const size_t other_length, const uint32_t with_char);
  DECANCER_EXPORT bool decancer_censor_multiple_wide(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_wide_t)* other,
                                                     const size_t other_length, const uint32_t with_char);
  DECANCER_EXPORT bool decancer_replace_multiple(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_t)* other,
                                                 const size_t other_length, const uint8_t* with_str,
                                                 const size_t with_size);
  DECANCER_EXPORT bool decancer_replace_multiple_wide(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_wide_t)* other,
                                                      const size_t other_length, const uint16_t* with_str,
                                                      const size_t with_size);
  DECANCER_EXPORT bool decancer_contains(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size);
  DECANCER_EXPORT bool decancer_contains_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size);
  DECANCER_EXPORT bool decancer_starts_with(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size);
  DECANCER_EXPORT bool decancer_starts_with_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str,
                                                 const size_t other_size);
  DECANCER_EXPORT bool decancer_ends_with(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size);
  DECANCER_EXPORT bool decancer_ends_with_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size);
  DECANCER_EXPORT bool decancer_equals(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size);
  DECANCER_EXPORT bool decancer_equals_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size);

  DECANCER_EXPORT void decancer_cured_raw_wide_free(DECANCER_EXPORT_NAME(cured_raw_wide_t) wide);
  DECANCER_EXPORT void decancer_matcher_free(DECANCER_EXPORT_NAME(matcher_t) matcher);
  DECANCER_EXPORT void decancer_matches_free(DECANCER_EXPORT_NAME(matches_t) matches);
  DECANCER_EXPORT void decancer_translation_free(DECANCER_EXPORT_NAME(translation_t)* translation);
  DECANCER_EXPORT void decancer_cured_free(DECANCER_EXPORT_NAME(cured_t) cured);

#ifdef __DECANCER_CXX_BUILDING__
  DECANCER_EXPORT DECANCER_EXPORT_NAME(cured_t) __decancer_cured_clone(DECANCER_EXPORT_NAME(cured_t) ptr);
  DECANCER_EXPORT void __decancer_translation_clone(const DECANCER_EXPORT_NAME(translation_t)* translation_in, DECANCER_EXPORT_NAME(translation_t)* translation_out);
#endif
#endif

#undef DECANCER_EXPORT
#undef DECANCER_EXPORT_NAME

#ifdef __cplusplus
} // extern "C"

#if defined(__DECANCER_CXX__) || defined(__DECANCER_CXX_BUILDING__)
namespace decancer {
#endif
#endif
#endif