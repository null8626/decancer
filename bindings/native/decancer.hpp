#pragma once

#include <initializer_list>
#include <stdexcept>
#include <variant>
#include <cstring>
#include <vector>
#include <string>

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
#define DECANCER_CXX_EXPORT __declspec(dllexport)
#else
#define DECANCER_CXX_EXPORT
#endif
#else
#ifdef _WIN32
#define DECANCER_CXX_EXPORT __declspec(dllimport)
#else
#define DECANCER_CXX_EXPORT
#endif
#define __DECANCER_CXX__
#endif

#include <decancer.h>

#ifdef __DECANCER_CXX__
#undef __DECANCER_CXX__
#endif

  class cured_string;
  class native_error;

  /**
   * @brief Represents any error thrown from decancer.
   */
  class error: public std::runtime_error {
    inline error(const char* message)
      : std::runtime_error(message) {}

    inline error()
      : std::runtime_error("") {}

    friend class cured_string;
    friend class native_error;
  };

  /**
   * @brief Represents an error caused by decancer not being able to cure a string.
   */
  class native_error: public error {
    char* m_ptr;
    size_t m_size;

    inline native_error(char* ptr, const size_t size)
      : m_ptr(ptr), m_size(size) {}

  public:
    /**
     * @brief No outsiders are allowed to use this class, internal use only :)
     */
    native_error() = delete;

    /**
     * @brief Creates a native error object by copying another native error object.
     * @param other The other native error object to copy from.
     */
    inline native_error(const native_error& other)
      : error(""), m_size(other.m_size) {
      m_ptr = new char[other.m_size];
      memcpy(m_ptr, other.m_ptr, other.m_size);
    }

    /**
     * @brief Creates a native error object by moving another native error object.
     * @param other The other native error object to move from.
     */
    inline native_error(native_error&& other)
      : m_ptr(other.m_ptr), m_size(other.m_size) {
      other.m_ptr = nullptr;
    }

    /**
     * @brief Copies another native error object.
     * @param other The other native error object to copy from.
     */
    native_error& operator=(const native_error& other) & {
      if (m_ptr != nullptr) {
        delete[] m_ptr;
      }

      m_ptr = new char[other.m_size];

      memcpy(m_ptr, other.m_ptr, other.m_size);
      m_size = other.m_size;

      return *this;
    }

    /**
     * @brief Moves another native error object.
     * @param other The other native error object to move from.
     */
    inline native_error& operator=(native_error&& other) & {
      m_ptr = other.m_ptr;
      m_size = other.m_size;
      other.m_ptr = nullptr;
      return *this;
    }

    /**
     * @brief Returns the raw null-terminated error message.
     * @return const char* The raw null-terminated error message.
     */
    inline const char* what() const noexcept override {
      return m_ptr;
    }

    /**
     * @brief Frees the native error object.
     */
    inline ~native_error() noexcept {
      if (m_ptr != nullptr) {
        delete[] m_ptr;
      }
    }

    friend class cured_string;
  };

  /**
   * @brief Represents a translation variant. This can either contain a unicode character or an ASCII string.
   */
  using translation_variant = std::variant<uint32_t, std::string>;

  /**
   * @brief Represents a translation of a unicode codepoint.
   */
  class DECANCER_CXX_EXPORT translation {
    translation_t m_translation;

  public:
    /**
     * @brief Cures a unicode codepoint.
     * @param code The unicode codepoint to cure.
     * @param opt Options to customize decancer's curing behavior. Defaults to DECANCER_OPTION_DEFAULT.
     */
    translation(const uint32_t code, const options_t opt = DECANCER_OPTION_DEFAULT);

    /**
     * @brief Creates a translation object by copying another translation object.
     * @param other The other translation object to copy from.
     */
    translation(const translation& other);

    /**
     * @brief Creates a translation object by moving another translation object.
     * @param other The other translation object to move from.
     */
    translation(translation&& other);

    /**
     * @brief Copies another translation object.
     * @param other The other translation object to copy from.
     */
    translation& operator=(const translation& other) &;

    /**
     * @brief Moves another translation object.
     * @param other The other translation object to copy from.
     */
    translation& operator=(translation&& other) &;

    /**
     * @brief Returns the variant contained in this translation object.
     * @return translation_variant The variant contained in this translation object.
     */
    translation_variant variant() const noexcept;

    /**
     * @brief Frees this translation object.
     */
    ~translation();
  };

  /**
   * @brief Represents a cured string.
   */
  class DECANCER_CXX_EXPORT cured_string {
    cured_t m_ptr;

  public:
    /**
     * @brief Creates a cured string object by copying another cured string object.
     * @param other The other cured string object to copy from.
     */
    cured_string(const cured_string& other);

    /**
     * @brief Creates a cured string object by moving another cured string object.
     * @param other The other cured string object to move from.
     */
    inline cured_string(cured_string&& other)
      : m_ptr(other.m_ptr) {
      other.m_ptr = nullptr;
    }

    /**
     * @brief Copies another cured string object.
     * @param other The other cured string object to copy from.
     */
    cured_string& operator=(const cured_string& other) &;

    /**
     * @brief Move another cured string object.
     * @param other The other cured string object to move from.
     */
    inline cured_string& operator=(cured_string&& other) & {
      m_ptr = other.m_ptr;
      other.m_ptr = nullptr;
      return *this;
    }

    /**
     * @brief Cures a raw null-terminated UTF-8 encoded string.
     * @param text The raw null-terminated UTF-8 encoded string.
     * @param opt Options to customize decancer's curing behavior. Defaults to DECANCER_OPTION_DEFAULT.
     */
    cured_string(const char* text, const options_t opt = DECANCER_OPTION_DEFAULT);

    /**
     * @brief Cures a raw UTF-8 encoded string.
     * @param text The raw UTF-8 encoded string.
     * @param size UTF-8 size of the other string, in bytes.
     * @param opt Options to customize decancer's curing behavior. Defaults to DECANCER_OPTION_DEFAULT.
     */
    cured_string(const char* text, const size_t size, const options_t opt = DECANCER_OPTION_DEFAULT);

    /**
     * @brief Cures a UTF-8 encoded string.
     * @param text The UTF-8 encoded string.
     * @param opt Options to customize decancer's curing behavior. Defaults to DECANCER_OPTION_DEFAULT.
     */
    cured_string(const std::string& text, const options_t opt = DECANCER_OPTION_DEFAULT);

    /**
     * @brief Cures a raw null-terminated UTF-16 encoded string.
     * @param text The raw null-terminated UTF-16 encoded string.
     * @param opt Options to customize decancer's curing behavior. Defaults to DECANCER_OPTION_DEFAULT.
     */
    cured_string(const wchar_t* text, const options_t opt = DECANCER_OPTION_DEFAULT);

    /**
     * @brief Cures a raw UTF-16 encoded string.
     * @param text The raw UTF-16 encoded string.
     * @param size UTF-16 size of the other string, in bytes.
     * @param opt Options to customize decancer's curing behavior. Defaults to DECANCER_OPTION_DEFAULT.
     */
    cured_string(const wchar_t* text, const size_t size, const options_t opt = DECANCER_OPTION_DEFAULT);

    /**
     * @brief Cures a UTF-16 encoded string.
     * @param text The UTF-16 encoded string.
     * @param opt Options to customize decancer's curing behavior. Defaults to DECANCER_OPTION_DEFAULT.
     */
    cured_string(const std::wstring& text, const options_t opt = DECANCER_OPTION_DEFAULT);

    /**
     * @brief Checks if this cured string similarly starts with another raw null-terminated UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   assert(cured_utf8.starts_with("very"), "starts_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-8 encoded string to match with.
     * @return bool Whether this cured string similarly starts with the specified string.
     */
    bool starts_with(const char* text) const noexcept;

    /**
     * @brief Checks if this cured string similarly starts with another raw UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   assert(cured_utf8.starts_with("very", 4), "starts_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-8 encoded string to match with.
     * @param size UTF-8 size of the other string, in bytes.
     * @return bool Whether this cured string similarly starts with the specified string.
     */
    bool starts_with(const char* text, const size_t size) const noexcept;

    /**
     * @brief Checks if this cured string similarly starts with another UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string other_string{"very"};
     *
     *   assert(cured_utf8.starts_with(other_string), "starts_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-8 encoded string to match with.
     * @return bool Whether this cured string similarly starts with the specified string.
     */
    bool starts_with(const std::string& text) const noexcept;

    /**
     * @brief Checks if this cured string similarly starts with another raw null-terminated UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   wassert(cured_utf16.starts_with(L"very"), "starts_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-16 encoded string to match with.
     * @return bool Whether this cured string similarly starts with the specified string.
     */
    bool starts_with(const wchar_t* text) const noexcept;

    /**
     * @brief Checks if this cured string similarly starts with another raw UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   wassert(cured_utf16.starts_with(L"very", 8), "starts_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-16 encoded string to match with.
     * @param size UTF-16 size of the other string, in bytes.
     * @return bool Whether this cured string similarly starts with the specified string.
     */
    bool starts_with(const wchar_t* text, const size_t size) const noexcept;

    /**
     * @brief Checks if this cured string similarly starts with another UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring other_string{L"very"};
     *
     *   wassert(cured_utf16.starts_with(other_string), "starts_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-16 encoded string to match with.
     * @return bool Whether this cured string similarly starts with the specified string.
     */
    bool starts_with(const std::wstring& text) const noexcept;

    /**
     * @brief Checks if this cured string similarly ends with another raw null-terminated UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   assert(cured_utf8.ends_with("text"), "ends_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-8 encoded string to match with.
     * @return bool Whether this cured string similarly ends with the specified string.
     */
    bool ends_with(const char* text) const noexcept;

    /**
     * @brief Checks if this cured string similarly ends with another raw UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   assert(cured_utf8.ends_with("text", 4), "ends_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-8 encoded string to match with.
     * @param size UTF-8 size of the other string, in bytes.
     * @return bool Whether this cured string similarly ends with the specified string.
     */
    bool ends_with(const char* text, const size_t size) const noexcept;

    /**
     * @brief Checks if this cured string similarly ends with another UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string other_string{"text"};
     *
     *   assert(cured_utf8.ends_with(other_string), "ends_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-8 encoded string to match with.
     * @return bool Whether this cured string similarly ends with the specified string.
     */
    bool ends_with(const std::string& text) const noexcept;

    /**
     * @brief Checks if this cured string similarly ends with another raw null-terminated UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   wassert(cured_utf16.ends_with(L"text"), "ends_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-16 encoded string to match with.
     * @return bool Whether this cured string similarly ends with the specified string.
     */
    bool ends_with(const wchar_t* text) const noexcept;

    /**
     * @brief Checks if this cured string similarly ends with another raw UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   wassert(cured_utf16.ends_with(L"text", 8), "ends_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-16 encoded string to match with.
     * @param size UTF-16 size of the other string, in bytes.
     * @return bool Whether this cured string similarly ends with the specified string.
     */
    bool ends_with(const wchar_t* text, const size_t size) const noexcept;

    /**
     * @brief Checks if this cured string similarly ends with another UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring other_string{L"text"};
     *
     *   wassert(cured_utf16.ends_with(other_string), "ends_with");
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-16 encoded string to match with.
     * @return bool Whether this cured string similarly ends with the specified string.
     */
    bool ends_with(const std::wstring& text) const noexcept;

    /**
     * @brief Checks if this cured string similarly contains another raw null-terminated UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   assert(cured_utf8.contains("funny"), "contains");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-8 encoded string to match with.
     * @return bool Whether this cured string similarly contains the specified string.
     */
    bool contains(const char* text) const noexcept;

    /**
     * @brief Checks if this cured string similarly contains another raw UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   assert(cured_utf8.contains("funny", 5), "contains");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-8 encoded string to match with.
     * @param size UTF-8 size of the other string, in bytes.
     * @return bool Whether this cured string similarly contains the specified string.
     */
    bool contains(const char* text, const size_t size) const noexcept;

    /**
     * @brief Checks if this cured string similarly contains another UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string other_string{"funny"};
     *
     *   assert(cured_utf8.contains(other_string), "contains");
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-8 encoded string to match with.
     * @return bool Whether this cured string similarly contains the specified string.
     */
    bool contains(const std::string& text) const noexcept;

    /**
     * @brief Checks if this cured string similarly contains another raw null-terminated UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   wassert(cured_utf16.contains(L"funny"), "contains");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-16 encoded string to match with.
     * @return bool Whether this cured string similarly contains the specified string.
     */
    bool contains(const wchar_t* text) const noexcept;

    /**
     * @brief Checks if this cured string similarly contains another raw UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   wassert(cured_utf16.contains(L"funny", 10), "contains");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-16 encoded string to match with.
     * @param size UTF-16 size of the other string, in bytes.
     * @return bool Whether this cured string similarly contains the specified string.
     */
    bool contains(const wchar_t* text, const size_t size) const noexcept;

    /**
     * @brief Checks if this cured string similarly contains another UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring other_string{L"funny"};
     *
     *   wassert(cured_utf16.contains(other_string), "contains");
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-16 encoded string to match with.
     * @return bool Whether this cured string similarly contains the specified string.
     */
    bool contains(const std::wstring& text) const noexcept;

    /**
     * @brief Checks if this cured string is similar with another raw null-terminated UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   assert(cured_utf8 == "very funny text", "operator=");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-8 encoded string to match with.
     * @return bool Whether this cured string is similar with the specified string.
     */
    bool operator==(const char* text) const noexcept;

    /**
     * @brief Checks if this cured string is similar with another UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string other_string{"very funny text"};
     *
     *   assert(cured_utf8 == other_string, "operator=");
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-8 encoded string to match with.
     * @return bool Whether this cured string is similar with the specified string.
     */
    bool operator==(const std::string& text) const noexcept;

    /**
     * @brief Checks if this cured string is similar with another raw null-terminated UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   wassert(cured_utf16 == L"very funny text", "operator=");
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-16 encoded string to match with.
     * @return bool Whether this cured string is similar with the specified string.
     */
    bool operator==(const wchar_t* text) const noexcept;

    /**
     * @brief Checks if this cured string is similar with another UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring other_string{L"very funny text"};
     *
     *   wassert(cured_utf16 == other_string, "operator=");
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-16 encoded string to match with.
     * @return bool Whether this cured string is similar with the specified string.
     */
    bool operator==(const std::wstring& text) const noexcept;

    /**
     * @brief Finds every similar-looking match of a UTF-8 encoded string in the cured string.
     * If you want to use a list of keywords, see find_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf8.find("funny");
     *   assert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   assert(first_match.start == 5, "match start");
     *   assert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-8 encoded string to match with.
     * @see find_multiple
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find(const char* text) const noexcept;

    /**
     * @brief Finds every similar-looking match of a UTF-8 encoded string in the cured string.
     * If you want to use a list of keywords, see find_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf8.find("funny", 5);
     *   assert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   assert(first_match.start == 5, "match start");
     *   assert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-8 encoded string to match with.
     * @param size UTF-8 size of the other string, in bytes.
     * @see find_multiple
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find(const char* text, const size_t size) const noexcept;

    /**
     * @brief Finds every similar-looking match of a UTF-8 encoded string in the cured string.
     * If you want to use a list of keywords, see find_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string other_string{"funny"};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf8.find(other_string);
     *   assert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   assert(first_match.start == 5, "match start");
     *   assert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-8 encoded string to match with.
     * @see find_multiple
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find(const std::string& text) const noexcept;

    /**
     * @brief Finds every similar-looking match of a UTF-16 encoded string in the cured string.
     * If you want to use a list of keywords, see find_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf16.find(L"funny");
     *   wassert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   wassert(first_match.start == 5, "match start");
     *   wassert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-16 encoded string to match with.
     * @see find_multiple
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find(const wchar_t* text) const noexcept;

    /**
     * @brief Finds every similar-looking match of a UTF-16 encoded string in the cured string.
     * If you want to use a list of keywords, see find_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf16.find(L"funny", 10);
     *   wassert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   wassert(first_match.start == 5, "match start");
     *   wassert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-16 encoded string to match with.
     * @param size UTF-16 size of the other string, in bytes.
     * @see find_multiple
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find(const wchar_t* text, const size_t size) const noexcept;

    /**
     * @brief Finds every similar-looking match of a UTF-16 encoded string in the cured string.
     * If you want to use a list of keywords, see find_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring other_string{L"funny"};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf16.find(other_string);
     *   wassert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   wassert(first_match.start == 5, "match start");
     *   wassert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-16 encoded string to match with.
     * @see find_multiple
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find(const std::wstring& text) const noexcept;

    /**
     * @brief Finds every similar-looking match from a list of UTF-8 keywords in the cured string.
     * Unlike find, this method also takes note of overlapping matches and merges them together.
     * If you only want to use a single keyword, see find.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf8.find_multiple({"very", "funny"});
     *   assert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   assert(first_match.start == 0, "match start");
     *   assert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-8 keywords to match with.
     * @see find
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find_multiple(const std::initializer_list<const char*>& keywords) const noexcept;

    /**
     * @brief Finds every similar-looking match from a list of UTF-8 keywords in the cured string.
     * Unlike find, this method also takes note of overlapping matches and merges them together.
     * If you only want to use a single keyword, see find.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string very{"very"};
     *   std::string funny{"funny"};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf8.find_multiple({very, funny});
     *   assert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   assert(first_match.start == 0, "match start");
     *   assert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-8 keywords to match with.
     * @see find
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find_multiple(const std::initializer_list<std::string>& keywords) const noexcept;

    /**
     * @brief Finds every similar-looking match from a list of UTF-16 keywords in the cured string.
     * Unlike find, this method also takes note of overlapping matches and merges them together.
     * If you only want to use a single keyword, see find.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf16.find_multiple({L"very", L"funny"});
     *   wassert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   wassert(first_match.start == 0, "match start");
     *   wassert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-16 keywords to match with.
     * @see find
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find_multiple(const std::initializer_list<const wchar_t*>& keywords) const noexcept;

    /**
     * @brief Finds every similar-looking match from a list of UTF-16 keywords in the cured string.
     * Unlike find, this method also takes note of overlapping matches and merges them together.
     * If you only want to use a single keyword, see find.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring very{L"very"};
     *   std::wstring funny{L"funny"};
     *   std::vector<decancer::match_t> matches{};
     *   decancer::match_t first_match;
     *
     *   matches = cured_utf16.find_multiple({very, funny});
     *   wassert(matches.size() == 1, "matches size");
     *
     *   first_match = matches.at(0);
     *   wassert(first_match.start == 0, "match start");
     *   wassert(first_match.end == 10, "match end");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-16 keywords to match with.
     * @see find
     * @return std::vector<decancer::match_t> A list of every match in the cured string.
     * @note Each match is based on UTF-8 character indices.
     */
    std::vector<match_t> find_multiple(const std::initializer_list<std::wstring>& keywords) const noexcept;

    /**
     * @brief Censors every similar-looking match of the specified UTF-8 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.censor("funny", '-');
     *   assert(cured_utf8 == "very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-8 encoded string to match with.
     * @param replacement The replacement character, in ASCII.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const char* text, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-8 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.censor("funny", 5, '-');
     *   assert(cured_utf8 == "very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-8 encoded string to match with.
     * @param size UTF-8 size of the other string, in bytes.
     * @param replacement The replacement character, in ASCII.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const char* text, const size_t size, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-8 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string other_string{"funny"};
     *
     *   cured_utf8.censor(other_string, '-');
     *   assert(cured_utf8 == "very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-8 encoded string to match with.
     * @param replacement The replacement character, in ASCII.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const std::string& text, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-16 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.censor(L"funny", '-');
     *   wassert(cured_utf16 == L"very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-16 encoded string to match with.
     * @param replacement The replacement character, in ASCII.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const wchar_t* text, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-16 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.censor(L"funny", 10, '-');
     *   wassert(cured_utf16 == "very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-16 encoded string to match with.
     * @param size UTF-16 size of the other string, in bytes.
     * @param replacement The replacement character, in ASCII.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const wchar_t* text, const size_t size, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-16 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring other_string{L"funny"};
     *
     *   cured_utf16.censor(other_string, '-');
     *   wassert(cured_utf16 == L"very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-16 encoded string to match with.
     * @param replacement The replacement character, in ASCII.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const std::wstring& text, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-8 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.censor("funny", 0x2dUL);
     *   assert(cured_utf8 == "very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-8 encoded string to match with.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const char* text, const uint32_t replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-8 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.censor("funny", 5, 0x2dUL);
     *   assert(cured_utf8 == "very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-8 encoded string to match with.
     * @param size UTF-8 size of the other string, in bytes.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const char* text, const size_t size, const uint32_t replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-8 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string other_string{"funny"};
     *
     *   cured_utf8.censor(other_string, 0x2dUL);
     *   assert(cured_utf8 == "very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-8 encoded string to match with.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const std::string& text, const uint32_t replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-16 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.censor(L"funny", 0x2dUL);
     *   wassert(cured_utf16 == L"very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw null-terminated UTF-16 encoded string to match with.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const wchar_t* text, const uint32_t replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-16 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.censor(L"funny", 10, 0x2dUL);
     *   wassert(cured_utf16 == "very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The raw UTF-16 encoded string to match with.
     * @param size UTF-16 size of the other string, in bytes.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const wchar_t* text, const size_t size, const uint32_t replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified UTF-16 encoded string.
     * If you want to use a list of keywords, see censor_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring other_string{L"funny"};
     *
     *   cured_utf16.censor(other_string, 0x2dUL);
     *   wassert(cured_utf16 == L"very ----- text", "censor");
     *
     *   return 0;
     * }
     * ```
     *
     * @param text The UTF-16 encoded string to match with.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor_multiple
     * @throw decancer::error Thrown if the input string or replacement character is malformed.
     */
    void censor(const std::wstring& text, const uint32_t replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified list of UTF-8 keywords.
     * Unlike censor, this method also takes note of overlapping matches.
     * If you only want to use a single keyword, see censor.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.censor_multiple({"very", "funny"}, '-');
     *   assert(cured_utf8 == "---- ----- text", "censor multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-8 keywords to match with.
     * @param replacement The replacement character, in ASCII.
     * @see censor
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void censor_multiple(const std::initializer_list<const char*>& keywords, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified list of UTF-8 keywords.
     * Unlike censor, this method also takes note of overlapping matches.
     * If you only want to use a single keyword, see censor.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.censor_multiple({"very", "funny"}, 0x2dUL);
     *   assert(cured_utf8 == "---- ----- text", "censor multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-8 keywords to match with.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void censor_multiple(const std::initializer_list<const char*>& keywords, const uint32_t replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified list of UTF-8 keywords.
     * Unlike censor, this method also takes note of overlapping matches.
     * If you only want to use a single keyword, see censor.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string very{"very"};
     *   std::string funny{"funny"};
     *
     *   cured_utf8.censor_multiple({very, funny}, '-');
     *   assert(cured_utf8 == "---- ----- text", "censor multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-8 keywords to match with.
     * @param replacement The replacement character, in ASCII.
     * @see censor
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void censor_multiple(const std::initializer_list<std::string>& keywords, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified list of UTF-8 keywords.
     * Unlike censor, this method also takes note of overlapping matches.
     * If you only want to use a single keyword, see censor.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string very{"very"};
     *   std::string funny{"funny"};
     *
     *   cured_utf8.censor_multiple({very, funny}, 0x2dUL);
     *   assert(cured_utf8 == "---- ----- text", "censor multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-8 keywords to match with.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void censor_multiple(const std::initializer_list<std::string>& keywords, const uint32_t replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified list of UTF-16 keywords.
     * Unlike censor, this method also takes note of overlapping matches.
     * If you only want to use a single keyword, see censor.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.censor_multiple({L"very", L"funny"}, '-');
     *   wassert(cured_utf16 == L"---- ----- text", "censor multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-16 keywords to match with.
     * @param replacement The replacement character, in ASCII.
     * @see censor
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void censor_multiple(const std::initializer_list<const wchar_t*>& keywords, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified list of UTF-16 keywords.
     * Unlike censor, this method also takes note of overlapping matches.
     * If you only want to use a single keyword, see censor.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.censor_multiple({L"very", L"funny"}, 0x2dUL);
     *   wassert(cured_utf16 == L"---- ----- text", "censor multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-16 keywords to match with.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void censor_multiple(const std::initializer_list<const wchar_t*>& keywords, const uint32_t replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified list of UTF-16 keywords.
     * Unlike censor, this method also takes note of overlapping matches.
     * If you only want to use a single keyword, see censor.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring very{L"very"};
     *   std::wstring funny{L"funny"};
     *
     *   cured_utf16.censor_multiple({very, funny}, '-');
     *   wassert(cured_utf16 == L"---- ----- text", "censor multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-16 keywords to match with.
     * @param replacement The replacement character, in ASCII.
     * @see censor
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void censor_multiple(const std::initializer_list<std::wstring>& keywords, const char replacement) const;

    /**
     * @brief Censors every similar-looking match of the specified list of UTF-16 keywords.
     * Unlike censor, this method also takes note of overlapping matches.
     * If you only want to use a single keyword, see censor.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring very{L"very"};
     *   std::wstring funny{L"funny"};
     *
     *   cured_utf16.censor_multiple({very, funny}, 0x2dUL);
     *   wassert(cured_utf16 == L"---- ----- text", "censor multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-16 keywords to match with.
     * @param replacement The replacement character, as a unicode codepoint.
     * @see censor
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void censor_multiple(const std::initializer_list<std::wstring>& keywords, const uint32_t replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified UTF-8 encoded string with another UTF-8 encoded string.
     * If you want to replace a list of keywords, see replace_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.replace("very", "not");
     *   assert(cured_utf8 == "not funny text", "replace");
     *
     *   return 0;
     * }
     * ```
     *
     * @param find The raw null-terminated UTF-8 encoded string to match with.
     * @param replacement The raw null-terminated UTF-8 encoded string to replace with.
     * @see replace_multiple
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void replace(const char* find, const char* replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified UTF-8 encoded string with another UTF-8 encoded string.
     * If you want to replace a list of keywords, see replace_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.replace("very", 4, "not", 3);
     *   assert(cured_utf8 == "not funny text", "replace");
     *
     *   return 0;
     * }
     * ```
     *
     * @param find The raw UTF-8 encoded string to match with.
     * @param find_size UTF-8 size of the other string, in bytes.
     * @param replacement The raw UTF-8 encoded string to replace with.
     * @param replacement_size UTF-8 size of the replacement string, in bytes.
     * @see replace_multiple
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void replace(const char* find, const size_t find_size, const char* replacement, const size_t replacement_size) const;

    /**
     * @brief Replaces every similar-looking match of the specified UTF-8 encoded string with another UTF-8 encoded string.
     * If you want to replace a list of keywords, see replace_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string very{"very"};
     *   std::string not{"not"};
     *
     *   cured_utf8.replace(very, not);
     *   assert(cured_utf8 == "not funny text", "replace");
     *
     *   return 0;
     * }
     * ```
     *
     * @param find The UTF-8 encoded string to match with.
     * @param replacement The UTF-8 encoded string to replace with.
     * @see replace_multiple
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void replace(const std::string& find, const std::string& replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified UTF-16 encoded string with another UTF-16 encoded string.
     * If you want to replace a list of keywords, see replace_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.replace(L"very", L"not");
     *   wassert(cured_utf16 == L"not funny text", "replace");
     *
     *   return 0;
     * }
     * ```
     *
     * @param find The raw null-terminated UTF-16 encoded string to match with.
     * @param replacement The raw null-terminated UTF-16 encoded string to replace with.
     * @see replace_multiple
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void replace(const wchar_t* find, const wchar_t* replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified UTF-16 encoded string with another UTF-16 encoded string.
     * If you want to replace a list of keywords, see replace_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.replace(L"very", 8, L"not", 6);
     *   wassert(cured_utf16 == L"not funny text", "replace");
     *
     *   return 0;
     * }
     * ```
     *
     * @param find The raw UTF-16 encoded string to match with.
     * @param find_size UTF-16 size of the other string, in bytes.
     * @param replacement The raw UTF-16 encoded string to replace with.
     * @param replacement_size UTF-16 size of the replacement string, in bytes.
     * @see replace_multiple
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void replace(const wchar_t* find, const size_t find_size, const wchar_t* replacement, const size_t replacement_size) const;

    /**
     * @brief Replaces every similar-looking match of the specified UTF-16 encoded string with another UTF-16 encoded string.
     * If you want to replace a list of keywords, see replace_multiple.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring very{L"very"};
     *   std::wstring not{L"not"};
     *
     *   cured_utf16.replace(very, not);
     *   wassert(cured_utf16 == L"not funny text", "replace");
     *
     *   return 0;
     * }
     * ```
     *
     * @param find The UTF-16 encoded string to match with.
     * @param replacement The UTF-16 encoded string to replace with.
     * @see replace_multiple
     * @throw decancer::error Thrown if any of the arguments use an invalid encoding.
     */
    void replace(const std::wstring& find, const std::wstring& replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-8 keywords with another UTF-8 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.replace_multiple({"very", "not"}, "sussy");
     *   assert(cured_utf8 == "sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-8 keywords to match with.
     * @param replacement The raw null-terminated UTF-8 encoded string to replace with.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<const char*>& keywords, const char* replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-8 keywords with another UTF-8 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string very{"very"};
     *   std::string not{"not"};
     *
     *   cured_utf8.replace_multiple({very, not}, "sussy");
     *   assert(cured_utf8 == "sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-8 keywords to match with.
     * @param replacement The raw null-terminated UTF-8 encoded string to replace with.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<std::string>& keywords, const char* replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-8 keywords with another UTF-8 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *
     *   cured_utf8.replace_multiple({"very", "not"}, "sussy", 5);
     *   assert(cured_utf8 == "sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-8 keywords to match with.
     * @param replacement The raw UTF-8 encoded string to replace with.
     * @param replacement_size UTF-8 size of the replacement string, in bytes.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<const char*>& keywords, const char* replacement, const size_t replacement_size) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-8 keywords with another UTF-8 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string very{"very"};
     *   std::string not{"not"};
     *
     *   cured_utf8.replace_multiple({very, not}, "sussy", 5);
     *   assert(cured_utf8 == "sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-8 keywords to match with.
     * @param replacement The raw UTF-8 encoded string to replace with.
     * @param replacement_size UTF-8 size of the replacement string, in bytes.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<std::string>& keywords, const char* replacement, const size_t replacement_size) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-8 keywords with another UTF-8 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string sussy{"sussy"};
     *
     *   cured_utf8.replace_multiple({"very", "not"}, sussy);
     *   assert(cured_utf8 == "sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-8 keywords to match with.
     * @param replacement The UTF-8 encoded string to replace with.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<const char*>& keywords, const std::string& replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-8 keywords with another UTF-8 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define assert(expr, notes)                                    \
     *   if (!(expr)) {                                               \
     *     std::cerr << "assertion failure at " notes << std::endl;   \
     *     return 1;                                                  \
     *   }
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string very{"very"};
     *   std::string not{"not"};
     *   std::string sussy{"sussy"};
     *
     *   cured_utf8.replace_multiple({very, not}, sussy);
     *   assert(cured_utf8 == "sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-8 keywords to match with.
     * @param replacement The UTF-8 encoded string to replace with.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<std::string>& keywords, const std::string& replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-16 keywords with another UTF-16 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.replace_multiple({L"very", L"not"}, L"sussy");
     *   wassert(cured_utf16 == L"sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-16 keywords to match with.
     * @param replacement The raw null-terminated UTF-16 encoded string to replace with.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<const wchar_t*>& keywords, const wchar_t* replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-16 keywords with another UTF-16 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring very{L"very"};
     *   std::wstring not{L"not"};
     *
     *   cured_utf16.replace_multiple({very, not}, L"sussy");
     *   wassert(cured_utf16 == L"sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-16 keywords to match with.
     * @param replacement The raw null-terminated UTF-16 encoded string to replace with.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<std::wstring>& keywords, const wchar_t* replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-16 keywords with another UTF-16 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *
     *   cured_utf16.replace_multiple({L"very", L"not"}, L"sussy", 10);
     *   wassert(cured_utf16 == L"sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-16 keywords to match with.
     * @param replacement The raw UTF-16 encoded string to replace with.
     * @param replacement_size UTF-16 size of the replacement string, in bytes.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<const wchar_t*>& keywords, const wchar_t* replacement, const size_t replacement_size) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-16 keywords with another UTF-16 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring very{L"very"};
     *   std::wstring not{L"not"};
     *
     *   cured_utf16.replace_multiple({very, not}, L"sussy", 10);
     *   wassert(cured_utf16 == L"sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-16 keywords to match with.
     * @param replacement The raw UTF-16 encoded string to replace with.
     * @param replacement_size UTF-16 size of the replacement string, in bytes.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<std::wstring>& keywords, const wchar_t* replacement, const size_t replacement_size) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-16 keywords with another UTF-16 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring sussy{L"sussy"};
     *
     *   cured_utf16.replace_multiple({L"very", L"not"}, sussy);
     *   wassert(cured_utf16 == L"sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of raw null-terminated UTF-16 keywords to match with.
     * @param replacement The UTF-16 encoded string to replace with.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<const wchar_t*>& keywords, const std::wstring& replacement) const;

    /**
     * @brief Replaces every similar-looking match of the specified list of UTF-16 keywords with another UTF-16 encoded string.
     * Unlike replace, this method also takes note of overlapping matches.
     * If you only want to replace a single keyword, see replace.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * #define wassert(expr, notes)                                        \
     *   if (!(expr)) {                                                    \
     *     std::cerr << "wide assertion failure at " notes << std::endl;   \
     *     return 1;                                                       \
     *   }
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring very{L"very"};
     *   std::wstring not{L"not"};
     *   std::wstring sussy{L"sussy"};
     *
     *   cured_utf16.replace_multiple({very, not}, sussy);
     *   wassert(cured_utf16 == L"sussy sussy text", "replace_multiple");
     *
     *   return 0;
     * }
     * ```
     *
     * @param keywords A list of UTF-16 keywords to match with.
     * @param replacement The UTF-16 encoded string to replace with.
     * @see replace
     * @throw decancer::error Thrown if the input keywords or replacement character is malformed.
     */
    void replace_multiple(const std::initializer_list<std::wstring>& keywords, const std::wstring& replacement) const;

    /**
     * @brief Explicitly converts this cured string to a UTF-8 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * int main() {
     *   const char very_funny_text[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d, 0x94,
     *                                   0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99, 0x20, 0xc5,
     *                                   0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3, 0x00};
     *
     *   decancer::cured_string cured_utf8{very_funny_text};
     *   std::string cured_utf8_string = static_cast<std::wstring>(cured_utf16);
     *
     *   std::cout << cured_utf8_string << std::endl;
     *
     *   return 0;
     * }
     * ```
     *
     * @return std::string The UTF-8 encoded string representation of this cured string object.
     * @note It's NOT recommended to coerce this output to a C++ string
     * and process it manually from there, as decancer has its own
     * custom comparison measures, including leetspeak matching!
     */
    explicit operator std::string() const noexcept;

    /**
     * @brief Explicitly converts this cured string to a UTF-16 encoded string.
     *
     * Example:
     *
     * ```cpp
     * #include <decancer.hpp>
     * #include <iostream>
     *
     * int main() {
     *   const wchar_t wide_very_funny_text[] = {0x0076, 0xff25, 0x24e1, 0xd835, 0xdd02, 0x0020, 0xd835, 0xdd3d, 0xd835, 0xdd4c, 0x0147,
     *                                           0x2115, 0xff59, 0x0020, 0x0163, 0x4e47, 0xd835, 0xdd4f, 0xd835, 0xdce3, 0x0000};
     *
     *   decancer::cured_string cured_utf16{wide_very_funny_text};
     *   std::wstring cured_utf16_string = static_cast<std::wstring>(cured_utf16);
     *
     *   std::wcout << cured_utf16_string << std::endl;
     *
     *   return 0;
     * }
     * ```
     *
     * @return std::wstring The UTF-16 encoded string representation of this cured string object.
     * @note It's NOT recommended to coerce this output to a C++ string
     * and process it manually from there, as decancer has its own
     * custom comparison measures, including leetspeak matching!
     */
    explicit operator std::wstring() const noexcept;

    /**
     * @brief Frees the cured string object.
     */
    ~cured_string() noexcept;
  };
}; // namespace decancer

#undef DECANCER_CXX_EXPORT
#undef DECANCER_REPLACE_MULTIPLE_METHOD
#undef DECANCER_REPLACE_MULTIPLE_WIDE_METHOD