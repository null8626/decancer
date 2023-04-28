#![doc = include_str!("../README.md")]

mod confusables;
mod similar;
mod string;
mod translation;
mod util;

#[cfg(test)]
mod tests;

use core::cmp::Ordering;

/// A small wrapper around the [`String`] datatype for comparison purposes.
pub use string::CuredString;

/// The translation for a single character/confusable.
pub use translation::Translation;

/// Cures a single character.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured_e = decancer::cure_char('Ｅ');
///
/// match cured_e {
///   decancer::Translation::Character(e) => assert_eq!(e, 'e'),
///   _ => unreachable!(),
/// }
///
/// let cured_ae = decancer::cure_char('ӕ');
///
/// match cured_ae {
///   decancer::Translation::String(ae) => assert_eq!(ae, "ae"),
///   _ => unreachable!(),
/// }
///
/// // control characters, surrogates, combining characters, private use characters, byte order characters, etc.
/// let cured_surrogate = decancer::cure_char(0xD800u32);
///
/// assert!(matches!(cured_surrogate, decancer::Translation::None));
/// ```
#[must_use]
pub fn cure_char<C>(code: C) -> Translation
where
  C: Into<u32>,
{
  let code = code.into();

  if code <= 31
    || code == 127
    || (0xD800..=0xF8FF).contains(&code)
    || (0xE0100..=0xE01EF).contains(&code)
    || code >= 0xF0000
  {
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
  let mut end = confusables::CASE_SENSITIVE_CONFUSABLES_COUNT;

  if code != code_lowercased {
    while start <= end {
      let mid = (start + end) / 2;
      let confusable = confusables::Confusable::case_sensitive_at(mid);

      match confusable.matches(code) {
        Ordering::Equal => return confusable.translation(code),
        Ordering::Greater => start = mid + 1,
        Ordering::Less => end = mid - 1,
      };
    }

    start = 0;
  }

  end = confusables::CONFUSABLES_COUNT;

  while start <= end {
    let mid = (start + end) / 2;
    let confusable = confusables::Confusable::at(mid);

    match confusable.matches(code_lowercased) {
      Ordering::Equal => return confusable.translation(code_lowercased),
      Ordering::Greater => start = mid + 1,
      Ordering::Less => end = mid - 1,
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
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.ends_with("text"));
/// assert!(cured.contains("funny"));
/// ```
#[must_use]
pub fn cure<S>(input: &S) -> CuredString
where
  S: AsRef<str> + ?Sized,
{
  let input = input.as_ref();

  CuredString(input.chars().map(cure_char).fold(
    String::with_capacity(input.len()),
    |mut res, acc| {
      res += acc;
      res
    },
  ))
}
