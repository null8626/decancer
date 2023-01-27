#include "decancer.h"

#ifdef __clang__
#pragma clang diagnostic ignored "-Wwritable-strings"
#endif

#include <cstdio>
#include <cstdlib>

#define DECANCER_TEST(name, scope)                                                                                     \
    printf("Running tests for " name "... ");                                                                          \
    scope;                                                                                                             \
    puts("ok")

decancer_cured_t cured;
wdecancer_raw_cured_t output_raw = NULL;

#define assert_message(expr, message) _assert(expr, message)
#define assert(expr) _assert(expr, NULL)

static void _assert(const bool expr, const char *message)
{
    if (!expr)
    {
        if (message == NULL)
        {
            puts("failed");
        }
        else
        {
            printf("failed (%s)\n", message);
        }

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

    cured = decancer_cure(string, sizeof(string) - sizeof(uint8_t));

    DECANCER_TEST("decancer_equals", assert(decancer_equals(cured, (uint8_t *)("very funny text"), 15)));
    DECANCER_TEST("decancer_starts_with", assert(decancer_starts_with(cured, (uint8_t *)("very"), 4)));
    DECANCER_TEST("decancer_ends_with", assert(decancer_ends_with(cured, (uint8_t *)("text"), 4)));
    DECANCER_TEST("decancer_contains", assert(decancer_contains(cured, (uint8_t *)("funny"), 5)));

    size_t output_size;
    const uint8_t *output_raw = decancer_raw(cured, &output_size);

    DECANCER_TEST("decancer_raw output_size", assert(output_size == 15));

    const uint8_t expected_raw[] = {0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e,
                                    0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74};

    DECANCER_TEST("decancer_raw contents", {
        char message[38];
        for (uint32_t i = 0; i < sizeof(expected_raw); i++)
        {
            sprintf(message, "mismatched utf-8 contents at index %u", i);
            assert_message(output_raw[i] == expected_raw[i], message);
        }
    });

    decancer_free(cured);
}

static inline void test_utf16(void)
{
    wchar_t string[] = L"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";

    cured = wdecancer_cure(string, (sizeof(string) - sizeof(wchar_t)) / sizeof(wchar_t));

    DECANCER_TEST("wdecancer_equals", assert(wdecancer_equals(cured, L"very funny text", 15)));
    DECANCER_TEST("wdecancer_starts_with", assert(wdecancer_starts_with(cured, L"very", 4)));
    DECANCER_TEST("wdecancer_ends_with", assert(wdecancer_ends_with(cured, L"text", 4)));
    DECANCER_TEST("wdecancer_contains", assert(wdecancer_contains(cured, L"funny", 5)));

    size_t output_length;
    output_raw = wdecancer_raw(cured, &output_length);
    const wchar_t *output_raw_ptr = wdecancer_raw_ptr(output_raw);

    DECANCER_TEST("wdecancer_raw output_length", assert(output_length == 15));

    const wchar_t expected_raw[] = {0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e,
                                    0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74};

    DECANCER_TEST("wdecancer_raw contents", {
        char message[39];
        for (uint32_t i = 0; i < sizeof(expected_raw) / sizeof(wchar_t); i++)
        {
            sprintf(message, "mismatched utf-8 contents at index %u", i);
            assert_message(output_raw_ptr[i] == expected_raw[i], message);
        }
    });

    wdecancer_raw_free(output_raw);
    decancer_free(cured);
}

int main(void)
{
    test_utf8();
    test_utf16();

    return 0;
}