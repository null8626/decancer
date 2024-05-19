#include <decancer.hpp>
#include <cstring>

using namespace decancer;

#define DECANCER_STRING(text)              reinterpret_cast<const uint8_t*>(text)
#define DECANCER_WSTRING(text)             reinterpret_cast<const uint16_t*>(text)
#define DECANCER_INTO_ERROR(error_struct)  error(generate_error_message(&error_struct), error_struct.message_length + 1)

#define DECANCER_CURED_CTOR_IMPL(text_argument, length_argument, ...)                                                         \
  cured_string::cured_string(__VA_ARGS__) {                                                                                   \
    error_t err;                                                                                                              \
    if ((m_ptr = decancer_cure(DECANCER_STRING(text_argument), length_argument, DECANCER_OPTION_DEFAULT, &err)) == nullptr) { \
      throw DECANCER_INTO_ERROR(err);                                                                                         \
    }                                                                                                                         \
  }                                                                                                                           \
  cured_string::cured_string(__VA_ARGS__, const options_t opt) {                                                              \
    error_t err;                                                                                                              \
    if ((m_ptr = decancer_cure(DECANCER_STRING(text_argument), length_argument, opt, &err)) == nullptr) {                     \
      throw DECANCER_INTO_ERROR(err);                                                                                         \
    }                                                                                                                         \
  }

#define DECANCER_CURED_WIDE_CTOR_IMPL(text_argument, length_argument, ...)                                                          \
  cured_string::cured_string(__VA_ARGS__) {                                                                                         \
    error_t err;                                                                                                                    \
    if ((m_ptr = decancer_cure_wide(DECANCER_WSTRING(text_argument), length_argument, DECANCER_OPTION_DEFAULT, &err)) == nullptr) { \
      throw DECANCER_INTO_ERROR(err);                                                                                               \
    }                                                                                                                               \
  }                                                                                                                                 \
  cured_string::cured_string(__VA_ARGS__, const options_t opt) {                                                                    \
    error_t err;                                                                                                                    \
    if ((m_ptr = decancer_cure_wide(DECANCER_WSTRING(text_argument), length_argument, opt, &err)) == nullptr) {                     \
      throw DECANCER_INTO_ERROR(err);                                                                                               \
    }                                                                                                                               \
  }

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

#define DECANCER_FIND_METHOD_IMPL(string_argument, length_argument, ...)                                  \
  std::vector<match_t> cured_string::find(__VA_ARGS__) const noexcept {                                   \
    return collect_from_matcher(decancer_find(m_ptr, DECANCER_STRING(string_argument), length_argument)); \
  }

#define DECANCER_FIND_WIDE_METHOD_IMPL(string_argument, length_argument, ...)                                   \
  std::vector<match_t> cured_string::find(__VA_ARGS__) const noexcept {                                         \
    return collect_from_matcher(decancer_find_wide(m_ptr, DECANCER_WSTRING(string_argument), length_argument)); \
  }

