#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod codepoints;
mod similar;
#[cfg(feature = "std")]
mod string;
#[cfg(test)]
mod test;
mod translation;
mod util;

#[cfg(feature = "std")]
pub use string::CuredString;
pub use translation::Translation;

use codepoints::{
  Codepoint, CASE_SENSITIVE_CODEPOINTS_COUNT, CASE_SENSITIVE_CODEPOINTS_OFFSET, CODEPOINTS_COUNT,
};
use core::cmp::Ordering;

const fn translate(code: u32, offset: i32, mut end: i32) -> Option<Translation> {
  let mut start = 0;

  while start <= end {
    let mid = (start + end) / 2;
    let codepoint = Codepoint::at(offset + (mid * 5));

    match codepoint.matches(code) {
      Ordering::Equal => return Some(codepoint.translation(code)),
      Ordering::Greater => start = mid + 1,
      Ordering::Less => end = mid - 1,
    };
  }

  None
}

/// Cures a single character/unicode codepoint.
///
/// # Examples
///
/// Most of the time, this would yield only a single unicode character:
///
/// ```rust
/// use decancer::Translation;
///
/// assert!(matches!(decancer::cure_char('Ｅ'), Translation::Character('e')));
/// ```
///
/// However, for several special cases, it would yield an [ASCII](https://en.wikipedia.org/wiki/ASCII) [`&'static str`][prim@str]:
///
/// ```rust
/// use decancer::Translation;
///
/// assert!(matches!(decancer::cure_char('æ'), Translation::String("ae")));
/// assert!(matches!(decancer::cure_char('ĳ'), Translation::String("ij")));
/// assert!(matches!(decancer::cure_char('œ'), Translation::String("oe")));
/// assert!(matches!(decancer::cure_char('🆐'), Translation::String("dj")));
/// assert!(matches!(decancer::cure_char('🆑'), Translation::String("cl")));
/// assert!(matches!(decancer::cure_char('🆔'), Translation::String("id")));
/// assert!(matches!(decancer::cure_char('🆖'), Translation::String("ng")));
/// assert!(matches!(decancer::cure_char('🆗'), Translation::String("ok")));
/// assert!(matches!(decancer::cure_char('🆚'), Translation::String("vs")));
/// assert!(matches!(decancer::cure_char('🜀'), Translation::String("qe")));
/// assert!(matches!(decancer::cure_char('🜇'), Translation::String("ar")));
///
/// assert!(matches!(decancer::cure_char('⅓'), Translation::String("1/3")));
/// assert!(matches!(decancer::cure_char('⅔'), Translation::String("2/3")));
/// assert!(matches!(decancer::cure_char('⅕'), Translation::String("1/5")));
/// assert!(matches!(decancer::cure_char('⅖'), Translation::String("2/5")));
/// assert!(matches!(decancer::cure_char('⅗'), Translation::String("3/5")));
/// assert!(matches!(decancer::cure_char('⅘'), Translation::String("4/5")));
/// assert!(matches!(decancer::cure_char('㋍'), Translation::String("erg")));
/// assert!(matches!(decancer::cure_char('㋏'), Translation::String("ltd")));
///
/// assert!(matches!(decancer::cure_char('㍴'), Translation::String("bar")));
/// assert!(matches!(decancer::cure_char('㎈'), Translation::String("cal")));
/// assert!(matches!(decancer::cure_char('㎭'), Translation::String("rad")));
/// assert!(matches!(decancer::cure_char('㏇'), Translation::String("co.")));
/// assert!(matches!(decancer::cure_char('㏒'), Translation::String("log")));
/// assert!(matches!(decancer::cure_char('㏕'), Translation::String("mil")));
/// assert!(matches!(decancer::cure_char('㏖'), Translation::String("mol")));
/// assert!(matches!(decancer::cure_char('㏙'), Translation::String("ppm")));
/// ```
///
/// If your unicode character is a [control character](https://en.wikipedia.org/wiki/Control_character), [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates), [combining character](https://en.wikipedia.org/wiki/Script_(Unicode)#Special_script_property_values) (e.g diacritics), [private use character](https://en.wikipedia.org/wiki/Private_Use_Areas), [byte order character](https://en.wikipedia.org/wiki/Byte_order_mark), or any invalid unicode value (e.g beyond [`char::MAX`]), you would get [`None`][Translation::None]:
///
/// ```rust
/// use decancer::Translation;
///
/// assert!(matches!(decancer::cure_char(0xD800u32), Translation::None));
/// assert!(matches!(decancer::cure_char(char::REPLACEMENT_CHARACTER), Translation::None));
/// assert!(matches!(decancer::cure_char((char::MAX as u32) + 1), Translation::None));
/// ```
pub fn cure_char<C: Into<u32>>(code: C) -> Translation {
  let code = code.into();

  if matches!(code, 0..=31 | 127 | 0xd800..=0xf8ff | 0xe0100..) {
    return Translation::None;
  }

  // SAFETY: even if there is no lowercase mapping for some codepoints, it would just return itself.
  // therefore, the first iteration and/or codepoint always exists.
  let code_lowercased = unsafe {
    char::from_u32_unchecked(code)
      .to_lowercase()
      .next()
      .unwrap_unchecked() as _
  };

  if code_lowercased < 0x80 {
    return Translation::character(code_lowercased);
  } else if code != code_lowercased {
    if let Some(translation) = translate(
      code,
      CASE_SENSITIVE_CODEPOINTS_OFFSET as _,
      CASE_SENSITIVE_CODEPOINTS_COUNT as _,
    ) {
      return translation;
    }
  }

  translate(code_lowercased, 6, CODEPOINTS_COUNT as _)
    .unwrap_or_else(|| Translation::character(code_lowercased))
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
#[inline(always)]
pub fn cure(input: &str) -> CuredString {
  input.chars().map(cure_char).collect()
}
