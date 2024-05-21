#include <decancer.hpp>
#include <iostream>

#ifdef _MSC_VER
#pragma warning(disable: 4838)
#endif

#define assert(expr, notes)                                    \
  if (!(expr)) {                                               \
    std::cerr << "assertion failure at " notes << std::endl;   \
    goto UTF8_TEST_END;                                        \
  }

#define wassert(expr, notes)                                        \
  if (!(expr)) {                                                    \
    std::cerr << "wide assertion failure at " notes << std::endl;   \
    goto UTF16_TEST_END;                                            \
  }

int main() {
  const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};

  decancer::cured_string cured_utf8{very_funny_text};
  std::vector<decancer::match_t> matches{};
  decancer::match_t first_match;

  assert(cured_utf8 == "very funny text", "equals");
  assert(cured_utf8.starts_with("very"), "starts_with");
  assert(cured_utf8.contains("funny"), "contains");
  assert(cured_utf8.ends_with("text"), "ends_with");

  matches = cured_utf8.find("funny");
  assert(matches.size() == 1, "matches size");

  first_match = matches.at(0);
  assert(first_match.start == 5, "match start");
  assert(first_match.end == 10, "match end");

  cured_utf8.censor("funny", '*');
  assert(cured_utf8 == "very ***** text", "censored equals");

UTF8_TEST_END:
  const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147, 0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
  decancer::cured_string cured_utf16{wide_very_funny_text};

  wassert(cured_utf16 == L"very funny text", "equals");
  wassert(cured_utf16.starts_with(L"very"), "starts_with");
  wassert(cured_utf16.contains(L"funny"), "contains");
  wassert(cured_utf16.ends_with(L"text"), "ends_with");

  matches = cured_utf16.find(L"funny");
  wassert(matches.size() == 1, "matches size");

  first_match = matches.at(0);
  wassert(first_match.start == 5, "match start");
  wassert(first_match.end == 10, "match end");

  cured_utf16.censor(L"funny", L'*');
  wassert(cured_utf16 == L"very ***** text", "censored equals");
UTF16_TEST_END:

  return 0;
}