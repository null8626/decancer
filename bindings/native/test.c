#include "decancer.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef __clang__
#pragma clang diagnostic ignored "-Wwritable-strings"
#endif

decancer_cured_t cured = NULL;
decancer_raw_wide_t wide = NULL;
decancer_matcher_t matcher = NULL;
decancer_translation_t char_translation;

static void assert(const bool expr, const char *message, const char *prefix)
{
    if (!expr)
    {
        fprintf(stderr, "%sassertion failed (%s)\n", prefix, message);

        decancer_translation_free(&char_translation);

        if (wide != NULL)
        {
            decancer_raw_wide_free(wide);
        }

        if (matcher != NULL)
        {
            decancer_matcher_free(matcher);
        }

        if (cured != NULL)
        {
            decancer_cured_free(cured);
        }

        exit(1);
    }
}

static void print_error(decancer_error_t *error, const char *prefix)
{
    char message[90];
    uint8_t message_size;

    memcpy(message, error->message, error->message_size);

    message[error->message_size] = '\0';

    fprintf(stderr, "%serror: %s", prefix, message);
}

static bool test_utf8(uint8_t *string, size_t size, const char *error_prefix, decancer_error_t *error)
{
    cured = decancer_cure(string, size, DECANCER_OPTION_DEFAULT, error);

    if (cured == NULL)
    {
        print_error(error, error_prefix);
        return false;
    }

    assert(decancer_equals(cured, (uint8_t *)("very funny text"), 15), "equals failed", error_prefix);
    assert(decancer_starts_with(cured, (uint8_t *)("very"), 4), "starts_with failed", error_prefix);
    assert(decancer_ends_with(cured, (uint8_t *)("text"), 4), "ends_with failed", error_prefix);
    assert(decancer_contains(cured, (uint8_t *)("funny"), 5), "contains failed", error_prefix);

    matcher = decancer_find(cured, (uint8_t *)("funny"), 5);
    assert(matcher != NULL, "find returned NULL", error_prefix);

    decancer_match_t match;

    assert(decancer_matcher_next(matcher, &match) != 0, "match_next failed", error_prefix);
    assert(match.start == 5, "match.start is not 5", error_prefix);
    assert(match.end == 10, "match.end is not 10", error_prefix);

    size_t output_size;
    const uint8_t *output_raw = decancer_raw(cured, &output_size);

    assert(output_size == 15, "raw output size", error_prefix);

    const uint8_t expected_raw[] = {0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e,
                                    0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74};

    char assert_message[60];
    for (uint32_t i = 0; i < sizeof(expected_raw); i++)
    {
        sprintf(assert_message, "mismatched utf-8 contents at index %u", i);
        assert(output_raw[i] == expected_raw[i], assert_message, error_prefix);
    }

    decancer_cured_free(cured);
    return true;
}

static bool test_utf16(uint16_t *string, size_t size, const char *error_prefix, decancer_error_t *error)
{
    cured = decancer_cure_wide(string, size, DECANCER_OPTION_DEFAULT, error);

    if (cured == NULL)
    {
        print_error(error, error_prefix);
        return false;
    }

    assert(decancer_equals(cured, (uint8_t *)("very funny text"), 15), "equals", error_prefix);
    assert(decancer_contains(cured, (uint8_t *)("funny"), 5), "contains", error_prefix);

    uint16_t *utf16_output_ptr;
    size_t utf16_output_size;
    wide = decancer_raw_wide(cured, &utf16_output_ptr, &utf16_output_size);

    assert(utf16_output_size == (15 * sizeof(uint16_t)), "raw output size", error_prefix);

    const uint16_t expected_utf16_raw[] = {0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e,
                                           0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74};
    char assert_message[60];

    for (uint32_t i = 0; i < sizeof(expected_utf16_raw) / sizeof(uint16_t); i++)
    {
        sprintf(assert_message, "mismatched utf-16 contents at index %u", i);
        assert(utf16_output_ptr[i] == expected_utf16_raw[i], assert_message, error_prefix);
    }

    decancer_raw_wide_free(wide);
    decancer_cured_free(cured);
    return true;
}

int main(void)
{
    decancer_error_t error;
    memset(&char_translation, 0, sizeof(decancer_translation_t));

    decancer_cure_char(0xFF25, DECANCER_OPTION_DEFAULT, &char_translation);

    assert(char_translation.kind == DECANCER_TRANSLATION_KIND_CHARACTER, "char translation is a character", "");
    assert(char_translation.contents.character == 0x65, "char translation is 'e' (0x65)", "");

    decancer_cure_char(0x04D5, DECANCER_OPTION_DEFAULT, &char_translation);

    assert(char_translation.kind == DECANCER_TRANSLATION_KIND_STRING, "char translation is an ASCII string", "");
    assert(char_translation.contents.string.length == 2,
           "char translation is an ASCII string with the length of 2 bytes", "");
    assert(char_translation.contents.string.contents[0] == 'a' && char_translation.contents.string.contents[1] == 'e',
           "char translation is the ASCII string \"ae\".", "");

    decancer_cure_char(0, DECANCER_OPTION_DEFAULT, &char_translation);

    assert(char_translation.kind == DECANCER_TRANSLATION_KIND_NONE, "char translation is an empty string ('')", "");

    uint8_t string[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
                        0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
                        0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};

    if (!test_utf8(string, sizeof(string) - sizeof(uint8_t), "", &error) ||
        !test_utf8(string, 0, "null-terminated ", &error))
    {
        return 1;
    }

    uint16_t utf16_string[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
                               0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};

    if (!test_utf16(utf16_string, sizeof(utf16_string) - sizeof(uint16_t), "utf-16 ", &error) ||
        !test_utf16(utf16_string, 0, "utf-16 null-terminated ", &error))
    {
        return 1;
    }

    puts("ok");

    return 0;
}