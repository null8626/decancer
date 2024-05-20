#pragma once

#include <initializer_list>
#include <stdexcept>
#include <variant>
#include <vector>
#include <string>

#define DECANCER_CURED_CTOR(...)                  \
  cured_string(__VA_ARGS__);                      \
  cured_string(__VA_ARGS__, const options_t opt)

#define DECANCER_CURED_METHOD(ret_type, name)                             \
  ret_type name(const char* text) const noexcept;                         \
  ret_type name(const char* text, const size_t length) const noexcept;    \
  ret_type name(const std::string& text) const noexcept;                  \
  ret_type name(const wchar_t* text) const noexcept;                      \
  ret_type name(const wchar_t* text, const size_t length) const noexcept; \
  ret_type name(const std::wstring& text) const noexcept

#define DECANCER_CENSOR_METHOD(name, ...)                  \
  void name(__VA_ARGS__, const char replacement) const;    \
  void name(__VA_ARGS__, const wchar_t replacement) const; \
  void name(__VA_ARGS__, const uint32_t replacement) const

#define DECANCER_REPLACE_MULTIPLE_METHOD(...)                                                   \
  void replace_multiple(const std::initializer_list<const char*>& keywords, __VA_ARGS__) const; \
  void replace_multiple(const std::initializer_list<std::string>& keywords, __VA_ARGS__) const

#define DECANCER_REPLACE_MULTIPLE_WIDE_METHOD(...)                                                 \
  void replace_multiple(const std::initializer_list<const wchar_t*>& keywords, __VA_ARGS__) const; \
  void replace_multiple(const std::initializer_list<std::wstring>& keywords, __VA_ARGS__) const

namespace decancer {

#ifdef __DECANCER_CXX_BUILDING__
#ifdef _WIN32
#pragma comment(lib, "userenv")
#pragma comment(lib, "ntdll")
#pragma comment(lib, "ws2_32")
#endif
#else
#define __DECANCER_CXX__
#endif

#include <decancer.h>

#ifdef __DECANCER_CXX__
#undef __DECANCER_CXX__
#endif

  class cured_string;
  class native_error;

  class error: public std::runtime_error {
    inline error(const char* message): std::runtime_error(message) {}
    
    friend class cured_string;
    friend class native_error;
  };

  class native_error: public error {
    char* m_ptr;
    const size_t m_size;
    
    inline native_error(char* ptr, const size_t size): error(""), m_ptr(ptr), m_size(size) {}
  public:
    native_error() = delete;
    native_error(const native_error& other);
    
    inline const char* what() const noexcept override {
      return m_ptr;
    }
    
    inline ~native_error() noexcept {
      delete[] m_ptr;
    }
    
    friend class cured_string;
  };
  
  using translation_variant = std::variant<uint32_t, std::string>;
  
  class translation {
    translation_t m_translation;
    
  public:
    translation(const uint32_t code);
    translation(const uint32_t code, const options_t opt);
    translation(const translation& other);
    
    translation_variant variant() const noexcept;
    
    ~translation();
  };

  class cured_string {
    cured_t m_ptr;
  
  public:
    cured_string(const cured_string& other);
  
    DECANCER_CURED_CTOR(const char* text);
    DECANCER_CURED_CTOR(const char* text, const size_t length);
    DECANCER_CURED_CTOR(const std::string& text);
    
    DECANCER_CURED_CTOR(const wchar_t* text);
    DECANCER_CURED_CTOR(const wchar_t* text, const size_t length);
    DECANCER_CURED_CTOR(const std::wstring& text);
    
    DECANCER_CURED_METHOD(bool, starts_with);
    DECANCER_CURED_METHOD(bool, ends_with);
    DECANCER_CURED_METHOD(bool, contains);
    DECANCER_CURED_METHOD(std::vector<match_t>, find);
    
    DECANCER_CENSOR_METHOD(censor, const char* find);
    DECANCER_CENSOR_METHOD(censor, const char* find, const size_t find_length);
    DECANCER_CENSOR_METHOD(censor, const std::string& find);
    DECANCER_CENSOR_METHOD(censor, const wchar_t* find);
    DECANCER_CENSOR_METHOD(censor, const wchar_t* find, const size_t find_length);
    DECANCER_CENSOR_METHOD(censor, const std::wstring& find);
    
    std::vector<match_t> find_multiple(const std::initializer_list<const char*>& keywords) const noexcept;
    std::vector<match_t> find_multiple(const std::initializer_list<std::string>& keywords) const noexcept;
    std::vector<match_t> find_multiple(const std::initializer_list<const wchar_t*>& keywords) const noexcept;
    std::vector<match_t> find_multiple(const std::initializer_list<std::wstring>& keywords) const noexcept;
    
    DECANCER_CENSOR_METHOD(censor_multiple, const std::initializer_list<const char*>& keywords);
    DECANCER_CENSOR_METHOD(censor_multiple, const std::initializer_list<std::string>& keywords);
    DECANCER_CENSOR_METHOD(censor_multiple, const std::initializer_list<const wchar_t*>& keywords);
    DECANCER_CENSOR_METHOD(censor_multiple, const std::initializer_list<std::wstring>& keywords);
    
    void replace(const char* find, const char* replacement) const;
    void replace(const char* find, const size_t find_length, const char* replacement, const size_t replacement_length) const;
    void replace(const std::string& find, const std::string& replacement) const;
    void replace(const wchar_t* find, const wchar_t* replacement) const;
    void replace(const wchar_t* find, const size_t find_length, const wchar_t* replacement, const size_t replacement_length) const;
    void replace(const std::wstring& find, const std::wstring& replacement) const;
    
    DECANCER_REPLACE_MULTIPLE_METHOD(const char* replacement);
    DECANCER_REPLACE_MULTIPLE_METHOD(const char* replacement, const size_t replacement_length);
    DECANCER_REPLACE_MULTIPLE_METHOD(const std::string& replacement);
    
    DECANCER_REPLACE_MULTIPLE_WIDE_METHOD(const wchar_t* replacement);
    DECANCER_REPLACE_MULTIPLE_WIDE_METHOD(const wchar_t* replacement, const size_t replacement_length);
    DECANCER_REPLACE_MULTIPLE_WIDE_METHOD(const std::wstring& replacement);
    
    explicit operator std::string() const noexcept;
    explicit operator std::wstring() const noexcept;
    
    bool operator==(const char* text) const noexcept;
    bool operator==(const std::string& text) const noexcept;
    bool operator==(const wchar_t* text) const noexcept;
    bool operator==(const std::wstring& text) const noexcept;
    
    ~cured_string() noexcept;
  };
};

#undef DECANCER_CURED_CTOR
#undef DECANCER_CURED_METHOD
#undef DECANCER_CENSOR_METHOD
#undef DECANCER_REPLACE_MULTIPLE_METHOD
#undef DECANCER_REPLACE_MULTIPLE_WIDE_METHOD