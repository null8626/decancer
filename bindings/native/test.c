#include "decancer.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef __clang__
#pragma clang diagnostic ignored "-Wwritable-strings"
#endif

decancer_cured_t cured;
wdecancer_raw_cured_t output_raw = NULL;

static void assert(const bool expr, const char *message)
{
    if (!expr)
    {
        fprintf(stderr, "assertion failed (%s)\n", message);

        if (output_raw != NULL)
        {
            wdecancer_raw_free(output_raw);
            output_raw = NULL;
        }

        decancer_free(cured);
        exit(1);
    }
}

static inline void test_utf8(void)
{
    // utf-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
    uint8_t string[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
                        0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
                        0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};

    cured = decancer_cure(string, sizeof(string));

    assert(decancer_equals(cured, (uint8_t *)("very funny text"), 15), "equals");
    assert(decancer_starts_with(cured, (uint8_t *)("very"), 4), "starts_with");
    assert(decancer_ends_with(cured, (uint8_t *)("text"), 4), "ends_with");
    assert(decancer_contains(cured, (uint8_t *)("funny"), 5), "contains");

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
}

static inline void test_utf16(void)
{
    // utf-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
    wchar_t string[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c,
                        0x0147, 0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3};

    cured = wdecancer_cure(string, sizeof(string) / sizeof(wchar_t));

    assert(wdecancer_equals(cured, L"very funny text", 15), "wide equals");
    assert(wdecancer_starts_with(cured, L"very", 4), "wide starts_with");
    assert(wdecancer_ends_with(cured, L"text", 4), "wide ends_with");
    assert(wdecancer_contains(cured, L"funny", 5), "wide contains");

    size_t output_length;
    output_raw = wdecancer_raw(cured, &output_length);
    const wchar_t *output_raw_ptr = wdecancer_raw_ptr(output_raw);

    assert(output_length == 15, "wide raw output length");

    const wchar_t expected_raw[] = {0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e,
                                    0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74};

    char assert_message[39];
    for (uint32_t i = 0; i < sizeof(expected_raw) / sizeof(wchar_t); i++)
    {
        sprintf(assert_message, "mismatched utf-16 contents at index %u", i);
        assert(output_raw_ptr[i] == expected_raw[i], assert_message);
    }

    wdecancer_raw_free(output_raw);
    decancer_free(cured);
}

int main(void)
{
    test_utf8();
    test_utf16();
    puts("ok");

    return 0;
}