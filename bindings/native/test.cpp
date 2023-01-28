#include "decancer.h"

#ifdef __clang__
#pragma clang diagnostic ignored "-Wwritable-strings"
#endif

#include <cstdio>
#include <cstdlib>

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
    uint8_t string[] = u8"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";

    for (uint32_t i = 0; i < (sizeof(string) - sizeof(uint8_t)); i++) {
      printf("%x ", string[i]);
    }
    
    putchar('\n');

    cured = decancer_cure(string, sizeof(string) - sizeof(uint8_t));

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
    wchar_t string[] = L"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";

    cured = wdecancer_cure(string, (sizeof(string) - sizeof(wchar_t)) / sizeof(wchar_t));

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

    return 0;
}