use crate::{confusables::CONFUSABLES, similar::SIMILAR_END as STRINGS_OFFSET};
use core::{cmp::PartialEq, fmt, mem::transmute, ops::AddAssign, slice, str};

/// The translation for a single character/confusable.
pub enum Translation {
  /// A single unicode character.
  Character(char),
  /// A multi-character ASCII string.
  String(&'static str),
  /// This suggests that the translation is an empty string.
  /// You can get this when the input character is a [control character](https://en.wikipedia.org/wiki/Control_character).
  None,
}

impl Translation {
  pub(crate) const fn string(integer: u32, second_byte: u8) -> Self {
    unsafe {
      Self::String(str::from_utf8_unchecked(slice::from_raw_parts(
        CONFUSABLES.offset(
          (STRINGS_OFFSET + (((((integer >> 21) as u16) & 0x0f) << 8) | (second_byte as u16))) as _,
        ),
        ((integer >> 25) & 0x0f) as _,
      )))
    }
  }

  pub(crate) const fn character(code: u32) -> Self {
    Self::Character(unsafe { transmute(code) })
  }
}

impl AddAssign<Translation> for String {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Translation) {
    match rhs {
      Translation::Character(c) => self.push(c),
      Translation::String(s) => self.push_str(s),
      Translation::None => {}
    }
  }
}

impl<S> PartialEq<S> for Translation
where
  S: AsRef<str> + ?Sized,
{
  fn eq(&self, other: &S) -> bool {
    let o = other.as_ref();

    match self {
      Translation::Character(ch) => {
        let mut chars = o.chars();

        if let Some(next_char) = chars.next() {
          next_char == *ch && chars.next().is_none()
        } else {
          false
        }
      }

      Translation::String(s) => &o == s,
      Translation::None => o.is_empty(),
    }
  }
}

impl fmt::Debug for Translation {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\"")?;
    fmt::Display::fmt(self, f)?;
    write!(f, "\"")
  }
}

impl fmt::Display for Translation {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Translation::Character(ch) => write!(f, "{ch}"),
      Translation::String(s) => write!(f, "{s}"),
      Translation::None => fmt::Result::Ok(()),
    }
  }
}
