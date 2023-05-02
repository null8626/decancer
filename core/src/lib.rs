#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod confusables;
mod similar;
#[cfg(feature = "std")]
mod string;
#[cfg(feature = "std")]
#[cfg(test)]
mod tests;
mod translation;
mod util;

#[cfg(feature = "std")]
pub use string::CuredString;
pub use translation::Translation;

use core::cmp::Ordering;
#[cfg(feature = "std")]
use std::io::{self, ErrorKind, Read};

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
/// assert!(matches!(cured_e, Translation::Character('e')));
/// ```
///
/// However, for several special cases, it would yield an [ASCII](https://en.wikipedia.org/wiki/ASCII) string in the form of a [`&'static str`][str]:
///
/// ```rust
/// use decancer::Translation;
///
/// let cured_ae = decancer::cure_char('ӕ');
/// assert!(matches!(cured_ae, Translation::String("ae")));
/// ```
///
/// If your unicode character is a [control character](https://en.wikipedia.org/wiki/Control_character), [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates), [combining character](https://en.wikipedia.org/wiki/Script_(Unicode)#Special_script_property_values), [private use character](https://en.wikipedia.org/wiki/Private_Use_Areas), [byte order character](https://en.wikipedia.org/wiki/Byte_order_mark), or any invalid unicode value (e.g beyond [`char::MAX`]), you would get [`None`][Translation::None]:
///
/// ```rust
/// use decancer::Translation;
///
/// let cured_surrogate = decancer::cure_char(0xD800u32);
/// assert!(matches!(cured_surrogate, Translation::None));
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
#[cfg(feature = "std")]
#[must_use]
#[inline(always)]
pub fn cure<S>(input: &S) -> CuredString
where
  S: AsRef<str> + ?Sized,
{
  input.as_ref().chars().map(cure_char).collect()
}

#[cfg(feature = "std")]
#[allow(invalid_value, clippy::uninit_assumed_init)]
fn cure_next_bytes<R>(reader: &mut R) -> io::Result<Option<Translation>>
where
  R: Read,
{
  let mut first = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

  if let Err(err) = reader.read_exact(core::slice::from_mut(&mut first)) {
    return match err.kind() {
      ErrorKind::UnexpectedEof => Ok(None),
      _ => Err(err),
    };
  }

  let mut output = first as u32;

  if 0xF0 == (0xF8 & first) {
    let mut rest: [u8; 3] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    reader.read_exact(&mut rest)?;

    output = ((0x07 & first as u32) << 18)
      | ((0x3F & rest[0] as u32) << 12)
      | ((0x3F & rest[1] as u32) << 6)
      | (0x3F & rest[2] as u32);
  } else if 0xE0 == (0xf0 & first) {
    let mut rest: [u8; 2] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    reader.read_exact(&mut rest)?;

    output =
      ((0x0F & first as u32) << 12) | ((0x3F & rest[0] as u32) << 6) | (0x3F & rest[1] as u32);
  } else if 0xC0 == (0xE0 & first) {
    let mut next = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    reader.read_exact(core::slice::from_mut(&mut next))?;

    output = ((0x1F & first as u32) << 6) | (0x3F & next as u32);
  }

  Ok(Some(cure_char(unsafe { char::from_u32_unchecked(output) })))
}

/// Cures bytes from a reader. This can be a [`File`][std::fs::File], [`BufReader`][io::BufReader], [`Cursor`][io::Cursor], or any data type that implements [`Read`].
///
/// # Safety
///
/// This function assumes that the stream of bytes coming are already valid [UTF-8](https://en.wikipedia.org/wiki/UTF-8). Therefore, [UTF-8](https://en.wikipedia.org/wiki/UTF-8) validity will **NOT** be checked unless the reader EOFs prematurely (see [`UnexpectedEof`][ErrorKind::UnexpectedEof]).
///
/// # Errors
///
/// Errors only if the reader [ends prematurely][ErrorKind::UnexpectedEof] or [fails][io::Error].
///
/// # Examples
///
/// From an in-memory buffer with a [`Cursor`][io::Cursor]:
///
/// ```rust
/// use std::io::Cursor;
///
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let reader = Cursor::new(text.as_bytes());
/// let cured = decancer::cure_reader(reader).unwrap();
///
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.ends_with("text"));
/// assert!(cured.contains("funny"));
/// ```
///
/// From a [`File`][std::fs::File] through a [`BufReader`][io::BufReader]:
///
/// ```rust,ignore
/// use std::{fs::File, io::BufReader};
///
/// let reader = BufReader::new(File::open("cancer.txt").unwrap());
/// let cured = decancer::cure_reader(reader).unwrap();
///
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.ends_with("text"));
/// assert!(cured.contains("funny"));
/// ```
#[cfg(feature = "std")]
pub fn cure_reader<R>(mut reader: R) -> io::Result<CuredString>
where
  R: Read,
{
  let mut output = String::new();

  while let Some(next) = cure_next_bytes(&mut reader)? {
    output += next;
  }

  Ok(CuredString(output))
}