#include "decancer.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef __clang__
#pragma clang diagnostic ignored "-Wwritable-strings"
#endif

decancer_cured_t cured = NULL;
decancer_matcher_t matcher = NULL;
decancer_translation_t char_translation;

static void assert(const bool expr, const char *message)
{
    if (!expr)
    {
        fprintf(stderr, "assertion failed (%s)\n", message);

        decancer_translation_free(&char_translation);

        if (matcher != NULL)
        {
            decancer_matcher_free(matcher);
        }

        if (cured != NULL)
        {
            decancer_free(cured);
        }

        exit(1);
    }
}

static void print_error(decancer_error_t error_code)
{
    char message[90];
    uint8_t message_size;

    const uint8_t *ptr = decancer_error(error_code, &message_size);
    memcpy(message, ptr, message_size);

    message[message_size] = '\0';

    fprintf(stderr, "error: %s", message);
}

int main(void)
{
    decancer_error_t error_code;
    memset(&char_translation, 0, sizeof(decancer_translation_t));

    decancer_cure_char(0xFF25, DECANCER_OPTION_DEFAULT, &char_translation);

    assert(char_translation.kind == DECANCER_TRANSLATION_KIND_CHARACTER, "char translation is a character");
    assert(char_translation.contents.character == 0x65, "char translation is 'e' (0x65)");

    decancer_cure_char(0x04D5, DECANCER_OPTION_DEFAULT, &char_translation);

    assert(char_translation.kind == DECANCER_TRANSLATION_KIND_STRING, "char translation is an ASCII string");
    assert(char_translation.contents.string.length == 2,
           "char translation is an ASCII string with the length of 2 bytes");
    assert(char_translation.contents.string.contents[0] == 'a' && char_translation.contents.string.contents[1] == 'e',
           "char translation is the ASCII string \"ae\".");

    decancer_cure_char(0, DECANCER_OPTION_DEFAULT, &char_translation);

    assert(char_translation.kind == DECANCER_TRANSLATION_KIND_NONE, "char translation is an empty string ('')");

    uint8_t string[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
                        0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
                        0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};

    cured = decancer_cure(string, sizeof(string), DECANCER_OPTION_DEFAULT, &error_code);

    if (cured == NULL)
    {
        print_error(error_code);
        return 1;
    }

    assert(decancer_equals(cured, (uint8_t *)("very funny text"), 15), "equals failed");
    assert(decancer_starts_with(cured, (uint8_t *)("very"), 4), "starts_with failed");
    assert(decancer_ends_with(cured, (uint8_t *)("text"), 4), "ends_with failed");
    assert(decancer_contains(cured, (uint8_t *)("funny"), 5), "contains failed");

    matcher = decancer_find(cured, (uint8_t *)("funny"), 5);
    assert(matcher != NULL, "find returned NULL");

    decancer_match_t match;

    assert(decancer_matcher_next(matcher, &match) != 0, "match_next failed");
    assert(match.start == 5, "match.start is not 5");
    assert(match.end == 10, "match.end is not 10");

    size_t output_size;
    const uint8_t *output_raw = decancer_raw(cured, &output_size);

    assert(output_size == 15, "raw output size");

    const uint8_t expected_raw[] = {0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e,
                                    0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74};

    char assert_message[38];
    for (uint32_t i = 0; i < sizeof(expected_raw); i++)
    {
        sprintf(assert_message, "mismatched utf-8 contents at index %u", i);
        assert(output_raw[i] == expected_raw[i], assert_message);
    }

    decancer_free(cured);
    puts("ok");

    return 0;
}