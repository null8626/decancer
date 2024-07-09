/**
 * @file decancer.h
 * @brief A library that removes common unicode confusables/homoglyphs from strings.
 * @author Copyright (c) 2021-2024 null8626
 * @version 3.2.3
 */

#ifndef __DECANCER_H__
#define __DECANCER_H__

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef _WIN32
#define DECANCER_EXPORT __declspec(dllimport)
#else
#define DECANCER_EXPORT
#endif

/**
 * @brief The current version used by this library.
 *
 * @see DECANCER_VERSION_MAJOR
 * @see DECANCER_VERSION_MINOR
 * @see DECANCER_VERSION_PATCH
 * @since 3.2.2
 */
#define DECANCER_VERSION 0x030203

/**
 * @brief The current major version used by this library.
 *
 * @see DECANCER_VERSION
 * @see DECANCER_VERSION_MINOR
 * @see DECANCER_VERSION_PATCH
 * @since 3.2.2
 */
#define DECANCER_VERSION_MAJOR ((DECANCER_VERSION & 0xff0000) >> 16)

/**
 * @brief The current minor version used by this library.
 *
 * @see DECANCER_VERSION
 * @see DECANCER_VERSION_MAJOR
 * @see DECANCER_VERSION_PATCH
 * @since 3.2.2
 */
#define DECANCER_VERSION_MINOR ((DECANCER_VERSION & 0xff00) >> 8)

/**
 * @brief The current patch version used by this library.
 *
 * @see DECANCER_VERSION
 * @see DECANCER_VERSION_MAJOR
 * @see DECANCER_VERSION_MINOR
 * @since 3.2.2
 */
#define DECANCER_VERSION_PATCH (DECANCER_VERSION & 0xff)

/**
 * @brief The translation is a single unicode character.
 * @since 1.5.4
 */
#define DECANCER_TRANSLATION_KIND_CHARACTER 0

/**
 * @brief The translation is a string.
 * @since 1.5.4
 */
#define DECANCER_TRANSLATION_KIND_STRING 1

/**
 * @brief The translation is an empty string.
 *
 * You can get this when the input character is a control character, surrogate, combining character (e.g diacritics), private use character, byte order character, or any invalid unicode value.
 * @since 1.5.4
 */
#define DECANCER_TRANSLATION_KIND_NONE 2

/**
 * @brief Uses decancer's default options -- AKA to be AS AGGRESSIVE AS POSSIBLE.
 *
 * This makes decancer cures as much characters as possible and turns all the output characters to lowercase.
 * @since 3.0.0
 */
#define DECANCER_OPTION_DEFAULT 0

/**
 * @brief Prevents decancer from changing all characters to lowercase. Therefore, if the input character is in uppercase, the output character will be in uppercase as well.
 *
 * @note Many confusables are neither an uppercase or a lowercase character. Therefore, the decancer defaults to displaying the translation in LOWERCASE.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_CAPITALIZATION (1 << 0)

/**
 * @brief Prevents decancer from applying the Unicode Bidirectional Algorithm.
 *
 * Use this ONLY when you don't expect any right-to-left characters.
 * Enabling this option has no effect if it's called on decancer_cure_char.
 *
 * @see DECANCER_OPTION_RETAIN_ARABIC
 * @see DECANCER_OPTION_RETAIN_HEBREW
 * @warning This speeds up the function call, but CAN BREAK right-to-left characters. It's highly recommended to also use DECANCER_OPTION_RETAIN_ARABIC and DECANCER_OPTION_RETAIN_HEBREW.
 * @since 3.0.0
 */
#define DECANCER_OPTION_DISABLE_BIDI (1 << 1)

/**
 * @brief Prevents decancer from curing characters WITH diacritics or accents.
 *
 * @note Decancer can still cure standalone diacritic characters, which is used in Zalgo texts.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_DIACRITICS (1 << 2)

/**
 * @brief Prevents decancer from curing all greek characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_GREEK (1 << 3)

/**
 * @brief Prevents decancer from curing all cyrillic characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_CYRILLIC (1 << 4)

/**
 * @brief Prevents decancer from curing all hebrew characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_HEBREW (1 << 5)

/**
 * @brief Prevents decancer from curing all arabic characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_ARABIC (1 << 6)

/**
 * @brief Prevents decancer from curing all devanagari characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_DEVANAGARI (1 << 7)

/**
 * @brief Prevents decancer from curing all bengali characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_BENGALI (1 << 8)

/**
 * @brief Prevents decancer from curing all armenian characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_ARMENIAN (1 << 9)

/**
 * @brief Prevents decancer from curing all gujarati characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_GUJARATI (1 << 10)

/**
 * @brief Prevents decancer from curing all tamil characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_TAMIL (1 << 11)

/**
 * @brief Prevents decancer from curing all thai characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_THAI (1 << 12)

/**
 * @brief Prevents decancer from curing all lao characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_LAO (1 << 13)

/**
 * @brief Prevents decancer from curing all burmese characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_BURMESE (1 << 14)

/**
 * @brief Prevents decancer from curing all khmer characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_KHMER (1 << 15)

/**
 * @brief Prevents decancer from curing all mongolian characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_MONGOLIAN (1 << 16)

/**
 * @brief Prevents decancer from curing all chinese characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_CHINESE (1 << 17)

/**
 * @brief Prevents decancer from curing all katakana and hiragana characters.
 *
 * @see DECANCER_OPTION_RETAIN_CHINESE
 * @note To also provent decancer from curing kanji characters, use DECANCER_OPTION_RETAIN_CHINESE.
 * @since 3.0.0 
 */
#define DECANCER_OPTION_RETAIN_JAPANESE (1 << 18)

/**
 * @brief Prevents decancer from curing all korean characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_KOREAN (1 << 19)

/**
 * @brief Prevents decancer from curing all braille characters.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_BRAILLE (1 << 20)

/**
 * @brief Prevents decancer from curing all emojis.
 * @since 3.0.0
 */
#define DECANCER_OPTION_RETAIN_EMOJIS (1 << 21)

/**
 * @brief Removes all non-ASCII characters from the result.
 *
 * @see DECANCER_OPTION_ALPHANUMERIC_ONLY
 * @since 3.2.0
 */
#define DECANCER_OPTION_ASCII_ONLY (1 << 22)

/**
 * @brief Removes all non-alphanumeric characters from the result.
 *
 * @see DECANCER_OPTION_ASCII_ONLY
 * @since 3.2.0
 */
#define DECANCER_OPTION_ALPHANUMERIC_ONLY (1 << 23)

/**
 * @brief A configuration where every option is enabled.
 * @since 3.2.0
 */
#define DECANCER_OPTION_ALL 0xffffff

/**
 * @brief Prevents decancer from curing characters from major foreign writing systems, including diacritics.
 * @since 3.0.0
 */
#define DECANCER_OPTION_PURE_HOMOGLYPH 0x1ffffc

/**
 * @brief Represents an error caused by decancer not being able to cure a string.
 *
 * ```c
 * typedef struct {
 *   const char* message;
 *   uint8_t message_length;
 * } decancer_error_t;
 * ```
 *
 * @see decancer_cure
 * @see decancer_cure_utf16
 * @since 2.0.0
 */
typedef struct {
  /**
   * @brief Null-terminated ASCII encoded error message.
   * @since 2.0.0
   */
  const char* message;

  /**
   * @brief The length of the error message.
   * @since 2.0.0
   */
  uint8_t message_length;
} decancer_error_t;

/**
 * @brief Represents a UTF-8 encoded keyword. This struct is often used inside an array.
 *
 * ```c
 * typedef struct {
 *   const uint8_t* string;
 *   size_t size;
 * } decancer_keyword_t;
 * ```
 *
 * @see decancer_find_multiple
 * @see decancer_censor_multiple
 * @see decancer_replace_multiple
 * @since 3.2.2
 */
typedef struct {
  /**
   * @brief UTF-8 encoded string.
   * @since 3.2.2
   */
  const uint8_t* string;

  /**
   * @brief UTF-8 size of the string, in bytes.
   * @since 3.2.2
   */
  size_t size;
} decancer_keyword_t;

/**
 * @brief Represents a UTF-16 encoded keyword. This struct is often used inside an array.
 *
 * ```c
 * typedef struct {
 *   const uint16_t* string;
 *   size_t length;
 * } decancer_keyword_utf16_t;
 * ```
 *
 * @see decancer_find_multiple_utf16
 * @see decancer_censor_multiple_utf16
 * @see decancer_replace_multiple_utf16
 * @since 3.2.2
 */
typedef struct {
  /**
   * @brief UTF-16 encoded string.
   * @since 3.2.2
   */
  const uint16_t* string;

  /**
   * @brief Length of the UTF-16 encoded string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @since 3.2.2
   */
  size_t length;
} decancer_keyword_utf16_t;

