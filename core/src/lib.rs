#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod codepoints;
mod similar;
#[cfg(feature = "std")]
mod string;
mod translation;
mod util;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use string::CuredString;
pub use translation::Translation;

use codepoints::{Codepoint, CASE_SENSITIVE_CODEPOINTS_COUNT, CODEPOINTS_COUNT};
use core::cmp::Ordering;

/// Cures a single character/unicode codepoint.
///
/// # Examples
///
/// Most of the time, this would yield only a single unicode character:
///
/// ```rust
/// use decancer::Translation;
///
/// let cured_e = decancer::cure_char('Ｅ');
///
/// assert!(matches!(cured_e, Translation::Character('e')));
/// ```
///
/// However, for several special cases, it would yield an [ASCII](https://en.wikipedia.org/wiki/ASCII) [`&'static str`][prim@str]:
///
/// ```rust
/// use decancer::Translation;
///
/// let cured_ae = decancer::cure_char('ӕ');
///
/// assert!(matches!(cured_ae, Translation::String("ae")));
/// ```
///
/// If your unicode character is a [control character](https://en.wikipedia.org/wiki/Control_character), [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates), [combining character](https://en.wikipedia.org/wiki/Script_(Unicode)#Special_script_property_values), [private use character](https://en.wikipedia.org/wiki/Private_Use_Areas), [byte order character](https://en.wikipedia.org/wiki/Byte_order_mark), or any invalid unicode value (e.g beyond [`char::MAX`]), you would get [`None`][Translation::None]:
///
/// ```rust
/// use decancer::Translation;
///
/// let cured_surrogate = decancer::cure_char(0xD800u32);
///
/// assert!(matches!(cured_surrogate, Translation::None));
/// ```
pub fn cure_char<C>(code: C) -> Translation
where
  C: Into<u32>,
{
  let code = code.into();

  if code <= 31 || code == 127 || (0xD800..=0xF8FF).contains(&code) || code >= 0xE0100 {
    return Translation::None;
  }

  let code_lowercased = unsafe {
    char::from_u32_unchecked(code)
      .to_lowercase()
      .next()
      .unwrap_unchecked() as _
  };

  if code_lowercased < 0x80 {
    return Translation::character(code_lowercased);
  }

  let mut start = 0;
  let mut end = CASE_SENSITIVE_CODEPOINTS_COUNT;

  if code != code_lowercased {
    while start <= end {
      let mid = (start + end) / 2;
      let codepoint = Codepoint::case_sensitive_at(mid);

      match codepoint.matches(code) {
        Ordering::Equal => return codepoint.translation(code),
        Ordering::Greater => start = mid + 1,
        _ => end = mid - 1,
      };
    }

    start = 0;
  }

  end = CODEPOINTS_COUNT;

  while start <= end {
    let mid = (start + end) / 2;
    let codepoint = Codepoint::at(mid);

    match codepoint.matches(code_lowercased) {
      Ordering::Equal => return codepoint.translation(code_lowercased),
      Ordering::Greater => start = mid + 1,
      _ => end = mid - 1,
    };
  }

  Translation::character(code_lowercased)
}

/// Cures a string.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// // cured here is a decancer::CuredString struct wrapping over the cured string
/// // for comparison purposes, it's more recommended to use the methods provided by the decancer::CuredString struct.
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.contains("funny"));
/// assert!(cured.ends_with("text"));
///
/// // retrieve the String inside and consume the struct.
/// let _output_str = cured.into_str();
/// ```
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[inline(always)]
pub fn cure<S>(input: &S) -> CuredString
where
  S: AsRef<str> + ?Sized,
{
  input.as_ref().chars().collect()
}
