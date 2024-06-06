#include <decancer.hpp>
#include <cstring>

using namespace decancer;

#define DECANCER_STRING(text)              reinterpret_cast<const uint8_t*>(text)
#define DECANCER_WSTRING(text)             reinterpret_cast<const uint16_t*>(text)
#define DECANCER_INTO_ERROR(error_struct)  native_error(generate_error_message(&error_struct), error_struct.message_length + 1)

#define DECANCER_CTOR_IMPL(method_name, options, text_argument, length_argument, ...)                 \
  cured_string::cured_string(__VA_ARGS__) {                                                           \
    error_t err;                                                                                      \
    if ((m_ptr = decancer_##method_name(text_argument, length_argument, options, &err)) == nullptr) { \
      throw DECANCER_INTO_ERROR(err);                                                                 \
    }                                                                                                 \
  }

#define DECANCER_GENERATE_CTOR_IMPL(text_argument, length_argument, ...)                                                      \
  DECANCER_CTOR_IMPL(cure, DECANCER_OPTION_DEFAULT, DECANCER_STRING(text_argument), length_argument, __VA_ARGS__)             \
  DECANCER_CTOR_IMPL(cure, options, DECANCER_STRING(text_argument), length_argument, __VA_ARGS__, const options_t options)

#define DECANCER_GENERATE_WIDE_CTOR_IMPL(text_argument, length_argument, ...)                                                       \
  DECANCER_CTOR_IMPL(cure_wide, DECANCER_OPTION_DEFAULT, DECANCER_WSTRING(text_argument), length_argument, __VA_ARGS__)             \
  DECANCER_CTOR_IMPL(cure_wide, options, DECANCER_WSTRING(text_argument), length_argument, __VA_ARGS__, const options_t options)

#define DECANCER_COMPARISON_METHOD_IMPL(method_name, string_argument, length_argument, ...)   \
  bool cured_string::method_name(__VA_ARGS__) const noexcept {                                \
    return decancer_##method_name(m_ptr, DECANCER_STRING(string_argument), length_argument);  \
  }

#define DECANCER_COMPARISON_WIDE_METHOD_IMPL(method_name, string_argument, length_argument, ...)     \
  bool cured_string::method_name(__VA_ARGS__) const noexcept {                                       \
    return decancer_##method_name##_wide(m_ptr, DECANCER_WSTRING(string_argument), length_argument); \
  }

#define DECANCER_GENERATE_COMPARISON_METHODS_IMPL(method_name)                                              \
  DECANCER_COMPARISON_METHOD_IMPL(method_name, text, strlen(text), const char* text)                        \
  DECANCER_COMPARISON_METHOD_IMPL(method_name, text, length, const char* text, const size_t length)         \
  DECANCER_COMPARISON_METHOD_IMPL(method_name, text.data(), text.size(), const std::string& text)           \
  DECANCER_COMPARISON_WIDE_METHOD_IMPL(method_name, text, wcslen(text), const wchar_t* text)                \
  DECANCER_COMPARISON_WIDE_METHOD_IMPL(method_name, text, length, const wchar_t* text, const size_t length) \
  DECANCER_COMPARISON_WIDE_METHOD_IMPL(method_name, text.data(), text.size(), const std::wstring& text)

#define DECANCER_CENSOR_METHOD_IMPL(method_name, replacement_type_name, error_message, string_argument, length_argument, ...) \
  void cured_string::censor(__VA_ARGS__, const replacement_type_name replacement) const {                                     \
    if (!decancer_##method_name(m_ptr, string_argument, length_argument, static_cast<uint32_t>(replacement))) {               \
      throw error{error_message};                                                                                             \
    }                                                                                                                         \
  }

#define DECANCER_GENERATE_CENSOR_METHOD_IMPL(string_argument, length_argument, ...)                                                                                      \
  DECANCER_CENSOR_METHOD_IMPL(censor, char, "Invalid string input.", DECANCER_STRING(string_argument), length_argument, __VA_ARGS__)                                     \
  DECANCER_CENSOR_METHOD_IMPL(censor, wchar_t, "Invalid string input or censor replacement character.", DECANCER_STRING(string_argument), length_argument, __VA_ARGS__)  \
  DECANCER_CENSOR_METHOD_IMPL(censor, uint32_t, "Invalid string input or censor replacement character.", DECANCER_STRING(string_argument), length_argument, __VA_ARGS__)

#define DECANCER_GENERATE_CENSOR_WIDE_METHOD_IMPL(string_argument, length_argument, ...)                                                                                       \
  DECANCER_CENSOR_METHOD_IMPL(censor_wide, char, "Invalid string input.", DECANCER_WSTRING(string_argument), length_argument, __VA_ARGS__)                                     \
  DECANCER_CENSOR_METHOD_IMPL(censor_wide, wchar_t, "Invalid string input or censor replacement character.", DECANCER_WSTRING(string_argument), length_argument, __VA_ARGS__)  \
  DECANCER_CENSOR_METHOD_IMPL(censor_wide, uint32_t, "Invalid string input or censor replacement character.", DECANCER_WSTRING(string_argument), length_argument, __VA_ARGS__)

#define DECANCER_CENSOR_MULTIPLE_METHOD_IMPL(method_name, type_name)                                                                 \
  void cured_string::censor_multiple(const std::initializer_list<type_name>& keywords, const char replacement) const {               \
    auto keywords_in = keywords_from_list(keywords);                                                                                 \
    const auto result = decancer_##method_name(m_ptr, keywords_in, keywords.size(), static_cast<uint32_t>(replacement));             \
    delete[] keywords_in;                                                                                                            \
    if (!result) {                                                                                                                   \
      throw error{"Invalid string input."};                                                                                          \
    }                                                                                                                                \
  }                                                                                                                                  \
  void cured_string::censor_multiple(const std::initializer_list<type_name>& keywords, const wchar_t replacement) const {            \
    auto keywords_in = keywords_from_list(keywords);                                                                                 \
    const auto result = decancer_##method_name(m_ptr, keywords_in, keywords.size(), static_cast<uint32_t>(replacement));             \
    delete[] keywords_in;                                                                                                            \
    if (!result) {                                                                                                                   \
      throw error{"Invalid string input or censor replacement character."};                                                          \
    }                                                                                                                                \
  }                                                                                                                                  \
  void cured_string::censor_multiple(const std::initializer_list<type_name>& keywords, const uint32_t replacement) const {           \
    auto keywords_in = keywords_from_list(keywords);                                                                                 \
    const auto result = decancer_##method_name(m_ptr, keywords_in, keywords.size(), replacement);                                    \
    delete[] keywords_in;                                                                                                            \
    if (!result) {                                                                                                                   \
      throw error{"Invalid string input or censor replacement character."};                                                          \
    }                                                                                                                                \
  }

#define DECANCER_REPLACE_METHOD_IMPL(first_string_argument, first_length_argument, second_string_argument, second_length_argument, ...)                              \
  void cured_string::replace(__VA_ARGS__) const {                                                                                                                    \
    if (!decancer_replace(m_ptr, DECANCER_STRING(first_string_argument), first_length_argument, DECANCER_STRING(second_string_argument), second_length_argument)) {  \
      throw error{"Invalid replacement string input."};                                                                                                              \
    }                                                                                                                                                                \
  }

#define DECANCER_REPLACE_WIDE_METHOD_IMPL(first_string_argument, first_length_argument, second_string_argument, second_length_argument, ...)                                \
  void cured_string::replace(__VA_ARGS__) const {                                                                                                                           \
    if (!decancer_replace_wide(m_ptr, DECANCER_WSTRING(first_string_argument), first_length_argument, DECANCER_WSTRING(second_string_argument), second_length_argument)) {  \
      throw error{"Invalid replacement string input."};                                                                                                                     \
    }                                                                                                                                                                       \
  }

#define DECANCER_REPLACE_MULTIPLE_METHOD_IMPL(method_name, type_name, string_argument, length_argument, ...)              \
  void cured_string::replace_multiple(const std::initializer_list<type_name>& keywords, __VA_ARGS__) const {              \
    auto keywords_in = keywords_from_list(keywords);                                                                      \
    const auto result = decancer_##method_name(m_ptr, keywords_in, keywords.size(), string_argument, length_argument);    \
    delete[] keywords_in;                                                                                                 \
                                                                                                                          \
    if (!result) {                                                                                                        \
      throw error{"Invalid replacement string(s)."};                                                                      \
    }                                                                                                                     \
  }

#define DECANCER_GENERATE_REPLACE_MULTIPLE_METHODS_IMPL(string_argument, length_argument, ...)                                             \
  DECANCER_REPLACE_MULTIPLE_METHOD_IMPL(replace_multiple, const char*, DECANCER_STRING(string_argument), length_argument, __VA_ARGS__)     \
  DECANCER_REPLACE_MULTIPLE_METHOD_IMPL(replace_multiple, std::string, DECANCER_STRING(string_argument), length_argument, __VA_ARGS__)

#define DECANCER_GENERATE_REPLACE_MULTIPLE_WIDE_METHODS_IMPL(string_argument, length_argument, ...)                                                 \
  DECANCER_REPLACE_MULTIPLE_METHOD_IMPL(replace_multiple_wide, const wchar_t*, DECANCER_WSTRING(string_argument), length_argument, __VA_ARGS__)     \
  DECANCER_REPLACE_MULTIPLE_METHOD_IMPL(replace_multiple_wide, std::wstring, DECANCER_WSTRING(string_argument), length_argument, __VA_ARGS__)

#define DECANCER_FIND_METHOD_IMPL(string_argument, length_argument, ...)                                  \
  std::vector<match_t> cured_string::find(__VA_ARGS__) const noexcept {                                   \
    return collect_from_matcher(decancer_find(m_ptr, DECANCER_STRING(string_argument), length_argument)); \
  }

#define DECANCER_FIND_WIDE_METHOD_IMPL(string_argument, length_argument, ...)                                   \
  std::vector<match_t> cured_string::find(__VA_ARGS__) const noexcept {                                         \
    return collect_from_matcher(decancer_find_wide(m_ptr, DECANCER_WSTRING(string_argument), length_argument)); \
  }

#define DECANCER_FIND_MULTIPLE_METHOD_IMPL(method_name, type_name)                                                     \
  std::vector<match_t> cured_string::find_multiple(const std::initializer_list<type_name>& keywords) const noexcept {  \
    auto keywords_in = keywords_from_list(keywords);                                                                   \
    auto matches = decancer_##method_name(m_ptr, keywords_in, keywords.size());                                        \
    delete[] keywords_in;                                                                                              \
                                                                                                                       \
    return collect_from_matches(matches);                                                                              \
  }

#define DECANCER_EQUALS_METHOD_IMPL(string_argument, length_argument, ...)                            \
  bool cured_string::operator==(__VA_ARGS__) const noexcept {                                         \
    return decancer_equals(m_ptr, DECANCER_STRING(string_argument), length_argument);                 \
  }

#define DECANCER_EQUALS_WIDE_METHOD_IMPL(string_argument, length_argument, ...)                       \
  bool cured_string::operator==(__VA_ARGS__) const noexcept {                                         \
    return decancer_equals_wide(m_ptr, DECANCER_WSTRING(string_argument), length_argument);           \
  }

static char* generate_error_message(error_t* err) {
  char* ptr = new char[err->message_length + 1];
  memcpy(ptr, err->message, err->message_length);
  ptr[err->message_length] = 0;

  return ptr;
}

static keyword_t* keywords_from_list(const std::initializer_list<const char*>& keywords) {
  auto keywords_in = new keyword_t[keywords.size()];
  const auto keywords_ptr = keywords.begin();

  for (size_t i = 0; i < keywords.size(); i++) {
    const char* s = keywords_ptr[i];

    keywords_in[i].string = DECANCER_STRING(s);
    keywords_in[i].length = strlen(s);
  }

  return keywords_in;
}

static keyword_t* keywords_from_list(const std::initializer_list<std::string>& keywords) {
  auto keywords_in = new keyword_t[keywords.size()];
  const auto keywords_ptr = keywords.begin();

  for (size_t i = 0; i < keywords.size(); i++) {
    const std::string* s = &keywords_ptr[i];

    keywords_in[i].string = DECANCER_STRING(s->data());
    keywords_in[i].length = s->size();
  }

  return keywords_in;
}

static keyword_wide_t* keywords_from_list(const std::initializer_list<const wchar_t*>& keywords) {
  auto keywords_in = new keyword_wide_t[keywords.size()];
  const auto keywords_ptr = keywords.begin();

  for (size_t i = 0; i < keywords.size(); i++) {
    const wchar_t* s = keywords_ptr[i];

    keywords_in[i].string = DECANCER_WSTRING(s);
    keywords_in[i].length = wcslen(s);
  }

  return keywords_in;
}

static keyword_wide_t* keywords_from_list(const std::initializer_list<std::wstring>& keywords) {
  auto keywords_in = new keyword_wide_t[keywords.size()];
  const auto keywords_ptr = keywords.begin();

  for (size_t i = 0; i < keywords.size(); i++) {
    const std::wstring* s = &keywords_ptr[i];

    keywords_in[i].string = DECANCER_WSTRING(s->data());
    keywords_in[i].length = s->size();
  }

  return keywords_in;
}

static std::vector<match_t> collect_from_matcher(matcher_t matcher) {
  std::vector<match_t> output{};
  match_t portion;

  while (decancer_matcher_next(matcher, &portion)) {
    output.push_back(portion);
  }

  return output;
}

static std::vector<match_t> collect_from_matches(matches_t matches) {
  std::vector<match_t> output{};
  size_t size;
  const auto ptr = decancer_matches_raw(matches, &size);
  output.reserve(size);

  for (size_t i = 0; i < size; i++) {
    output.push_back(ptr[i]);
  }

  decancer_matches_free(matches);
  return output;
}

translation::translation(const translation& other) {
  __decancer_translation_clone(&other.m_translation, &m_translation);
}

translation::translation(translation&& other) {
  memcpy(&m_translation, &other.m_translation, sizeof(translation_t));
  memset(&other.m_translation, 0, sizeof(translation_t));
}

translation::translation(const uint32_t code) {
  memset(&m_translation, 0, sizeof(translation_t));

  decancer_cure_char(code, DECANCER_OPTION_DEFAULT, &m_translation);
}

translation::translation(const uint32_t code, const options_t opt) {
  memset(&m_translation, 0, sizeof(translation_t));

  decancer_cure_char(code, opt, &m_translation);
}

translation& translation::operator=(const translation& other) & {
  decancer_translation_free(&m_translation);
  __decancer_translation_clone(&other.m_translation, &m_translation);
  return *this;
}

translation& translation::operator=(translation&& other) & {
  memcpy(&m_translation, &other.m_translation, sizeof(translation_t));
  memset(&other.m_translation, 0, sizeof(translation_t));
  return *this;
}

translation_variant translation::variant() const noexcept {
  switch (m_translation.kind) {
  case DECANCER_TRANSLATION_KIND_CHARACTER: {
    return translation_variant{m_translation.contents.character};
  }

  case DECANCER_TRANSLATION_KIND_STRING: {
    return translation_variant{std::string(
      reinterpret_cast<const char*>(m_translation.contents.string.contents),
      m_translation.contents.string.length
    )};
  }

  default: {
    return translation_variant{std::string{}};
  }
  }
}

translation::~translation() noexcept {
  decancer_translation_free(&m_translation);
}

cured_string::cured_string(const cured_string& other)
  : m_ptr(__decancer_cured_clone(other.m_ptr)) {}

cured_string& cured_string::operator=(const cured_string& other) & {
  if (m_ptr != nullptr) {
    decancer_cured_free(m_ptr);
  }

  m_ptr = __decancer_cured_clone(other.m_ptr);
  return *this;
}

DECANCER_GENERATE_CTOR_IMPL(text, strlen(text), const char* text)
DECANCER_GENERATE_CTOR_IMPL(text, length, const char* text, const size_t length)
DECANCER_GENERATE_CTOR_IMPL(text.data(), text.size(), const std::string& text)

DECANCER_GENERATE_WIDE_CTOR_IMPL(text, wcslen(text), const wchar_t* text)
DECANCER_GENERATE_WIDE_CTOR_IMPL(text, length, const wchar_t* text, const size_t length)
DECANCER_GENERATE_WIDE_CTOR_IMPL(text.data(), text.size(), const std::wstring& text)

DECANCER_GENERATE_COMPARISON_METHODS_IMPL(starts_with)
DECANCER_GENERATE_COMPARISON_METHODS_IMPL(ends_with)
DECANCER_GENERATE_COMPARISON_METHODS_IMPL(contains)

DECANCER_GENERATE_CENSOR_METHOD_IMPL(text, strlen(text), const char* text)
DECANCER_GENERATE_CENSOR_METHOD_IMPL(text, length, const char* text, const size_t length)
DECANCER_GENERATE_CENSOR_METHOD_IMPL(text.data(), text.size(), const std::string& text)

DECANCER_GENERATE_CENSOR_WIDE_METHOD_IMPL(text, wcslen(text), const wchar_t* text)
DECANCER_GENERATE_CENSOR_WIDE_METHOD_IMPL(text, length, const wchar_t* text, const size_t length)
DECANCER_GENERATE_CENSOR_WIDE_METHOD_IMPL(text.data(), text.size(), const std::wstring& text)

DECANCER_FIND_METHOD_IMPL(text, strlen(text), const char* text)
DECANCER_FIND_METHOD_IMPL(text, length, const char* text, const size_t length)
DECANCER_FIND_METHOD_IMPL(text.data(), text.size(), const std::string& text)

DECANCER_FIND_WIDE_METHOD_IMPL(text, wcslen(text), const wchar_t* text)
DECANCER_FIND_WIDE_METHOD_IMPL(text, length, const wchar_t* text, const size_t length)
DECANCER_FIND_WIDE_METHOD_IMPL(text.data(), text.size(), const std::wstring& text)

DECANCER_CENSOR_MULTIPLE_METHOD_IMPL(censor_multiple, const char*)
DECANCER_CENSOR_MULTIPLE_METHOD_IMPL(censor_multiple, std::string)
DECANCER_CENSOR_MULTIPLE_METHOD_IMPL(censor_multiple_wide, const wchar_t*)
DECANCER_CENSOR_MULTIPLE_METHOD_IMPL(censor_multiple_wide, std::wstring)

DECANCER_REPLACE_METHOD_IMPL(find, strlen(find), replacement, strlen(replacement), const char* find, const char* replacement)
DECANCER_REPLACE_METHOD_IMPL(find, find_length, replacement, replacement_length, const char* find, const size_t find_length, const char* replacement, const size_t replacement_length)
DECANCER_REPLACE_METHOD_IMPL(find.data(), find.size(), replacement.data(), replacement.size(), const std::string& find, const std::string& replacement)

DECANCER_REPLACE_WIDE_METHOD_IMPL(find, wcslen(find), replacement, wcslen(replacement), const wchar_t* find, const wchar_t* replacement)
DECANCER_REPLACE_WIDE_METHOD_IMPL(find, find_length, replacement, replacement_length, const wchar_t* find, const size_t find_length, const wchar_t* replacement, const size_t replacement_length)
DECANCER_REPLACE_WIDE_METHOD_IMPL(find.data(), find.size(), replacement.data(), replacement.size(), const std::wstring& find, const std::wstring& replacement)

DECANCER_GENERATE_REPLACE_MULTIPLE_METHODS_IMPL(replacement, strlen(replacement), const char* replacement)
DECANCER_GENERATE_REPLACE_MULTIPLE_METHODS_IMPL(replacement, replacement_length, const char* replacement, const size_t replacement_length)
DECANCER_GENERATE_REPLACE_MULTIPLE_METHODS_IMPL(replacement.data(), replacement.size(), const std::string& replacement)

DECANCER_GENERATE_REPLACE_MULTIPLE_WIDE_METHODS_IMPL(replacement, wcslen(replacement), const wchar_t* replacement)
DECANCER_GENERATE_REPLACE_MULTIPLE_WIDE_METHODS_IMPL(replacement, replacement_length, const wchar_t* replacement, const size_t replacement_length)
DECANCER_GENERATE_REPLACE_MULTIPLE_WIDE_METHODS_IMPL(replacement.data(), replacement.size(), const std::wstring& replacement)

DECANCER_FIND_MULTIPLE_METHOD_IMPL(find_multiple, const char*)
DECANCER_FIND_MULTIPLE_METHOD_IMPL(find_multiple, std::string)
DECANCER_FIND_MULTIPLE_METHOD_IMPL(find_multiple_wide, const wchar_t*)
DECANCER_FIND_MULTIPLE_METHOD_IMPL(find_multiple_wide, std::wstring)

DECANCER_EQUALS_METHOD_IMPL(text, strlen(text), const char* text)
DECANCER_EQUALS_METHOD_IMPL(text.data(), text.size(), const std::string& text)
DECANCER_EQUALS_WIDE_METHOD_IMPL(text, wcslen(text), const wchar_t* text)
DECANCER_EQUALS_WIDE_METHOD_IMPL(text.data(), text.size(), const std::wstring& text)

cured_string::operator std::string() const noexcept {
  size_t size;
  const uint8_t* ptr = decancer_cured_raw(m_ptr, &size);

  return std::string(reinterpret_cast<const char*>(ptr), size);
}

cured_string::operator std::wstring() const noexcept {
  size_t size;
  uint16_t* ptr;

  auto handle = decancer_cured_raw_wide(m_ptr, &ptr, &size);

  std::wstring output(reinterpret_cast<wchar_t*>(ptr), size);
  decancer_cured_raw_wide_free(handle);

  return output;
}

cured_string::~cured_string() noexcept {
  if (m_ptr != nullptr) {
    decancer_cured_free(m_ptr);
  }
}