/**
 * @brief Represents a rust object returned from decancer_cured_raw_utf16. This value has no use other than retaining the lifetime of the returned UTF-16 pointer.
 *
 * @see decancer_cured_raw_utf16
 * @see decancer_cured_raw_utf16_clone
 * @see decancer_cured_raw_utf16_free
 * @note You are responsible in freeing this object later by calling decancer_cured_raw_utf16_free.
 * @since 3.2.2
 */
typedef void* decancer_cured_raw_utf16_t;

/**
 * @brief Represents a UTF-8 matcher iterator object returned from decancer_find.
 *
 * @see decancer_matcher_utf16_t
 * @see decancer_find
 * @see decancer_matcher_next
 * @see decancer_matcher_consume
 * @see decancer_matcher_free
 * @note For its UTF-16 counterpart, see decancer_matcher_utf16_t.
 * @note You are responsible in freeing this object later by calling decancer_matcher_free.
 * @since 3.1.0
 */
typedef void* decancer_matcher_t;

/**
 * @brief Represents a UTF-16 matcher iterator object returned from decancer_find_utf16.
 *
 * @see decancer_matcher_t
 * @see decancer_find_utf16
 * @see decancer_matcher_utf16_consume
 * @see decancer_matcher_utf16_next
 * @see decancer_matcher_utf16_free
 * @note For its UTF-8 counterpart, see decancer_matcher_t.
 * @note You are responsible in freeing this object later by calling decancer_matcher_utf16_free.
 * @since 3.2.2
 */
typedef void* decancer_matcher_utf16_t;

/**
 * @brief Represents a matcher iterator object returned from decancer_find_multiple and decancer_find_multiple_utf16.
 *
 * @see decancer_find_multiple
 * @see decancer_find_multiple_utf16
 * @see decancer_matcher_consume
 * @see decancer_matcher_utf16_consume
 * @see decancer_matches_clone
 * @see decancer_matches_free
 * @note You are responsible in freeing this object later by calling decancer_matches_free.
 * @since 3.1.1
 */
typedef void* decancer_matches_t;

/**
 * @brief Represents a translation of a unicode codepoint.
 *
 * ```c
 * typedef struct {
 *   uint8_t kind;
 *
 *   union {
 *     uint32_t character;
 *
 *     struct {
 *       const uint8_t* contents;
 *       size_t size;
 *       void* __heap;
 *     } string;
 *   } contents;
 * } decancer_translation_t;
 * ```
 *
 * @see decancer_cure_char
 * @see decancer_translation_init
 * @see decancer_translation_clone
 * @see decancer_translation_free
 * @note You are responsible in freeing this object later passing it as a pointer to decancer_translation_free.
 * @warning You MUST pass this struct to decancer_translation_init first before using decancer_cure_char. Not doing so could result in possible undefined behavior.
 */
typedef struct {
  /**
   * @brief The type of the translation result. This can be any of the following values:
   *
   * @see DECANCER_TRANSLATION_KIND_CHARACTER
   * @see DECANCER_TRANSLATION_KIND_STRING
   * @see DECANCER_TRANSLATION_KIND_NONE
   */
  uint8_t kind;

  /**
   * @brief A union of translation results. This can either be a unicode character or a UTF-8 encoded string.
   * @since 1.5.4
   */
  union {
    /**
     * @brief The translation, as a unicode character.
     * @since 1.5.4
     */
    uint32_t character;

    /**
     * @brief The translation, as a UTF-8 encoded string.
     * @since 1.5.4
     */
    struct {
      /**
       * @brief Raw UTF-8 encoded string.
       * @since 1.5.4
       */
      const uint8_t* contents;

      /**
       * @brief UTF-8 size of the string, in bytes.
       * @since 1.5.4
       */
      size_t size;

      /**
       * @brief A pointer to a heap memory block, unused.
       * @note If this value is not NULL and kind is DECANCER_TRANSLATION_KIND_STRING, then you must pass this struct to decancer_translation_free later.
       * @since 3.1.0
       */
      void* __heap;
    } string;
  } contents;
} decancer_translation_t;

/**
 * @brief Represents a cured string returned from decancer_cure and decancer_cure_utf16.
 *
 * @see decancer_cure
 * @see decancer_cure_utf16
 * @see decancer_cured_clone
 * @see decancer_cured_free
 * @note You are responsible in freeing this object later by calling decancer_cured_free.
 * @since 1.5.3
 */
typedef void* decancer_cured_t;

/**
 * @brief Represents a match in UTF-8 indices.
 *
 * ```c
 * typedef struct {
 *   size_t start;
 *   size_t end;
 * } decancer_match_t;
 * ```
 *
 * @see decancer_find
 * @see decancer_find_utf16
 * @see decancer_matcher_consume
 * @see decancer_matcher_next
 * @since 3.1.0
 */
typedef struct {
  /**
   * @brief Start of the match in UTF-8 indices.
   * @since 3.1.0
   */
  size_t start;

  /**
   * @brief End of the match in UTF-8 indices (non-inclusive).
   * @since 3.1.0
   */
  size_t end;
} decancer_match_t;

/**
 * @brief An unsigned 32-bit bitflags that lets you customize decancer's behavior in its curing functions.
 *
 * @see decancer_cure
 * @see decancer_cure_utf16
 * @see decancer_cure_char
 * @see DECANCER_OPTION_DEFAULT
 * @see DECANCER_OPTION_RETAIN_CAPITALIZATION
 * @see DECANCER_OPTION_DISABLE_BIDI
 * @see DECANCER_OPTION_RETAIN_DIACRITICS
 * @see DECANCER_OPTION_RETAIN_GREEK
 * @see DECANCER_OPTION_RETAIN_CYRILLIC
 * @see DECANCER_OPTION_RETAIN_HEBREW
 * @see DECANCER_OPTION_RETAIN_ARABIC
 * @see DECANCER_OPTION_RETAIN_DEVANAGARI
 * @see DECANCER_OPTION_RETAIN_BENGALI
 * @see DECANCER_OPTION_RETAIN_ARMENIAN
 * @see DECANCER_OPTION_RETAIN_GUJARATI
 * @see DECANCER_OPTION_RETAIN_TAMIL
 * @see DECANCER_OPTION_RETAIN_THAI
 * @see DECANCER_OPTION_RETAIN_LAO
 * @see DECANCER_OPTION_RETAIN_BURMESE
 * @see DECANCER_OPTION_RETAIN_KHMER
 * @see DECANCER_OPTION_RETAIN_MONGOLIAN
 * @see DECANCER_OPTION_RETAIN_CHINESE
 * @see DECANCER_OPTION_RETAIN_JAPANESE
 * @see DECANCER_OPTION_RETAIN_KOREAN
 * @see DECANCER_OPTION_RETAIN_BRAILLE
 * @see DECANCER_OPTION_RETAIN_EMOJIS
 * @see DECANCER_OPTION_ASCII_ONLY
 * @see DECANCER_OPTION_ALPHANUMERIC_ONLY
 * @see DECANCER_OPTION_ALL
 * @see DECANCER_OPTION_PURE_HOMOGLYPH
 * @since 3.0.0
 */
typedef uint32_t decancer_options_t;

