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
// clang-format off
}; // namespace decancer
// clang-format on
#endif

extern "C" {
#endif

#ifndef __DECANCER_CXX__
#ifdef __DECANCER_CXX_BUILDING__
#undef DECANCER_EXPORT_NAME
#define DECANCER_EXPORT_NAME(name) decancer::name
#endif

  /**
   * @brief Cures a UTF-8 string.
   * For its UTF-16 counterpart, see decancer_cure_wide.
   *
   * @param input_str Input string, in UTF-8 bytes.
   * @param input_size UTF-8 size of the input string, in bytes.
   * @param options Options to customize decancer's curing behavior. To use decancer's default behavior, pass in DECANCER_OPTION_DEFAULT.
   * @param error A pointer to a decancer_error_t struct. This pointer can be NULL if you want to ignore errors.
   * @see decancer_cure_wide
   * @see decancer_cure_char
   * @see decancer_cured_free
   * @return decancer_cured_t The cured string object or NULL on failure -- see the modified error struct for more details.
   * @note You are responsible in freeing the returned object later by calling decancer_cured_free.
   */
  DECANCER_EXPORT DECANCER_EXPORT_NAME(cured_t) decancer_cure(const uint8_t* input_str, const size_t input_size, const DECANCER_EXPORT_NAME(options_t) options, DECANCER_EXPORT_NAME(error_t)* error);
  
  /**
   * @brief Cures a UTF-16 string.
   * For its UTF-8 counterpart, see decancer_cure.
   *
   * @param input_str Input string, in UTF-16 bytes.
   * @param input_size UTF-16 size of the input string, in bytes.
   * @param options Options to customize decancer's curing behavior. To use decancer's default behavior, pass in DECANCER_OPTION_DEFAULT.
   * @param error A pointer to a decancer_error_t struct. This pointer can be NULL if you want to ignore errors.
   * @see decancer_cure
   * @see decancer_cure_char
   * @see decancer_cured_free
   * @return decancer_cured_t The cured string object or NULL on failure -- see the modified error struct for more details.
   * @note You are responsible in freeing the returned object later by calling decancer_cured_free.
   */
  DECANCER_EXPORT DECANCER_EXPORT_NAME(cured_t) decancer_cure_wide(const uint16_t* input_str, const size_t input_size, const DECANCER_EXPORT_NAME(options_t) options, DECANCER_EXPORT_NAME(error_t)* error);
  
  /**
   * @brief Cures a single unicode codepoint.
   *
   * @param input The unicode codepoint.
   * @param options Options to customize decancer's curing behavior. To use decancer's default behavior, pass in DECANCER_OPTION_DEFAULT.
   * @param translation A pointer to the output translation struct.
   * @see decancer_cure
   * @see decancer_cure_wide
   * @see decancer_translation_free
   * @note You are responsible in freeing the translation struct later by passing it as a pointer to decancer_translation_free.
   */
  DECANCER_EXPORT void decancer_cure_char(uint32_t input, const DECANCER_EXPORT_NAME(options_t) options, DECANCER_EXPORT_NAME(translation_t)* translation);

  /**
   * @brief Retrieves the raw UTF-8 bytes from a cured string object.
   * For its UTF-16 counterpart, see decancer_cured_raw_wide.
   *
   * @param cured The cured string object.
   * @param output_size A pointer to the output's UTF-8 size, in bytes.
   * @see decancer_cured_raw_wide
   * @see decancer_cured_raw_wide_free
   * @return const uint8_t* An immutable UTF-8 pointer representing raw contents of the cured string object.
   * @note The returned pointer remains valid until cured gets passed onto decancer_cured_free.
   */
  DECANCER_EXPORT const uint8_t* decancer_cured_raw(DECANCER_EXPORT_NAME(cured_t) cured, size_t* output_size);
  
  /**
   * @brief Retrieves the raw UTF-16 bytes from a cured string object.
   * For its UTF-8 counterpart, see decancer_cured_raw.
   *
   * @param cured The cured string object.
   * @param output_ptr A pointer to the output's UTF-16 pointer.
   * @param output_size A pointer to the output's UTF-16 size, in bytes.
   * @see decancer_cured_raw
   * @see decancer_cured_raw_wide_free
   * @return decancer_cured_raw_wide_t A rust object. This value has no use other than retaining the lifetime of the returned UTF-16 pointer.
   * @note You are responsible in freeing the returned object later by calling decancer_cured_raw_wide_free.
   * The lifetime of the UTF-16 pointer remains valid until the returned object gets passed onto decancer_cured_raw_wide_free.
   */
  DECANCER_EXPORT DECANCER_EXPORT_NAME(cured_raw_wide_t) decancer_cured_raw_wide(DECANCER_EXPORT_NAME(cured_t) cured, uint16_t** output_ptr, size_t* output_length);

  /**
   * @brief Returns the raw list of every similar-looking match from a decancer_matches_t object.
   *
   * @param matches The matches object.
   * @param output_size A pointer to the output's array size.
   * @return const decancer_match_t* The raw pointer containing every similar-looking match.
   * @note The returned pointer remains valid until the matches object gets passed onto decancer_matches_free.
   */
  DECANCER_EXPORT const DECANCER_EXPORT_NAME(match_t)* decancer_matches_raw(DECANCER_EXPORT_NAME(matches_t) matches, size_t* output_size);

  /**
   * @brief Finds every similar-looking match of a UTF-8 string in the cured string.
   * For its UTF-16 counterpart, see decancer_find_wide.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_find_wide
   * @see decancer_find_multiple
   * @see decancer_find_multiple_wide
   * @see decancer_matcher_free
   * @return decancer_matcher_t A matcher iterator object.
   * @note You are responsible in freeing the returned object later by calling decancer_matcher_free.
   */
  DECANCER_EXPORT DECANCER_EXPORT_NAME(matcher_t) decancer_find(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size);
  
  /**
   * @brief Finds every similar-looking match of a UTF-16 string in the cured string.
   * For its UTF-8 counterpart, see decancer_find.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 string to match with.
   * @param other_size UTF-16 size of the other string, in bytes.
   * @see decancer_find
   * @see decancer_find_multiple
   * @see decancer_find_multiple_wide
   * @see decancer_matcher_free
   * @return decancer_matcher_t A matcher iterator object.
   * @note You are responsible in freeing the returned object later by calling decancer_matcher_free.
   */
  DECANCER_EXPORT DECANCER_EXPORT_NAME(matcher_t) decancer_find_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size);
  
  /**
   * @brief Finds every similar-looking match from a list of UTF-8 keywords in the cured string.
   * Unlike decancer_find, this function also takes note of overlapping matches and merges them together.
   * For its UTF-16 counterpart, see decancer_find_multiple_wide.
   *
   * @param cured The cured string object.
   * @param other A list of UTF-8 keywords to match with.
   * @param other_length The length of the keywords array (NOT the size of the entire array in bytes).
   * @see decancer_find
   * @see decancer_find_wide
   * @see decancer_find_multiple_wide
   * @see decancer_matches_free
   * @return decancer_matches_t A matches object.
   * @note You are responsible in freeing the returned object later by calling decancer_matches_free.
   */
  DECANCER_EXPORT DECANCER_EXPORT_NAME(matches_t) decancer_find_multiple(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_t)* other, const size_t other_length);
  
  /**
   * @brief Finds every similar-looking match from a list of UTF-16 keywords in the cured string.
   * Unlike decancer_find_wide, this function also takes note of overlapping matches and merges them together.
   * For its UTF-8 counterpart, see decancer_find_multiple.
   *
   * @param cured The cured string object.
   * @param other A list of UTF-16 keywords to match with.
   * @param other_length The length of the keywords array (NOT the size of the entire array in bytes).
   * @see decancer_find
   * @see decancer_find_wide
   * @see decancer_matches_free
   * @return decancer_matches_t A matches object.
   * @note You are responsible in freeing the returned object later by calling decancer_matches_free.
   */
  DECANCER_EXPORT DECANCER_EXPORT_NAME(matches_t) decancer_find_multiple_wide(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_wide_t)* other, const size_t other_length);
  
  /**
   * @brief Iterates to the next element of the matcher iterator.
   *
   * @param matcher The matcher iterator object.
   * @param match A pointer to a decancer_match_t struct.
   * @return bool true if a new value is present, or false if the iteration is complete.
   */
  DECANCER_EXPORT bool decancer_matcher_next(DECANCER_EXPORT_NAME(matcher_t) matcher, DECANCER_EXPORT_NAME(match_t)* match);

  /**
   * @brief Censors every similar-looking match of the specified UTF-8 string.
   * For its UTF-16 counterpart, see decancer_censor_wide.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @param with_char The censor unicode codepoint. Ideally '*' (0x2a) or '-' (0x2d).
   * @see decancer_censor_wide
   * @see decancer_censor_multiple
   * @see decancer_censor_multiple_wide
   * @return bool true on success, or false on failure due to invalid encoding.
   */
  DECANCER_EXPORT bool decancer_censor(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size, const uint32_t with_char);
  
  /**
   * @brief Censors every similar-looking match of the specified UTF-16 string.
   * For its UTF-8 counterpart, see decancer_censor.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 string to match with.
   * @param other_size UTF-16 size of the other string, in bytes.
   * @param with_char The censor unicode codepoint. Ideally '*' (0x2a) or '-' (0x2d).
   * @see decancer_censor
   * @see decancer_censor_multiple
   * @see decancer_censor_multiple_wide
   * @return bool true on success, or false on failure due to invalid encoding.
   */
  DECANCER_EXPORT bool decancer_censor_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size, const uint32_t with_char);
  
  /**
   * @brief Replaces every similar-looking match of the specified UTF-8 string with another UTF-8 string.
   * For its UTF-16 counterpart, see decancer_replace_wide.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @param with_str The UTF-8 string to replace with.
   * @param with_size UTF-8 size of the replacement string, in bytes.
   * @see decancer_replace_wide
   * @see decancer_replace_multiple
   * @see decancer_replace_multiple_wide
   * @return bool true on success, or false on failure due to invalid encoding.
   */
  DECANCER_EXPORT bool decancer_replace(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size, const uint8_t* with_str, const size_t with_size);
  
  /**
   * @brief Replaces every similar-looking match of the specified UTF-16 string with another UTF-16 string.
   * For its UTF-8 counterpart, see decancer_replace.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 string to match with.
   * @param other_size UTF-16 size of the other string, in bytes.
   * @param with_str The UTF-16 string to replace with.
   * @param with_size UTF-16 size of the replacement string, in bytes.
   * @see decancer_replace
   * @see decancer_replace_multiple
   * @see decancer_replace_multiple_wide
   * @return bool true on success, or false on failure due to invalid encoding.
   */
  DECANCER_EXPORT bool decancer_replace_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size, const uint16_t* with_str, const size_t with_size);
  
  /**
   * @brief Censors every similar-looking match of the specified list of UTF-8 keywords.
   * Unlike decancer_censor, this function also takes note of overlapping matches.
   * For its UTF-16 counterpart, see decancer_censor_multiple_wide.
   *
   * @param cured The cured string object.
   * @param other A list of UTF-8 keywords to match with.
   * @param other_length The length of the keywords array (NOT the size of the entire array in bytes).
   * @param with_char The censor unicode codepoint. Ideally '*' (0x2a) or '-' (0x2d).
   * @see decancer_censor
   * @see decancer_censor_wide
   * @see decancer_censor_multiple_wide
   * @return bool true on success, or false on failure due to invalid encoding.
   */
  DECANCER_EXPORT bool decancer_censor_multiple(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_t)* other, const size_t other_length, const uint32_t with_char);
  
  /**
   * @brief Censors every similar-looking match of the specified list of UTF-16 keywords.
   * Unlike decancer_censor_wide, this function also takes note of overlapping matches.
   * For its UTF-8 counterpart, see decancer_censor_multiple.
   *
   * @param cured The cured string object.
   * @param other A list of UTF-16 keywords to match with.
   * @param other_length The length of the keywords array (NOT the size of the entire array in bytes).
   * @param with_char The censor unicode codepoint. Ideally '*' (0x2a) or '-' (0x2d).
   * @see decancer_censor
   * @see decancer_censor_wide
   * @see decancer_censor_multiple
   * @return bool true on success, or false on failure due to invalid encoding.
   */
  DECANCER_EXPORT bool decancer_censor_multiple_wide(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_wide_t)* other, const size_t other_length, const uint32_t with_char);
  
  /**
   * @brief Replaces every similar-looking match of the specified list of UTF-8 keywords with another UTF-8 string.
   * Unlike decancer_replace, this function also takes note of overlapping matches.
   * For its UTF-16 counterpart, see decancer_replace_multiple_wide.
   *
   * @param cured The cured string object.
   * @param other A list of UTF-8 keywords to match with.
   * @param other_length The length of the keywords array (NOT the size of the entire array in bytes).
   * @param with_str The UTF-8 string to replace with.
   * @param with_size UTF-8 size of the replacement string, in bytes.
   * @see decancer_replace
   * @see decancer_replace_wide
   * @see decancer_replace_multiple_wide
   * @return bool true on success, or false on failure due to invalid encoding.
   */
  DECANCER_EXPORT bool decancer_replace_multiple(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_t)* other, const size_t other_length, const uint8_t* with_str, const size_t with_size);
  
  /**
   * @brief Replaces every similar-looking match of the specified list of UTF-16 keywords with another UTF-16 string.
   * Unlike decancer_replace, this function also takes note of overlapping matches.
   * For its UTF-8 counterpart, see decancer_replace_multiple.
   *
   * @param cured The cured string object.
   * @param other A list of UTF-16 keywords to match with.
   * @param other_length The length of the keywords array (NOT the size of the entire array in bytes).
   * @param with_str The UTF-16 string to replace with.
   * @param with_size UTF-16 size of the replacement string, in bytes.
   * @see decancer_replace
   * @see decancer_replace_wide
   * @see decancer_replace_multiple
   * @return bool true on success, or false on failure due to invalid encoding.
   */
  DECANCER_EXPORT bool decancer_replace_multiple_wide(DECANCER_EXPORT_NAME(cured_t) cured, const DECANCER_EXPORT_NAME(keyword_wide_t)* other, const size_t other_length, const uint16_t* with_str, const size_t with_size);
  
  /**
   * @brief Checks if the cured string similarly contains the specified UTF-8 string.
   * For its UTF-16 counterpart, see decancer_contains_wide.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_contains_wide
   * @return bool true if the cured string similarly contains the specified string, false otherwise.
   */
  DECANCER_EXPORT bool decancer_contains(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size);
  
  /**
   * @brief Checks if the cured string similarly contains the specified UTF-16 string.
   * For its UTF-8 counterpart, see decancer_contains.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 string to match with.
   * @param other_size UTF-16 size of the other string, in bytes.
   * @see decancer_contains
   * @return bool true if the cured string similarly contains the specified string, false otherwise.
   */
  DECANCER_EXPORT bool decancer_contains_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size);
  
  /**
   * @brief Checks if the cured string similarly starts with the specified UTF-8 string.
   * For its UTF-16 counterpart, see decancer_starts_with_wide.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_starts_with_wide
   * @return bool true if the cured string similarly starts with the specified string, false otherwise.
   */
  DECANCER_EXPORT bool decancer_starts_with(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size);
  
  /**
   * @brief Checks if the cured string similarly starts with the specified UTF-16 string.
   * For its UTF-8 counterpart, see decancer_starts_with.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 string to match with.
   * @param other_size UTF-16 size of the other string, in bytes.
   * @see decancer_starts_with
   * @return bool true if the cured string similarly starts with the specified string, false otherwise.
   */
  DECANCER_EXPORT bool decancer_starts_with_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size);
  
  /**
   * @brief Checks if the cured string similarly ends with the specified UTF-8 string.
   * For its UTF-16 counterpart, see decancer_ends_with_wide.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_ends_with_wide
   * @return bool true if the cured string similarly ends with the specified string, false otherwise.
   */
  DECANCER_EXPORT bool decancer_ends_with(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size);
  
  /**
   * @brief Checks if the cured string similarly ends with the specified UTF-16 string.
   * For its UTF-8 counterpart, see decancer_ends_with.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 string to match with.
   * @param other_size UTF-16 size of the other string, in bytes.
   * @see decancer_ends_with
   * @return bool true if the cured string similarly ends with the specified string, false otherwise.
   */
  DECANCER_EXPORT bool decancer_ends_with_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size);
  
  /**
   * @brief Checks if the cured string is similar with the specified UTF-8 string.
   * For its UTF-16 counterpart, see decancer_equals_wide.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_equals_wide
   * @return bool true if the cured string is similar with the specified string, false otherwise.
   */
  DECANCER_EXPORT bool decancer_equals(DECANCER_EXPORT_NAME(cured_t) cured, const uint8_t* other_str, const size_t other_size);
  
  /**
   * @brief Checks if the cured string is similar with the specified UTF-16 string.
   * For its UTF-8 counterpart, see decancer_equals.
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 string to match with.
   * @param other_size UTF-16 size of the other string, in bytes.
   * @see decancer_equals
   * @return bool true if the cured string is similar with the specified string, false otherwise.
   */
  DECANCER_EXPORT bool decancer_equals_wide(DECANCER_EXPORT_NAME(cured_t) cured, const uint16_t* other_str, const size_t other_size);

  /**
   * @brief Frees the rust object created by decancer_cured_raw_wide.
   */
  DECANCER_EXPORT void decancer_cured_raw_wide_free(DECANCER_EXPORT_NAME(cured_raw_wide_t) wide);
  
  /**
   * @brief Frees the matcher iterator object created by decancer_find and decancer_find_wide.
   */
  DECANCER_EXPORT void decancer_matcher_free(DECANCER_EXPORT_NAME(matcher_t) matcher);
  
  /**
   * @brief Frees the matches object created by decancer_find_multiple and decancer_find_multiple_wide.
   */
  DECANCER_EXPORT void decancer_matches_free(DECANCER_EXPORT_NAME(matches_t) matches);
  
  /**
   * @brief Frees the translation struct used in decancer_cure_char.
   */
  DECANCER_EXPORT void decancer_translation_free(DECANCER_EXPORT_NAME(translation_t)* translation);
  
  /**
   * @brief Frees the cured string object created by decancer_cure and decancer_cure_wide.
   */
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