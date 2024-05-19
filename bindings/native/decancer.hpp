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

  class error: public std::runtime_error {
    char* m_ptr;
    const size_t m_size;
    
    inline error(char* ptr, const size_t size): std::runtime_error(""), m_ptr(ptr), m_size(size) {}
  public:
    error() = delete;
    error(const error& other);
    
    inline const char* what() const noexcept override {
      return m_ptr;
    }
    
    inline ~error() noexcept {
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
    
    std::vector<match_t> find_multiple(const std::initializer_list<const char*>& keywords) const noexcept;
    std::vector<match_t> find_multiple(const std::initializer_list<std::string>& keywords) const noexcept;
    std::vector<match_t> find_multiple(const std::initializer_list<const wchar_t*>& keywords) const noexcept;
    std::vector<match_t> find_multiple(const std::initializer_list<std::wstring>& keywords) const noexcept;
    
    explicit operator std::string() const noexcept;
    explicit operator std::wstring() const noexcept;
    
    bool operator=(const char* text) const noexcept;
    bool operator=(const std::string& text) const noexcept;
    bool operator=(const wchar_t* text) const noexcept;
    bool operator=(const std::wstring& text) const noexcept;
    
    ~cured_string() noexcept;
  };
};

#undef DECANCER_CURED_CTOR
#undef DECANCER_CURED_METHOD