#ifdef __cplusplus
extern "C" {
#endif

  /**
   * @brief Cures a UTF-8 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * int main(void) {
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_cured_free(cured);
   *   return 0;
   * }
   * ```
   *
   * @param input_str The UTF-8 encoded string.
   * @param input_size UTF-8 size of the input string, in bytes.
   * @param options Options to customize decancer's curing behavior. To use decancer's default behavior, pass in DECANCER_OPTION_DEFAULT.
   * @param error A pointer to a decancer_error_t struct. This pointer can be NULL if you want to ignore errors.
   * @see decancer_cure_utf16
   * @see decancer_cure_char
   * @see decancer_cured_clone
   * @see decancer_cured_free
   * @return decancer_cured_t The cured string object or NULL failure -- see the modified error struct for more details.
   * @note For its UTF-16 counterpart, see decancer_cure_utf16.
   * @note You are responsible in freeing the returned object later by calling decancer_cured_free.
   * @since 1.5.3
   */
  DECANCER_EXPORT decancer_cured_t decancer_cure(const uint8_t* input_str, const size_t input_size, const decancer_options_t options, decancer_error_t* error);

  /**
   * @brief Cures a UTF-16 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * int main(void) {
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_cured_free(cured);
   *   return 0;
   * }
   * ```
   *
   * @param input_str The UTF-16 encoded string.
   * @param input_length Length of the UTF-16 encoded string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @param options Options to customize decancer's curing behavior. To use decancer's default behavior, pass in DECANCER_OPTION_DEFAULT.
   * @param error A pointer to a decancer_error_t struct. This pointer can be NULL if you want to ignore errors.
   * @see decancer_cure
   * @see decancer_cure_char
   * @see decancer_cured_clone
   * @see decancer_cured_free
   * @return decancer_cured_t The cured string object or NULL failure -- see the modified error struct for more details.
   * @note For its UTF-8 counterpart, see decancer_cure.
   * @note You are responsible in freeing the returned object later by calling decancer_cured_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT decancer_cured_t decancer_cure_utf16(const uint16_t* input_str, const size_t input_length, const decancer_options_t options, decancer_error_t* error);

  /**
   * @brief Cures a single unicode codepoint.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *   decancer_translation_t translation;
   *   decancer_translation_init(&translation);
   *
   *   decancer_cure_char(0xFF25, DECANCER_OPTION_DEFAULT, &translation);
   *
   *   decancer_assert(translation.kind == DECANCER_TRANSLATION_KIND_CHARACTER, "translation not a character");
   *   decancer_assert(translation.contents.character == 0x65, "character translation contents");
   *
   *   decancer_cure_char(0x04D5, DECANCER_OPTION_DEFAULT, &translation);
   *
   *   decancer_assert(translation.kind == DECANCER_TRANSLATION_KIND_STRING, "translation not a string");
   *   decancer_assert(translation.contents.string.size == 2, "string translation size");
   *   decancer_assert(translation.contents.string.contents[0] == 'a' && translation.contents.string.contents[1] == 'e', "string translation contents");
   *
   *   decancer_cure_char(0, DECANCER_OPTION_DEFAULT, &translation);
   *
   *   decancer_assert(translation.kind == DECANCER_TRANSLATION_KIND_NONE, "translation not an empty string");
   *
   * END:
   *   decancer_translation_free(&translation);
   *   return ret;
   * }
   * ```
   *
   * @param input The unicode codepoint.
   * @param options Options to customize decancer's curing behavior. To use decancer's default behavior, pass in DECANCER_OPTION_DEFAULT.
   * @param translation A pointer to the output translation struct.
   * @see decancer_cure
   * @see decancer_cure_utf16
   * @see decancer_translation_init
   * @see decancer_translation_clone
   * @see decancer_translation_free
   * @note You are responsible in freeing the translation struct later by passing it as a pointer to decancer_translation_free.
   * @warning You MUST pass the translation struct to decancer_translation_init first before using this function. Not doing so could result in possible undefined behavior.
   * @since 1.5.4
   */
  DECANCER_EXPORT void decancer_cure_char(const uint32_t input, const decancer_options_t options, decancer_translation_t* translation);

  /**
   * @brief Retrieves the raw UTF-8 bytes from a cured string object.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   size_t raw_contents_size;
   *   const uint8_t* raw_contents = decancer_cured_raw(cured, NULL, &raw_contents_size);
   *
   *   decancer_assert(raw_contents_size == 15, "size of very funny text");
   *   decancer_assert(!memcmp(raw_contents, "very funny text", raw_contents_size), "contents of very funny text");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param match A pointer to a match object if you just want a slice, otherwise NULL if you want the entire string.
   * @param output_size A pointer to the output's UTF-8 size, in bytes.
   * @return const uint8_t* An immutable UTF-8 pointer representing raw contents of the cured string object.
   * @see decancer_cured_raw_utf16
   * @note For its UTF-16 counterpart, see decancer_cured_raw_utf16.
   * @note The returned pointer remains valid until cured gets passed onto decancer_cured_free.
   * @since 3.1.1
   */
  DECANCER_EXPORT const uint8_t* decancer_cured_raw(decancer_cured_t cured, const decancer_match_t* match, size_t* output_size);

  /**
   * @brief Retrieves the raw UTF-16 bytes from a cured string object.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "very funny text"
   *   uint16_t expected_contents[] = { 0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e, 0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74 };
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   size_t raw_contents_length;
   *   uint16_t* raw_contents;
   *   decancer_cured_raw_utf16_t raw_contents_handle = decancer_cured_raw_utf16(cured, NULL, &raw_contents, &raw_contents_length);
   *
   *   decancer_assert(raw_contents_length == (sizeof(expected_contents) / sizeof(uint16_t)), "length of very funny text");
   *   decancer_assert(!memcmp(raw_contents, expected_contents, sizeof(expected_contents)), "contents of very funny text");
   *
   * END:
   *   decancer_cured_raw_utf16_free(raw_contents_handle);
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param match A pointer to a match object if you just want a slice, otherwise NULL if you want the entire string.
   * @param output_ptr A pointer to the output's UTF-16 encoded string.
   * @param output_length A pointer to the length of the UTF-16 encoded string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @see decancer_cured_raw
   * @see decancer_cured_raw_utf16_clone
   * @see decancer_cured_raw_utf16_free
   * @return decancer_cured_raw_utf16_t A rust object. This value has no use other than retaining the lifetime of the returned UTF-16 pointer.
   * @note For its UTF-8 counterpart, see decancer_cured_raw.
   * @note You are responsible in freeing the returned object later by calling decancer_cured_raw_utf16_free.
   * @note The lifetime of the UTF-16 encoded string remains valid until the returned object gets passed onto decancer_cured_raw_utf16_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT decancer_cured_raw_utf16_t decancer_cured_raw_utf16(decancer_cured_t cured, const decancer_match_t* match, uint16_t** output_ptr, size_t* output_length);

  /**
   * @brief Returns the raw list of every similar-looking match from a decancer_matches_t object.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_keyword_t keywords[] = {
   *     {"very", 4},
   *     {"funny", 5}
   *   };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   decancer_matches_t matches;
   *   size_t raw_matches_length;
   *   const decancer_match_t* raw_matches;
   *   const uint8_t* raw_contents;
   *   size_t raw_contents_size;
   *
   *   cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   matches = decancer_find_multiple(cured, keywords, sizeof(keywords) / sizeof(decancer_keyword_t));
   *   decancer_assert(matches != NULL, "decancer_find_multiple", CURED_END);
   *
   *   raw_matches = decancer_matches_raw(matches, &raw_matches_length);
   *
   *   decancer_assert(raw_matches_length == 2, "raw_matches_length", MATCHES_END);
   *
   *   decancer_assert(raw_matches[0].start == 0, "start of very", MATCHES_END);
   *   decancer_assert(raw_matches[0].end == 4, "end of very", MATCHES_END);
   *
   *   raw_contents = decancer_cured_raw(cured, &raw_matches[0], &raw_contents_size);
   *
   *   decancer_assert(raw_contents_size == 4, "size of very", MATCHES_END);
   *   decancer_assert(!memcmp(raw_contents, "very", raw_contents_size), "contents of very", MATCHES_END);
   *
   *   decancer_assert(raw_matches[1].start == 5, "start of funny", MATCHES_END);
   *   decancer_assert(raw_matches[1].end == 10, "end of funny", MATCHES_END);
   *
   *   raw_contents = decancer_cured_raw(cured, &raw_matches[1], &raw_contents_size);
   *
   *   decancer_assert(raw_contents_size == 5, "size of funny", MATCHES_END);
   *   decancer_assert(!memcmp(raw_contents, "funny", raw_contents_size), "contents of funny", MATCHES_END);
   *
   * MATCHES_END:
   *   decancer_matches_free(matches);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param matches The matches object.
   * @param output_size A pointer to the output's array size.
   * @return const decancer_match_t* The raw pointer containing every similar-looking match.
   * @note The returned pointer remains valid until the matches object gets passed onto decancer_matches_free.
   * @since 3.1.1
   */
  DECANCER_EXPORT const decancer_match_t* decancer_matches_raw(decancer_matches_t matches, size_t* output_size);

  /**
   * @brief Finds every similar-looking match of a UTF-8 encoded string in the cured string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_size;
   *   const uint8_t* raw_contents;
   *   decancer_matcher_t matcher;
   *   decancer_match_t match;
   *
   *   cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   matcher = decancer_find(cured, "funny", 5);
   *   decancer_assert(matcher != NULL, "decancer_find", CURED_END);
   *
   *   decancer_assert(decancer_matcher_next(matcher, &match), "first iteration of decancer_matcher_next", MATCHER_END);
   *
   *   decancer_assert(match.start == 5, "start of funny", MATCHER_END);
   *   decancer_assert(match.end == 10, "end of funny", MATCHER_END);
   *
   *   raw_contents = decancer_cured_raw(cured, &match, &raw_contents_size);
   *
   *   decancer_assert(raw_contents_size == 5, "size of funny", MATCHER_END);
   *   decancer_assert(!memcmp(raw_contents, "funny", raw_contents_size), "contents of funny", MATCHER_END);
   *
   *   decancer_assert(!decancer_matcher_next(matcher, &match), "no more matches", MATCHER_END);
   *
   * MATCHER_END:
   *   decancer_matcher_free(matcher);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 encoded string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_find_utf16
   * @see decancer_find_multiple
   * @see decancer_find_multiple_utf16
   * @see decancer_matcher_consume
   * @see decancer_matcher_next
   * @see decancer_matcher_free
   * @return decancer_matcher_t A UTF-8 matcher iterator object or NULL if the other string is not properly UTF-8 encoded.
   * @note For its UTF-16 counterpart, see decancer_find_utf16.
   * @note You are responsible in freeing the returned object later by calling decancer_matcher_free.
   * @since 3.1.0
   */
  DECANCER_EXPORT decancer_matcher_t decancer_find(decancer_cured_t cured, const uint8_t* other_str, const size_t other_size);

  /**
   * @brief Finds every similar-looking match of a UTF-16 encoded string in the cured string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "funny"
   *   uint16_t funny[] = { 0x66, 0x75, 0x6e, 0x6e, 0x79 };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   decancer_cured_raw_utf16_t raw_contents_handle;
   *   size_t raw_contents_length;
   *   uint16_t* raw_contents;
   *   decancer_matcher_utf16_t matcher;
   *   decancer_match_t match;
   *
   *   cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   matcher = decancer_find_utf16(cured, funny, sizeof(funny) / sizeof(uint16_t));
   *   decancer_assert(matcher != NULL, "decancer_find_utf16", CURED_END);
   *
   *   decancer_assert(decancer_matcher_utf16_next(matcher, &match), "first iteration of decancer_matcher_next", MATCHER_END);
   *
   *   decancer_assert(match.start == 5, "start of funny", RAW_CONTENTS_END);
   *   decancer_assert(match.end == 10, "end of funny", RAW_CONTENTS_END);
   *
   *   raw_contents_handle = decancer_cured_raw_utf16(cured, &match, &raw_contents, &raw_contents_length);
   *
   *   decancer_assert(raw_contents_length == (sizeof(funny) / sizeof(uint16_t)), "length of funny", RAW_CONTENTS_END);
   *   decancer_assert(!memcmp(raw_contents, funny, sizeof(funny)), "contents of funny", RAW_CONTENTS_END);
   *
   *   decancer_assert(!decancer_matcher_utf16_next(matcher, &match), "end of iteration", RAW_CONTENTS_END);
   *
   * RAW_CONTENTS_END:
   *   decancer_cured_raw_utf16_free(raw_contents_handle);
   * MATCHER_END:
   *   decancer_matcher_utf16_free(matcher);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 encoded string to match with.
   * @param other_length Length of the UTF-16 encoded string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @see decancer_find
   * @see decancer_find_multiple
   * @see decancer_find_multiple_utf16
   * @see decancer_matcher_utf16_consume
   * @see decancer_matcher_utf16_next
   * @see decancer_matcher_utf16_free
   * @return decancer_matcher_utf16_t A UTF-16 matcher iterator object or NULL if the other string is not properly UTF-8 encoded.
   * @note For its UTF-8 counterpart, see decancer_find.
   * @note You are responsible in freeing the returned object later by calling decancer_matcher_utf16_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT decancer_matcher_utf16_t decancer_find_utf16(decancer_cured_t cured, const uint16_t* other_str, const size_t other_length);

  /**
   * @brief Finds every similar-looking match from a list of UTF-8 keywords in the cured string.
   * Unlike decancer_find, this function also takes note of overlapping matches and merges them together.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_keyword_t keywords[] = {
   *     {"very", 4},
   *     {"funny", 5}
   *   };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   decancer_matches_t matches;
   *   size_t raw_matches_length;
   *   const decancer_match_t* raw_matches;
   *   const uint8_t* raw_contents;
   *   size_t raw_contents_size;
   *
   *   cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   matches = decancer_find_multiple(cured, keywords, sizeof(keywords) / sizeof(decancer_keyword_t));
   *   decancer_assert(matches != NULL, "decancer_find_multiple", CURED_END);
   *
   *   raw_matches = decancer_matches_raw(matches, &raw_matches_length);
   *
   *   decancer_assert(raw_matches_length == 2, "raw_matches_length", MATCHES_END);
   *
   *   decancer_assert(raw_matches[0].start == 0, "start of very", MATCHES_END);
   *   decancer_assert(raw_matches[0].end == 4, "end of very", MATCHES_END);
   *
   *   raw_contents = decancer_cured_raw(cured, &raw_matches[0], &raw_contents_size);
   *
   *   decancer_assert(raw_contents_size == 4, "size of very", MATCHES_END);
   *   decancer_assert(!memcmp(raw_contents, "very", raw_contents_size), "contents of very", MATCHES_END);
   *
   *   decancer_assert(raw_matches[1].start == 5, "start of funny", MATCHES_END);
   *   decancer_assert(raw_matches[1].end == 10, "end of funny", MATCHES_END);
   *
   *   raw_contents = decancer_cured_raw(cured, &raw_matches[1], &raw_contents_size);
   *
   *   decancer_assert(raw_contents_size == 5, "size of funny", MATCHES_END);
   *   decancer_assert(!memcmp(raw_contents, "funny", raw_contents_size), "contents of funny", MATCHES_END);
   *
   * MATCHES_END:
   *   decancer_matches_free(matches);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other A list of UTF-8 keywords to match with.
   * @param other_length Length of the keywords array in units of decancer_keyword_t -- or sizeof(array) / sizeof(decancer_keyword_t).
   * @see decancer_find
   * @see decancer_find_utf16
   * @see decancer_find_multiple_utf16
   * @see decancer_matcher_consume
   * @see decancer_matcher_utf16_consume
   * @see decancer_matches_clone
   * @see decancer_matches_free
   * @return decancer_matches_t A matches object or NULL if the keywords are not properly UTF-8 encoded.
   * @note For its UTF-16 counterpart, see decancer_find_multiple_utf16.
   * @note You are responsible in freeing the returned object later by calling decancer_matches_free.
   * @since 3.1.1
   */
  DECANCER_EXPORT decancer_matches_t decancer_find_multiple(decancer_cured_t cured, const decancer_keyword_t* other, const size_t other_length);

  /**
   * @brief Finds every similar-looking match from a list of UTF-16 keywords in the cured string.
   * Unlike decancer_find_utf16, this function also takes note of overlapping matches and merges them together.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "very"
   *   uint16_t very[] = { 0x76, 0x65, 0x72, 0x79 };
   *
   *   // UTF-16 bytes for "funny"
   *   uint16_t funny[] = { 0x66, 0x75, 0x6e, 0x6e, 0x79 };
   *
   *   decancer_keyword_utf16_t keywords[] = {
   *     {very, sizeof(very) / sizeof(uint16_t)},
   *     {funny, sizeof(funny) / sizeof(uint16_t)}
   *   };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   decancer_matches_t matches;
   *   size_t raw_matches_length;
   *   const decancer_match_t* raw_matches;
   *   decancer_cured_raw_utf16_t raw_contents_handle;
   *   uint16_t* raw_contents;
   *   size_t raw_contents_length;
   *
   *   cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   matches = decancer_find_multiple_utf16(cured, keywords, sizeof(keywords) / sizeof(decancer_keyword_utf16_t));
   *   decancer_assert(matches != NULL, "decancer_find_multiple_utf16", CURED_END);
   *
   *   raw_matches = decancer_matches_raw(matches, &raw_matches_length);
   *
   *   decancer_assert(raw_matches_length == 2, "raw_matches_length", MATCHES_END);
   *
   *   decancer_assert(raw_matches[0].start == 0, "start of very", MATCHES_END);
   *   decancer_assert(raw_matches[0].end == 4, "end of very", MATCHES_END);
   *
   *   raw_contents_handle = decancer_cured_raw_utf16(cured, &raw_matches[0], &raw_contents, &raw_contents_length);
   *
   *   decancer_assert(raw_contents_length == (sizeof(very) / sizeof(uint16_t)), "length of very", RAW_CONTENTS_END);
   *   decancer_assert(!memcmp(raw_contents, very, sizeof(very)), "contents of very", RAW_CONTENTS_END);
   *
   *   decancer_assert(raw_matches[1].start == 5, "start of funny", RAW_CONTENTS_END);
   *   decancer_assert(raw_matches[1].end == 10, "end of funny", RAW_CONTENTS_END);
   *
   *   decancer_cured_raw_utf16_free(raw_contents_handle);
   *   raw_contents_handle = decancer_cured_raw_utf16(cured, &raw_matches[1], &raw_contents, &raw_contents_length);
   *
   *   decancer_assert(raw_contents_length == (sizeof(funny) / sizeof(uint16_t)), "length of funny", RAW_CONTENTS_END);
   *   decancer_assert(!memcmp(raw_contents, funny, sizeof(funny)), "contents of funny", RAW_CONTENTS_END);
   *
   * RAW_CONTENTS_END:
   *   decancer_cured_raw_utf16_free(raw_contents_handle);
   * MATCHES_END:
   *   decancer_matches_free(matches);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other A list of UTF-16 keywords to match with.
   * @param other_length Length of the keywords array in units of decancer_keyword_utf16_t -- or sizeof(array) / sizeof(decancer_keyword_utf16_t).
   * @see decancer_find
   * @see decancer_find_utf16
   * @see decancer_matcher_consume
   * @see decancer_matcher_utf16_consume
   * @see decancer_matches_clone
   * @see decancer_matches_free
   * @return decancer_matches_t A matches object or NULL if the keywords are not properly UTF-8 encoded.
   * @note For its UTF-8 counterpart, see decancer_find_multiple.
   * @note You are responsible in freeing the returned object later by calling decancer_matches_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT decancer_matches_t decancer_find_multiple_utf16(decancer_cured_t cured, const decancer_keyword_utf16_t* other, const size_t other_length);

  /**
   * @brief Iterates to the next element of a UTF-8 matcher iterator.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_size;
   *   const uint8_t* raw_contents;
   *   decancer_matcher_t matcher;
   *   decancer_match_t match;
   *
   *   cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   matcher = decancer_find(cured, "funny", 5);
   *   decancer_assert(matcher != NULL, "decancer_find", CURED_END);
   *
   *   decancer_assert(decancer_matcher_next(matcher, &match), "first iteration of decancer_matcher_next", MATCHER_END);
   *
   *   decancer_assert(match.start == 5, "start of funny", MATCHER_END);
   *   decancer_assert(match.end == 10, "end of funny", MATCHER_END);
   *
   *   raw_contents = decancer_cured_raw(cured, &match, &raw_contents_size);
   *
   *   decancer_assert(raw_contents_size == 5, "size of funny", MATCHER_END);
   *   decancer_assert(!memcmp(raw_contents, "funny", raw_contents_size), "contents of funny", MATCHER_END);
   *
   *   decancer_assert(!decancer_matcher_next(matcher, &match), "no more matches", MATCHER_END);
   *
   * MATCHER_END:
   *   decancer_matcher_free(matcher);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param matcher The UTF-8 matcher iterator object.
   * @param match A pointer to a decancer_match_t struct.
   * @see decancer_find
   * @see decancer_matcher_utf16_next
   * @see decancer_matcher_free
   * @return bool true if a new value is present, or false if the iteration is complete.
   * @note For its UTF-16 counterpart, see decancer_matcher_utf16_next.
   * @since 3.1.0
   */
  DECANCER_EXPORT bool decancer_matcher_next(decancer_matcher_t matcher, decancer_match_t* match);

  /**
   * @brief Iterates to the next element of a UTF-16 matcher iterator.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "funny"
   *   uint16_t funny[] = { 0x66, 0x75, 0x6e, 0x6e, 0x79 };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   decancer_cured_raw_utf16_t raw_contents_handle;
   *   size_t raw_contents_length;
   *   uint16_t* raw_contents;
   *   decancer_matcher_utf16_t matcher;
   *   decancer_match_t match;
   *
   *   cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   matcher = decancer_find_utf16(cured, funny, sizeof(funny) / sizeof(uint16_t));
   *   decancer_assert(matcher != NULL, "decancer_find_utf16", CURED_END);
   *
   *   decancer_assert(decancer_matcher_utf16_next(matcher, &match), "first iteration of decancer_matcher_next", MATCHER_END);
   *
   *   decancer_assert(match.start == 5, "start of funny", RAW_CONTENTS_END);
   *   decancer_assert(match.end == 10, "end of funny", RAW_CONTENTS_END);
   *
   *   raw_contents_handle = decancer_cured_raw_utf16(cured, &match, &raw_contents, &raw_contents_length);
   *
   *   decancer_assert(raw_contents_length == (sizeof(funny) / sizeof(uint16_t)), "length of funny", RAW_CONTENTS_END);
   *   decancer_assert(!memcmp(raw_contents, funny, sizeof(funny)), "contents of funny", RAW_CONTENTS_END);
   *
   *   decancer_assert(!decancer_matcher_utf16_next(matcher, &match), "end of iteration", RAW_CONTENTS_END);
   *
   * RAW_CONTENTS_END:
   *   decancer_cured_raw_utf16_free(raw_contents_handle);
   * MATCHER_END:
   *   decancer_matcher_utf16_free(matcher);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param matcher The UTF-16 matcher iterator object.
   * @param match A pointer to a decancer_match_t struct.
   * @return bool true if a new value is present, or false if the iteration is complete.
   * @see decancer_find_utf16
   * @see decancer_matcher_next
   * @see decancer_matcher_utf16_free
   * @note For its UTF-8 counterpart, see decancer_matcher_next.
   * @since 3.2.2
   */
  DECANCER_EXPORT bool decancer_matcher_utf16_next(decancer_matcher_utf16_t matcher, decancer_match_t* match);

  /**
   * @brief Censors every similar-looking match of the specified UTF-8 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_size;
   *   const uint8_t* raw_contents;
   *
   *   cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   // 0x2a is the ASCII representation of '*'
   *   decancer_assert(decancer_censor(cured, "funny", 5, 0x2a), "decancer_censor");
   *
   *   raw_contents = decancer_cured_raw(cured, NULL, &raw_contents_size);
   *   decancer_assert(!memcmp(raw_contents, "very ***** text", raw_contents_size), "censor result");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 encoded string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @param replacement_char The censor unicode codepoint. Ideally '*' (0x2a) or '-' (0x2a).
   * @see decancer_censor_utf16
   * @see decancer_censor_multiple
   * @see decancer_censor_multiple_utf16
   * @return bool true on success, or false on failure due to invalid encoding.
   * @note For its UTF-16 counterpart, see decancer_censor_utf16.
   * @since 3.1.1
   */
  DECANCER_EXPORT bool decancer_censor(decancer_cured_t cured, const uint8_t* other_str, const size_t other_size, const uint32_t replacement_char);

  /**
   * @brief Censors every similar-looking match of the specified UTF-16 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "funny"
   *   uint16_t funny[] = { 0x66, 0x75, 0x6e, 0x6e, 0x79 };
   *
   *   // UTF-16 bytes for "very ***** text"
   *   uint16_t expected_contents[] = { 0x76, 0x65, 0x72, 0x79, 0x20, 0x2a, 0x2a, 0x2a, 0x2a, 0x2a, 0x20, 0x74, 0x65, 0x78, 0x74 };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_length;
   *   uint16_t* raw_contents;
   *   decancer_cured_raw_utf16_t raw_contents_handle;
   *
   *   cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   // 0x2a is the ASCII representation of '*'
   *   decancer_assert(decancer_censor_utf16(cured, funny, sizeof(funny) / sizeof(uint16_t), 0x2a), "decancer_censor_utf16", CURED_END);
   *
   *   raw_contents_handle = decancer_cured_raw_utf16(cured, NULL, &raw_contents, &raw_contents_length);
   *
   *   decancer_assert(raw_contents_length == (sizeof(expected_contents) / sizeof(uint16_t)), "length of censor result", RAW_CONTENTS_END);
   *   decancer_assert(!memcmp(raw_contents, expected_contents, sizeof(expected_contents)), "censor result", RAW_CONTENTS_END);
   *
   * RAW_CONTENTS_END:
   *   decancer_cured_raw_utf16_free(raw_contents_handle);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 encoded string to match with.
   * @param other_length Length of the UTF-16 encoded string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @param replacement_char The censor unicode codepoint. Ideally '*' (0x2a) or '-' (0x2a).
   * @see decancer_censor
   * @see decancer_censor_multiple
   * @see decancer_censor_multiple_utf16
   * @return bool true on success, or false on failure due to invalid encoding.
   * @note For its UTF-8 counterpart, see decancer_censor.
   * @since 3.2.2
   */
  DECANCER_EXPORT bool decancer_censor_utf16(decancer_cured_t cured, const uint16_t* other_str, const size_t other_length, const uint32_t replacement_char);

  /**
   * @brief Replaces every similar-looking match of the specified UTF-8 encoded string with another UTF-8 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_size;
   *   const uint8_t* raw_contents;
   *
   *   cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_replace(cured, "very", 4, "not", 3), "decancer_replace");
   *
   *   raw_contents = decancer_cured_raw(cured, NULL, &raw_contents_size);
   *   decancer_assert(!memcmp(raw_contents, "not funny text", raw_contents_size), "replace result");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 encoded string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @param replacement_str The UTF-8 encoded string to replace with.
   * @param replacement_size UTF-8 size of the replacement string, in bytes.
   * @see decancer_replace_utf16
   * @see decancer_replace_multiple
   * @see decancer_replace_multiple_utf16
   * @return bool true on success, or false on failure due to invalid encoding.
   * @note For its UTF-16 counterpart, see decancer_replace_utf16.
   * @since 3.1.1
   */
  DECANCER_EXPORT bool decancer_replace(decancer_cured_t cured, const uint8_t* other_str, const size_t other_size, const uint8_t* replacement_str, const size_t replacement_size);

  /**
   * @brief Replaces every similar-looking match of the specified UTF-16 encoded string with another UTF-16 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "not"
   *   uint16_t not[] = { 0x6e, 0x6f, 0x74 };
   *
   *   // UTF-16 bytes for "very"
   *   uint16_t very[] = { 0x76, 0x65, 0x72, 0x79 };
   *
   *   // UTF-16 bytes for "not funny text"
   *   uint16_t expected_contents[] = { 0x6e, 0x6f, 0x74, 0x20, 0x66, 0x75, 0x6e, 0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74 };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_length;
   *   uint16_t* raw_contents;
   *   decancer_cured_raw_utf16_t raw_contents_handle;
   *
   *   cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_replace_utf16(cured, very, sizeof(very) / sizeof(uint16_t), not, sizeof(not) / sizeof(uint16_t)), "decancer_replace_utf16", CURED_END);
   *
   *   raw_contents_handle = decancer_cured_raw_utf16(cured, NULL, &raw_contents, &raw_contents_length);
   *
   *   decancer_assert(raw_contents_length == (sizeof(expected_contents) / sizeof(uint16_t)), "length of replace result", RAW_CONTENTS_END);
   *   decancer_assert(!memcmp(raw_contents, expected_contents, sizeof(expected_contents)), "replace result", RAW_CONTENTS_END);
   *
   * RAW_CONTENTS_END:
   *   decancer_cured_raw_utf16_free(raw_contents_handle);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 encoded string to match with.
   * @param other_length Length of the search string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @param replacement_str The UTF-16 encoded string to replace with.
   * @param replacement_length Length of the replacement string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @see decancer_replace
   * @see decancer_replace_multiple
   * @see decancer_replace_multiple_utf16
   * @return bool true on success, or false on failure due to invalid encoding.
   * @note For its UTF-8 counterpart, see decancer_replace.
   * @since 3.2.2
   */
  DECANCER_EXPORT bool decancer_replace_utf16(decancer_cured_t cured, const uint16_t* other_str, const size_t other_length, const uint16_t* replacement_str, const size_t replacement_length);

  /**
   * @brief Censors every similar-looking match of the specified list of UTF-8 keywords.
   * Unlike decancer_censor, this function also takes note of overlapping matches.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_keyword_t keywords[] = {
   *     {"very", 4},
   *     {"funny", 5}
   *   };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_size;
   *   const uint8_t* raw_contents;
   *
   *   cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   // 0x2a is the ASCII representation of '*'
   *   decancer_assert(decancer_censor_multiple(cured, keywords, sizeof(keywords) / sizeof(decancer_keyword_t), 0x2a), "decancer_censor_multiple");
   *
   *   raw_contents = decancer_cured_raw(cured, NULL, &raw_contents_size);
   *   decancer_assert(!memcmp(raw_contents, "**** ***** text", raw_contents_size), "censor multiple result");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other A list of UTF-8 keywords to match with.
   * @param other_length Length of the keywords array in units of decancer_keyword_t -- or sizeof(array) / sizeof(decancer_keyword_t).
   * @param replacement_char The censor unicode codepoint. Ideally '*' (0x2a) or '-' (0x2a).
   * @see decancer_censor
   * @see decancer_censor_utf16
   * @see decancer_censor_multiple_utf16
   * @return bool true on success, or false on failure due to invalid encoding.
   * @note For its UTF-16 counterpart, see decancer_censor_multiple_utf16.
   * @since 3.1.1
   */
  DECANCER_EXPORT bool decancer_censor_multiple(decancer_cured_t cured, const decancer_keyword_t* other, const size_t other_length, const uint32_t replacement_char);

  /**
   * @brief Censors every similar-looking match of the specified list of UTF-16 keywords.
   * Unlike decancer_censor_utf16, this function also takes note of overlapping matches.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "very"
   *   uint16_t very[] = { 0x76, 0x65, 0x72, 0x79 };
   *
   *   // UTF-16 bytes for "funny"
   *   uint16_t funny[] = { 0x66, 0x75, 0x6e, 0x6e, 0x79 };
   *
   *   // UTF-16 bytes for "**** ***** text"
   *   uint16_t expected_contents[] = { 0x2a, 0x2a, 0x2a, 0x2a, 0x20, 0x2a, 0x2a, 0x2a, 0x2a, 0x2a, 0x20, 0x74, 0x65, 0x78, 0x74 };
   *
   *   decancer_keyword_utf16_t keywords[] = {
   *     {very, sizeof(very) / sizeof(uint16_t)},
   *     {funny, sizeof(funny) / sizeof(uint16_t)}
   *   };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_length;
   *   uint16_t* raw_contents;
   *   decancer_cured_raw_utf16_t raw_contents_handle;
   *
   *   cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   // 0x2a is the ASCII representation of '*'
   *   decancer_assert(decancer_censor_multiple_utf16(cured, keywords, sizeof(keywords) / sizeof(decancer_keyword_utf16_t), 0x2a), "decancer_censor_multiple_utf16", CURED_END);
   *
   *   raw_contents_handle = decancer_cured_raw_utf16(cured, NULL, &raw_contents, &raw_contents_length);
   *
   *   decancer_assert(raw_contents_length == (sizeof(expected_contents) / sizeof(uint16_t)), "length of censor multiple result", RAW_CONTENTS_END);
   *   decancer_assert(!memcmp(raw_contents, expected_contents, sizeof(expected_contents)), "censor multiple result", RAW_CONTENTS_END);
   *
   * RAW_CONTENTS_END:
   *   decancer_cured_raw_utf16_free(raw_contents_handle);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other A list of UTF-16 keywords to match with.
   * @param other_length Length of the keywords array in units of decancer_keyword_t -- or sizeof(array) / sizeof(decancer_keyword_t).
   * @param replacement_char The censor unicode codepoint. Ideally '*' (0x2a) or '-' (0x2a).
   * @see decancer_censor
   * @see decancer_censor_utf16
   * @see decancer_censor_multiple
   * @return bool true on success, or false on failure due to invalid encoding.
   * @note For its UTF-8 counterpart, see decancer_censor_multiple.
   * @since 3.2.2
   */
  DECANCER_EXPORT bool decancer_censor_multiple_utf16(decancer_cured_t cured, const decancer_keyword_utf16_t* other, const size_t other_length, const uint32_t replacement_char);

  /**
   * @brief Replaces every similar-looking match of the specified list of UTF-8 keywords with another UTF-8 encoded string.
   * Unlike decancer_replace, this function also takes note of overlapping matches.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_keyword_t keywords[] = {
   *     {"very", 4},
   *     {"funny", 5}
   *   };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_size;
   *   const uint8_t* raw_contents;
   *
   *   cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_replace_multiple(cured, keywords, sizeof(keywords) / sizeof(decancer_keyword_t), "sussy", 5), "decancer_replace_multiple");
   *
   *   raw_contents = decancer_cured_raw(cured, NULL, &raw_contents_size);
   *   decancer_assert(!memcmp(raw_contents, "sussy sussy text", raw_contents_size), "replace multiple result");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other A list of UTF-8 keywords to match with.
   * @param other_length Length of the keywords array in units of decancer_keyword_t -- or sizeof(array) / sizeof(decancer_keyword_t).
   * @param replacement_str The UTF-8 encoded string to replace with.
   * @param replacement_size UTF-8 size of the replacement string, in bytes.
   * @see decancer_replace
   * @see decancer_replace_utf16
   * @see decancer_replace_multiple_utf16
   * @return bool true on success, or false on failure due to invalid encoding.
   * @note For its UTF-16 counterpart, see decancer_replace_multiple_utf16.
   * @since 3.1.1
   */
  DECANCER_EXPORT bool decancer_replace_multiple(decancer_cured_t cured, const decancer_keyword_t* other, const size_t other_length, const uint8_t* replacement_str, const size_t replacement_size);

  /**
   * @brief Replaces every similar-looking match of the specified list of UTF-16 keywords with another UTF-16 encoded string.
   * Unlike decancer_replace_utf16, this function also takes note of overlapping matches.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "very"
   *   uint16_t very[] = { 0x76, 0x65, 0x72, 0x79 };
   *
   *   // UTF-16 bytes for "funny"
   *   uint16_t funny[] = { 0x66, 0x75, 0x6e, 0x6e, 0x79 };
   *
   *   // UTF-16 bytes for "sussy"
   *   uint16_t sussy[] = { 0x73, 0x75, 0x73, 0x73, 0x79 };
   *
   *   // UTF-16 bytes for "sussy sussy text"
   *   uint16_t expected_contents[] = { 0x73, 0x75, 0x73, 0x73, 0x79, 0x20, 0x73, 0x75, 0x73, 0x73, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74 };
   *
   *   decancer_keyword_utf16_t keywords[] = {
   *     {very, sizeof(very) / sizeof(uint16_t)},
   *     {funny, sizeof(funny) / sizeof(uint16_t)}
   *   };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   size_t raw_contents_length;
   *   uint16_t* raw_contents;
   *   decancer_cured_raw_utf16_t raw_contents_handle;
   *
   *   cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_replace_multiple_utf16(cured, keywords, sizeof(keywords) / sizeof(decancer_keyword_utf16_t), sussy, sizeof(sussy) / sizeof(uint16_t)), "decancer_replace_multiple_utf16", CURED_END);
   *
   *   raw_contents_handle = decancer_cured_raw_utf16(cured, NULL, &raw_contents, &raw_contents_length);
   *
   *   decancer_assert(raw_contents_length == (sizeof(expected_contents) / sizeof(uint16_t)), "length of replace multiple result", RAW_CONTENTS_END);
   *   decancer_assert(!memcmp(raw_contents, expected_contents, sizeof(expected_contents)), "replace multiple result", RAW_CONTENTS_END);
   *
   * RAW_CONTENTS_END:
   *   decancer_cured_raw_utf16_free(raw_contents_handle);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other A list of UTF-16 keywords to match with.
   * @param other_length Length of the keywords array in units of decancer_keyword_t -- or sizeof(array) / sizeof(decancer_keyword_t).
   * @param replacement_str The UTF-16 encoded string to replace with.
   * @param replacement_length Length of the replacement string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @see decancer_replace
   * @see decancer_replace_utf16
   * @see decancer_replace_multiple
   * @return bool true on success, or false on failure due to invalid encoding.
   * @note For its UTF-8 counterpart, see decancer_replace_multiple.
   * @since 3.2.2
   */
  DECANCER_EXPORT bool decancer_replace_multiple_utf16(decancer_cured_t cured, const decancer_keyword_utf16_t* other, const size_t other_length, const uint16_t* replacement_str, const size_t replacement_length);

  /**
   * @brief Checks if the cured string similarly contains the specified UTF-8 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_contains(cured, "funny", 5), "decancer_contains");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 encoded string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_contains_utf16
   * @return bool true if the cured string similarly contains the specified string, false otherwise.
   * @note For its UTF-16 counterpart, see decancer_contains_utf16.
   * @since 1.5.3
   */
  DECANCER_EXPORT bool decancer_contains(decancer_cured_t cured, const uint8_t* other_str, const size_t other_size);

  /**
   * @brief Checks if the cured string similarly contains the specified UTF-16 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "funny"
   *   uint16_t funny[] = { 0x66, 0x75, 0x6e, 0x6e, 0x79 };
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_contains_utf16(cured, funny, sizeof(funny) / sizeof(uint16_t)), "decancer_contains_utf16");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 encoded string to match with.
   * @param other_length Length of the UTF-16 encoded string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @see decancer_contains
   * @return bool true if the cured string similarly contains the specified string, false otherwise.
   * @note For its UTF-8 counterpart, see decancer_contains.
   * @since 3.2.2
   */
  DECANCER_EXPORT bool decancer_contains_utf16(decancer_cured_t cured, const uint16_t* other_str, const size_t other_length);

  /**
   * @brief Checks if the cured string similarly starts with the specified UTF-8 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_starts_with(cured, "very", 4), "decancer_starts_with");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 encoded string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_starts_with_utf16
   * @return bool true if the cured string similarly starts with the specified string, false otherwise.
   * @note For its UTF-16 counterpart, see decancer_starts_with_utf16.
   * @since 1.5.3
   */
  DECANCER_EXPORT bool decancer_starts_with(decancer_cured_t cured, const uint8_t* other_str, const size_t other_size);

  /**
   * @brief Checks if the cured string similarly starts with the specified UTF-16 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "very"
   *   uint16_t very[] = { 0x76, 0x65, 0x72, 0x79 };
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_starts_with_utf16(cured, very, sizeof(very) / sizeof(uint16_t)), "decancer_starts_with_utf16");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 encoded string to match with.
   * @param other_length Length of the UTF-16 encoded string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @see decancer_starts_with
   * @return bool true if the cured string similarly starts with the specified string, false otherwise.
   * @note For its UTF-8 counterpart, see decancer_starts_with.
   * @since 3.2.2
   */
  DECANCER_EXPORT bool decancer_starts_with_utf16(decancer_cured_t cured, const uint16_t* other_str, const size_t other_length);

  /**
   * @brief Checks if the cured string similarly ends with the specified UTF-8 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_ends_with(cured, "text", 4), "decancer_ends_with");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 encoded string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_ends_with_utf16
   * @return bool true if the cured string similarly ends with the specified string, false otherwise.
   * @note For its UTF-16 counterpart, see decancer_ends_with_utf16.
   * @since 1.5.3
   */
  DECANCER_EXPORT bool decancer_ends_with(decancer_cured_t cured, const uint8_t* other_str, const size_t other_size);

  /**
   * @brief Checks if the cured string similarly ends with the specified UTF-16 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "text"
   *   uint16_t text[] = { 0x74, 0x65, 0x78, 0x74 };
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_ends_with_utf16(cured, text, sizeof(text) / sizeof(uint16_t)), "decancer_ends_with_utf16");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 encoded string to match with.
   * @param other_length Length of the UTF-16 encoded string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @see decancer_ends_with
   * @return bool true if the cured string similarly ends with the specified string, false otherwise.
   * @note For its UTF-8 counterpart, see decancer_ends_with.
   * @since 3.2.2
   */
  DECANCER_EXPORT bool decancer_ends_with_utf16(decancer_cured_t cured, const uint16_t* other_str, const size_t other_length);

  /**
   * @brief Checks if the cured string is similar with the specified UTF-8 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_equals(cured, "very funny text", 15), "decancer_equals");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-8 encoded string to match with.
   * @param other_size UTF-8 size of the other string, in bytes.
   * @see decancer_equals_utf16
   * @return bool true if the cured string is similar with the specified string, false otherwise.
   * @note For its UTF-16 counterpart, see decancer_equals_utf16.
   * @since 1.5.3
   */
  DECANCER_EXPORT bool decancer_equals(decancer_cured_t cured, const uint8_t* other_str, const size_t other_size);

  /**
   * @brief Checks if the cured string is similar with the specified UTF-16 encoded string.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes)                           \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto END;                                                  \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "very funny text"
   *   uint16_t expected_contents[] = { 0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e, 0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74 };
   *
   *   decancer_error_t error;
   *   decancer_cured_t cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   decancer_assert(decancer_equals_utf16(cured, expected_contents, sizeof(expected_contents) / sizeof(uint16_t)), "decancer_equals_utf16");
   *
   * END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param cured The cured string object.
   * @param other_str The UTF-16 encoded string to match with.
   * @param other_length Length of the UTF-16 encoded string in units of uint16_t -- or sizeof(string) / sizeof(uint16_t).
   * @see decancer_equals
   * @return bool true if the cured string is similar with the specified string, false otherwise.
   * @note For its UTF-8 counterpart, see decancer_equals.
   * @since 3.2.2
   */
  DECANCER_EXPORT bool decancer_equals_utf16(decancer_cured_t cured, const uint16_t* other_str, const size_t other_length);

  /**
   * @brief Clones the rust object created by decancer_cured_raw_utf16.
   *
   * @param raw_utf16_handle The rust object to clone.
   * @return decancer_cured_raw_utf16_t The newly cloned rust object.
   * @see decancer_cured_raw_utf16
   * @see decancer_cured_raw_utf16_free
   * @note You are responsible in freeing the returned object later by calling decancer_cured_raw_utf16_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT decancer_cured_raw_utf16_t decancer_cured_raw_utf16_clone(decancer_cured_raw_utf16_t raw_utf16_handle);

  /**
   * @brief Frees the rust object created by decancer_cured_raw_utf16.
   *
   * @param raw_utf16_handle The rust object to free.
   * @see decancer_cured_raw_utf16
   * @see decancer_cured_raw_utf16_clone
   * @since 3.2.2
   */
  DECANCER_EXPORT void decancer_cured_raw_utf16_free(decancer_cured_raw_utf16_t raw_utf16_handle);

  /**
   * @brief Consumes the UTF-8 matcher iterator object created by decancer_find and returns a matches object.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
   *                      0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
   *                      0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   decancer_matcher_t matcher;
   *   decancer_matches_t matches;
   *   const decancer_match_t* raw_matches;
   *   size_t raw_matches_length;
   *
   *   cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   matcher = decancer_find(cured, "funny", 5);
   *   decancer_assert(matcher != NULL, "decancer_find", CURED_END);
   *
   *   matches = decancer_matcher_consume(matcher);
   *
   *   raw_matches = decancer_matches_raw(matches, &raw_matches_length);
   *
   *   decancer_assert(raw_matches_length == 1, "raw_matches_length", MATCHES_END);
   *
   *   decancer_assert(raw_matches[0].start == 5, "start of funny", MATCHES_END);
   *   decancer_assert(raw_matches[0].end == 10, "end of funny", MATCHES_END);
   *
   * MATCHES_END:
   *   decancer_matches_free(matches);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param matcher The UTF-8 matcher iterator object to consume.
   * @return decancer_matches_t A new matches object created from the UTF-8 matcher iterator object.
   * @see decancer_find
   * @see decancer_matcher_next
   * @see decancer_matcher_free
   * @see decancer_matcher_utf16_consume
   * @note For its UTF-16 counterpart, see decancer_matcher_utf16_consume.
   * @note You don't need to pass the UTF-8 matcher iterator object to decancer_matcher_free after calling this.
   * @note You are responsible in freeing the returned object later by calling decancer_matches_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT decancer_matches_t decancer_matcher_consume(decancer_matcher_t matcher);

  /**
   * @brief Frees the UTF-8 matcher iterator object created by decancer_find.
   *
   * @param matcher The UTF-8 matcher iterator object to free.
   * @see decancer_find
   * @see decancer_matcher_consume
   * @see decancer_matcher_next
   * @see decancer_matcher_utf16_free
   * @since 3.1.0
   */
  DECANCER_EXPORT void decancer_matcher_free(decancer_matcher_t matcher);

  /**
   * @brief Consumes the UTF-16 matcher iterator object created by decancer_find_utf16 and returns a matches object.
   *
   * Example:
   *
   * ```c
   * #include <decancer.h>
   *
   * #include <string.h>
   * #include <stdlib.h>
   * #include <stdio.h>
   *
   * #define decancer_assert(expr, notes, label)                    \
   *   if (!(expr)) {                                               \
   *     fprintf(stderr, "assertion failure at " notes "\n");       \
   *     ret = 1;                                                   \
   *     goto label;                                                \
   *   }
   *
   * int main(void) {
   *   int ret = 0;
   *
   *   // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
   *   uint16_t input[] = {
   *     0x0076, 0xff25, 0x24e1,
   *     0xd835, 0xdd02, 0x0020,
   *     0xd835, 0xdd3d, 0xd835,
   *     0xdd4c, 0x0147, 0x2115,
   *     0xff59, 0x0020, 0x0163,
   *     0x4e47, 0xd835, 0xdd4f,
   *     0xd835, 0xdce3
   *   };
   *
   *   // UTF-16 bytes for "funny"
   *   uint16_t funny[] = { 0x66, 0x75, 0x6e, 0x6e, 0x79 };
   *
   *   decancer_cured_t cured;
   *   decancer_error_t error;
   *   decancer_matcher_utf16_t matcher;
   *   decancer_matches_t matches;
   *   const decancer_match_t* raw_matches;
   *   size_t raw_matches_length;
   *
   *   cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);
   *
   *   if (cured == NULL) {
   *     fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
   *     return 1;
   *   }
   *
   *   matcher = decancer_find_utf16(cured, funny, sizeof(funny) / sizeof(uint16_t));
   *   decancer_assert(matcher != NULL, "decancer_find_utf16", CURED_END);
   *
   *   matches = decancer_matcher_utf16_consume(matcher);
   *
   *   raw_matches = decancer_matches_raw(matches, &raw_matches_length);
   *
   *   decancer_assert(raw_matches_length == 1, "raw_matches_length", MATCHES_END);
   *
   *   decancer_assert(raw_matches[0].start == 5, "start of funny", MATCHES_END);
   *   decancer_assert(raw_matches[0].end == 10, "end of funny", MATCHES_END);
   *
   * MATCHES_END:
   *   decancer_matches_free(matches);
   * CURED_END:
   *   decancer_cured_free(cured);
   *   return ret;
   * }
   * ```
   *
   * @param matcher The UTF-16 matcher iterator object to consume.
   * @return decancer_matches_t A new matches object created from the UTF-16 matcher iterator object.
   * @see decancer_find_utf16
   * @see decancer_matcher_consume
   * @see decancer_matcher_utf16_next
   * @see decancer_matcher_utf16_free
   * @note For its UTF-8 counterpart, see decancer_matcher_consume.
   * @note You don't need to pass the UTF-16 matcher iterator object to decancer_matcher_utf16_free after calling this.
   * @note You are responsible in freeing the returned object later by calling decancer_matches_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT decancer_matches_t decancer_matcher_utf16_consume(decancer_matcher_utf16_t matcher);

  /**
   * @brief Frees the UTF-16 matcher iterator object created by decancer_find_utf16.
   *
   * @param matcher The UTF-16 matcher iterator object to free.
   * @see decancer_find_utf16
   * @see decancer_matcher_free
   * @see decancer_matcher_utf16_next
   * @since 3.2.2
   */
  DECANCER_EXPORT void decancer_matcher_utf16_free(decancer_matcher_utf16_t matcher);

  /**
   * @brief Clones the matches object created by decancer_find_multiple and decancer_find_multiple_utf16.
   *
   * @param matches The matches object to clone.
   * @return decancer_matches_t The newly cloned matches object.
   * @see decancer_find_multiple
   * @see decancer_find_multiple_utf16
   * @see decancer_matcher_consume
   * @see decancer_matcher_utf16_consume
   * @see decancer_matches_raw
   * @see decancer_matches_free
   * @note You are responsible in freeing the returned object later by calling decancer_matches_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT decancer_matches_t decancer_matches_clone(decancer_matches_t matches);

  /**
   * @brief Frees the matches object created by decancer_find_multiple and decancer_find_multiple_utf16.
   *
   * @param matches The matches object to free.
   * @see decancer_find_multiple
   * @see decancer_find_multiple_utf16
   * @see decancer_matcher_consume
   * @see decancer_matcher_utf16_consume
   * @see decancer_matches_clone
   * @see decancer_matches_raw
   * @since 3.1.1
   */
  DECANCER_EXPORT void decancer_matches_free(decancer_matches_t matches);

  /**
   * @brief Initiates a newly created translation struct for use.
   *
   * @param translation A pointer to a translation struct bound for decancer_cure_char.
   * @see decancer_cure_char
   * @see decancer_translation_clone
   * @see decancer_translation_free
   * @note This function MUST be called before any calls to decancer_cure_char.
   * @since 3.2.2
   */
  DECANCER_EXPORT void decancer_translation_init(decancer_translation_t* translation);

  /**
   * @brief Clones the translation struct used in decancer_cure_char.
   *
   * @param translation_in A pointer to a translation struct to clone.
   * @param translation_out A pointer to the newly cloned translation struct. This struct doesn't have to be initiated with decancer_translation_init first.
   * @see decancer_cure_char
   * @see decancer_translation_init
   * @see decancer_translation_free
   * @note You are responsible in freeing the translation struct later by passing it as a pointer to decancer_translation_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT void decancer_translation_clone(const decancer_translation_t* translation_in, decancer_translation_t* translation_out);

  /**
   * @brief Frees the translation struct used in decancer_cure_char.
   *
   * @param translation A pointer to a translation struct.
   * @see decancer_cure_char
   * @see decancer_translation_init
   * @see decancer_translation_clone
   * @since 3.1.0
   */
  DECANCER_EXPORT void decancer_translation_free(decancer_translation_t* translation);

  /**
   * @brief Clones the cured string object created by decancer_cure and decancer_cure_utf16.
   *
   * @param cured The cured string object to clone.
   * @return decancer_cured_t The newly cloned cured string object.
   * @see decancer_cure
   * @see decancer_cure_utf16
   * @see decancer_cured_free
   * @note You are responsible in freeing the returned object later by calling decancer_cured_free.
   * @since 3.2.2
   */
  DECANCER_EXPORT decancer_cured_t decancer_cured_clone(decancer_cured_t cured);

  /**
   * @brief Frees the cured string object created by decancer_cure and decancer_cure_utf16.
   *
   * @param cured The cured string object to free.
   * @see decancer_cure
   * @see decancer_cure_utf16
   * @see decancer_cured_clone
   * @since 3.1.0
   */
  DECANCER_EXPORT void decancer_cured_free(decancer_cured_t cured);

#undef DECANCER_EXPORT

#ifdef __cplusplus
} // extern "C"
#endif
#endif