static char* generate_error_message(error_t* err) {
  char* ptr = new char[err->message_length + 1];
  memcpy(ptr, err->message, err->message_length);
  ptr[err->message_length] = 0;
  
  return ptr;
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

error::error(const error& other): std::runtime_error(""), m_size(other.m_size) {
  m_ptr = new char[other.m_size];
  memcpy(m_ptr, other.m_ptr, other.m_size);
}

translation::translation(const translation& other) {
  __decancer_translation_clone(&other.m_translation, &m_translation);
}

translation::translation(const uint32_t code) {
  memset(&m_translation, 0, sizeof(translation_t));
  
  decancer_cure_char(code, DECANCER_OPTION_DEFAULT, &m_translation);
}

translation::translation(const uint32_t code, const options_t opt) {
  memset(&m_translation, 0, sizeof(translation_t));
  
  decancer_cure_char(code, opt, &m_translation);
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

cured_string::cured_string(const cured_string& other): m_ptr(__decancer_cured_clone(other.m_ptr)) {}

DECANCER_CURED_CTOR_IMPL(text, strlen(text), const char* text)
DECANCER_CURED_CTOR_IMPL(text, length, const char* text, const size_t length)
DECANCER_CURED_CTOR_IMPL(text.data(), text.size(), const std::string& text)

DECANCER_CURED_WIDE_CTOR_IMPL(text, wcslen(text), const wchar_t* text)
DECANCER_CURED_WIDE_CTOR_IMPL(text, length, const wchar_t* text, const size_t length)
DECANCER_CURED_WIDE_CTOR_IMPL(text.data(), text.size(), const std::wstring& text)

DECANCER_GENERATE_COMPARISON_METHODS_IMPL(starts_with)
DECANCER_GENERATE_COMPARISON_METHODS_IMPL(ends_with)
DECANCER_GENERATE_COMPARISON_METHODS_IMPL(contains)

DECANCER_FIND_METHOD_IMPL(text, strlen(text), const char* text)
DECANCER_FIND_METHOD_IMPL(text, length, const char* text, const size_t length)
DECANCER_FIND_METHOD_IMPL(text.data(), text.size(), const std::string& text)

DECANCER_FIND_WIDE_METHOD_IMPL(text, wcslen(text), const wchar_t* text)
DECANCER_FIND_WIDE_METHOD_IMPL(text, length, const wchar_t* text, const size_t length)
DECANCER_FIND_WIDE_METHOD_IMPL(text.data(), text.size(), const std::wstring& text)

std::vector<match_t> cured_string::find_multiple(const std::initializer_list<const char*>& keywords) const noexcept {
  auto keywords_in = new keyword_t[keywords.size()];
  const auto keywords_ptr = keywords.begin();
  
  for (size_t i = 0; i < keywords.size(); i++) {
    const char* s = keywords_ptr[i];
    
    keywords_in[i].string = DECANCER_STRING(s);
    keywords_in[i].size = strlen(s);
  }
  
  auto matches = decancer_find_multiple(m_ptr, keywords_in, keywords.size());
  delete[] keywords_in;
  
  return collect_from_matches(matches);
}

std::vector<match_t> cured_string::find_multiple(const std::initializer_list<std::string>& keywords) const noexcept {
  auto keywords_in = new keyword_t[keywords.size()];
  const auto keywords_ptr = keywords.begin();
  
  for (size_t i = 0; i < keywords.size(); i++) {
    const std::string* s = &keywords_ptr[i];
    
    keywords_in[i].string = DECANCER_STRING(s->data());
    keywords_in[i].size = s->size();
  }
  
  auto matches = decancer_find_multiple(m_ptr, keywords_in, keywords.size());
  delete[] keywords_in;
  
  return collect_from_matches(matches);
}

std::vector<match_t> cured_string::find_multiple(const std::initializer_list<const wchar_t*>& keywords) const noexcept {
  auto keywords_in = new keyword_wide_t[keywords.size()];
  const auto keywords_ptr = keywords.begin();
  
  for (size_t i = 0; i < keywords.size(); i++) {
    const wchar_t* s = keywords_ptr[i];
    
    keywords_in[i].string = DECANCER_WSTRING(s);
    keywords_in[i].size = wcslen(s);
  }
  
  auto matches = decancer_find_multiple_wide(m_ptr, keywords_in, keywords.size());
  delete[] keywords_in;
  
  return collect_from_matches(matches);
}

std::vector<match_t> cured_string::find_multiple(const std::initializer_list<std::wstring>& keywords) const noexcept {
  auto keywords_in = new keyword_wide_t[keywords.size()];
  const auto keywords_ptr = keywords.begin();
  
  for (size_t i = 0; i < keywords.size(); i++) {
    const std::wstring* s = &keywords_ptr[i];
    
    keywords_in[i].string = DECANCER_WSTRING(s->data());
    keywords_in[i].size = s->size();
  }
  
  auto matches = decancer_find_multiple_wide(m_ptr, keywords_in, keywords.size());
  delete[] keywords_in;
  
  return collect_from_matches(matches);
}

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

bool cured_string::operator==(const char* text) const noexcept {
  return decancer_equals(m_ptr, DECANCER_STRING(text), strlen(text));
}

bool cured_string::operator==(const std::string& text) const noexcept {
  return decancer_equals(m_ptr, DECANCER_STRING(text.data()), text.size());
}

bool cured_string::operator==(const wchar_t* text) const noexcept {
  return decancer_equals_wide(m_ptr, DECANCER_WSTRING(text), wcslen(text));
}

bool cured_string::operator==(const std::wstring& text) const noexcept {
  return decancer_equals_wide(m_ptr, DECANCER_WSTRING(text.data()), text.size());
}

cured_string::~cured_string() noexcept {
  if (m_ptr != nullptr) {
    decancer_cured_free(m_ptr);
  }
}