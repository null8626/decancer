#include <decancer.h>

#include <retain_data.h>

#include <stdbool.h>
#include <stdio.h>

#define decancer_assert(expr, notes)                     \
  if (!(expr)) {                                         \
    fprintf(stderr, "assertion failure at " notes "\n"); \
    ret = 1;                                             \
    goto END;                                            \
  }

#define _decancer_checked_cure(input, input_size, options, notes)                             \
  cured = decancer_cure(input, input_size, options, &error);                                  \
                                                                                              \
  if (cured == NULL) {                                                                        \
    fprintf(stderr, notes " curing error: %.*s\n", (int)error.message_length, error.message); \
    return 1;                                                                                 \
  }

#define decancer_checked_cure(input, options, notes) \
  _decancer_checked_cure(input, sizeof(input), options, notes)

int extra_tests() {
  int ret = 0;

  uint8_t retain_capitalization_input[] = { 0x64, 0x65, 0x63, 0xc3, 0x81, 0x6e, 0x63, 0x65, 0x72 };
  uint8_t retain_capitalization_expected_output[] = { 0x64, 0x65, 0x63, 0x41, 0x6e, 0x63, 0x65, 0x72 };

  uint8_t disable_leetspeak_input[] = { 0x7c, 0x2d, 0x7c, 0x33, 0x7c, 0x5f, 0x49, 0x5f, 0x30 };
  uint8_t disable_leetspeak_expected_non_match[] = { 0x68, 0x65, 0x6c, 0x6c, 0x6f };
  uint8_t disable_alphabetical_leetspeak_expected_match[] = { 0x68, 0x65, 0x6c, 0x49, 0x5f, 0x6f };

  decancer_error_t error;
  decancer_cured_t cured;
  
  decancer_checked_cure(retain_capitalization_input, DECANCER_OPTION_RETAIN_CAPITALIZATION, "retain capitalization");

  size_t raw_contents_size;
  const uint8_t* raw_contents = decancer_cured_raw(cured, NULL, &raw_contents_size);

  decancer_assert(raw_contents_size == sizeof(retain_capitalization_expected_output), "retain capitalization output length");

  for (size_t i = 0; i < raw_contents_size; i++) {
    decancer_assert(raw_contents[i] == retain_capitalization_expected_output[i], "retain capitalization output contents");
  }

  decancer_cured_free(cured);
  cured = NULL;

  decancer_checked_cure(disable_leetspeak_input, DECANCER_OPTION_DISABLE_LEETSPEAK, "disable leetspeak");

  decancer_assert(!decancer_equals(cured, disable_leetspeak_expected_non_match, sizeof(disable_leetspeak_expected_non_match)), "disable leetspeak option");

  decancer_disable_leetspeak(cured, false);
  decancer_disable_alphabetical_leetspeak(cured, true);

  decancer_assert(decancer_equals(cured, disable_alphabetical_leetspeak_expected_match, sizeof(disable_alphabetical_leetspeak_expected_match)), "disable alphabetical leetspeak method");

  decancer_cured_free(cured);
  cured = NULL;

  decancer_checked_cure(disable_leetspeak_input, DECANCER_OPTION_DISABLE_ALPHABETICAL_LEETSPEAK, "disable alphabetical leetspeak");

  decancer_assert(decancer_equals(cured, disable_alphabetical_leetspeak_expected_match, sizeof(disable_alphabetical_leetspeak_expected_match)), "disable alphabetical leetspeak option");

  decancer_disable_leetspeak(cured, true);
  decancer_disable_alphabetical_leetspeak(cured, false);

  decancer_assert(!decancer_equals(cured, disable_leetspeak_expected_non_match, sizeof(disable_leetspeak_expected_non_match)), "disable leetspeak method");

  decancer_cured_free(cured);
  cured = NULL;
  
  for (size_t i = 0; i < sizeof(g_retain_data) / sizeof(decancer_retain_data_t); i++) {
    const decancer_retain_data_t* data = g_retain_data[i];

    _decancer_checked_cure(data->input, data->input_size, DECANCER_OPTION_DISABLE_BIDI | data->options, "retain data");

    decancer_assert(decancer_equals(cured, data->input, data->input_size), "retain data equality");

    decancer_cured_free(cured);
    cured = NULL;

    _decancer_checked_cure(data->input, data->input_size, DECANCER_OPTION_DEFAULT, "retain data (placebo)");

    decancer_assert(!decancer_equals(cured, data->input, data->input_size), "retain data (placebo) inequality");

    decancer_cured_free(cured);
    cured = NULL;
  }

END:
  if (cured != NULL) {
    decancer_cured_free(cured);
  }

  return ret;
}