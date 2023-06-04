use crate::{
  codepoints::CODEPOINTS,
  cure_char,
  similar::{self, SIMILAR_END as STRINGS_OFFSET},
};
#[cfg(feature = "std")]
use core::ops::{Add, AddAssign};
use core::{
  cmp::PartialEq,
  fmt::{self, Debug, Display},
  mem::transmute,
  slice, str,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The translation for a single character/codepoint.
#[must_use]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Translation {
  /// A single unicode character.
  Character(char),
  /// A multi-character [ASCII](https://en.wikipedia.org/wiki/ASCII) string.
  String(&'static str),
  /// This suggests that the translation is an empty string. You can get this when the input character is a [control character](https://en.wikipedia.org/wiki/Control_character), [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates), [combining character](https://en.wikipedia.org/wiki/Script_(Unicode)#Special_script_property_values), [private use character](https://en.wikipedia.org/wiki/Private_Use_Areas), [byte order character](https://en.wikipedia.org/wiki/Byte_order_mark), or any invalid unicode value (e.g beyond [`char::MAX`]).
  None,
}

impl Translation {
  pub(crate) const fn string(integer: u32, second_byte: u8) -> Self {
    unsafe {
      Self::String(str::from_utf8_unchecked(slice::from_raw_parts(
        CODEPOINTS.offset(
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

/// Checks if this [`Translation`] is ***similar*** into another string.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured = decancer::cure_char('Ｅ');
///
/// assert_eq!(cured, "e");
/// ```
///
/// And since it checks if the strings are similar, please note that this is valid too:
///
/// ```rust
/// let cured = decancer::cure_char('Ｅ');
///
/// // it assumes that e is similar to 3
/// assert_eq!(cured, "3");
/// ```
impl<S> PartialEq<S> for Translation
where
  S: AsRef<str> + ?Sized,
{
  #[must_use]
  fn eq(&self, o: &S) -> bool {
    let o = o.as_ref();

    match self {
      Self::Character(ch) => {
        let mut chars = o.chars();

        chars
          .next()
          .map(|next_char| chars.next().is_none() && similar::is(*ch as _, next_char))
          .unwrap_or_default()
      }

      Self::String(s) => s.len() == o.len() && similar::is_str(s, o),
      _ => o.is_empty(),
    }
  }
}

impl<C> From<C> for Translation
where
  C: Into<u32>,
{
  #[inline(always)]
  fn from(ch: C) -> Self {
    cure_char(ch)
  }
}

impl Display for Translation {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Character(ch) => Display::fmt(ch, f),
      Self::String(s) => Display::fmt(s, f),
      _ => Ok(()),
    }
  }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl Add<Translation> for String {
  type Output = Self;

  #[inline(always)]
  fn add(mut self, rhs: Translation) -> Self::Output {
    self += rhs;
    self
  }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl AddAssign<Translation> for String {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Translation) {
    match rhs {
      Translation::Character(c) => self.push(c),
      Translation::String(s) => self.push_str(s),
      _ => {}
    }
  }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl Extend<Translation> for String {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = Translation>,
  {
    for part in iter {
      *self += part;
    }
  }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[allow(clippy::from_over_into)]
impl Into<Option<String>> for Translation {
  #[inline(always)]
  fn into(self) -> Option<String> {
    match self {
      Self::None => None,
      _ => Some(self.to_string()),
    }
  }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl FromIterator<Translation> for String {
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = Translation>,
  {
    let iter = iter.into_iter();
    let (size_hint, _) = iter.size_hint();

    let mut s = String::with_capacity(size_hint);
    s.extend(iter);

    s
  }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Translation {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::Character(ch) => serializer.serialize_char(*ch),
      Self::String(s) => serializer.serialize_str(s),
      _ => serializer.serialize_unit(),
    }
  }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Translation {
  #[inline(always)]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    char::deserialize(deserializer).map(cure_char)
  }
}
