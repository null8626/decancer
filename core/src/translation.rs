// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

#[cfg(feature = "options")]
use super::util::is_alphanumeric;
use super::{
  codepoints::CODEPOINTS,
  similar::{self, SIMILAR_END as STRINGS_OFFSET},
  Matcher,
};
use std::{
  borrow::Cow,
  cmp::PartialEq,
  fmt::{self, Debug, Display},
  ops::{Add, AddAssign},
  str,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The translation for a single character/codepoint.
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Translation {
  /// A single unicode character.
  Character(char),
  /// A string.
  String(Cow<'static, str>),
  /// This suggests that the translation is an empty string. You can get this when the input character is a [control character](https://en.wikipedia.org/wiki/Control_character), [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates), [combining character](https://en.wikipedia.org/wiki/Script_(Unicode)#Special_script_property_values) (e.g diacritics), [private use character](https://en.wikipedia.org/wiki/Private_Use_Areas), [byte order character](https://en.wikipedia.org/wiki/Byte_order_mark), or any invalid unicode value (e.g beyond [`char::MAX`]).
  None,
}

impl Translation {
  pub(super) fn string(integer: u32, second_byte: u8) -> Self {
    Self::String(Cow::Borrowed(
      str::from_utf8(CODEPOINTS.sliced(
        (STRINGS_OFFSET + (((((integer >> 20) as u16) & 0x07) << 8) | (second_byte as u16))) as _,
        ((integer >> 23) & 0x1f) as _,
      ))
      .unwrap(),
    ))
  }

  #[inline(always)]
  pub(super) fn character(code: u32) -> Self {
    Self::Character(char::from_u32(code).unwrap())
  }

  #[cfg(feature = "options")]
  pub(super) fn make_uppercase(&mut self) {
    match self {
      Self::Character(c) => *c = c.to_uppercase().next().unwrap_or(*c),

      Self::String(s) => s.to_mut().make_ascii_uppercase(),

      Self::None => {},
    }
  }

  #[cfg(feature = "options")]
  fn is_ascii(&self) -> bool {
    match self {
      Self::Character(c) => (*c as u32) <= 0x7f,

      Self::String(s) => s.is_ascii(),

      Self::None => false,
    }
  }

  #[cfg(feature = "options")]
  fn is_alphanumeric(&self) -> bool {
    match self {
      Self::Character(c) => is_alphanumeric(*c as _),

      Self::String(s) => s.bytes().all(|b| is_alphanumeric(b as _)),

      Self::None => false,
    }
  }

  #[cfg(feature = "options")]
  pub(super) fn ensure_stripped_if(self, ascii_only: bool, alphanumeric_only: bool) -> Self {
    if (ascii_only && !self.is_ascii()) || (alphanumeric_only && !self.is_alphanumeric()) {
      Self::None
    } else {
      self
    }
  }
}

impl From<Translation> for Cow<'static, str> {
  fn from(translation: Translation) -> Self {
    match translation {
      Translation::Character(c) => Self::Owned(String::from(c)),

      Translation::String(s) => s,

      Translation::None => Self::Borrowed(""),
    }
  }
}

impl Add<Translation> for String {
  type Output = String;

  #[inline(always)]
  fn add(mut self, translation: Translation) -> Self::Output {
    self += translation;
    self
  }
}

impl AddAssign<Translation> for String {
  fn add_assign(&mut self, translation: Translation) {
    match translation {
      Translation::Character(ch) => self.push(ch),

      Translation::String(s) => self.push_str(&s),

      Translation::None => {},
    }
  }
}

/// Checks if this [`Translation`] is similar with another string.
///
/// This comparison is *case-insensitive*.
impl<S> PartialEq<S> for Translation
where
  S: AsRef<str> + ?Sized,
{
  fn eq(&self, o: &S) -> bool {
    let o = o.as_ref();

    match self {
      Self::Character(ch) => {
        let mut chars = o.chars();

        chars.next().map_or_else(Default::default, |next_char| {
          chars.next().is_none() && similar::is(*ch as _, next_char)
        })
      },

      Self::String(s) => Matcher::is_equal(s, o),
      Self::None => o.is_empty(),
    }
  }
}

impl Display for Translation {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Character(ch) => Display::fmt(ch, f),

      Self::String(s) => Display::fmt(s, f),

      Self::None => Ok(()),
    }
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

      Self::None => serializer.serialize_unit(),
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
    char::deserialize(deserializer).map(|character| crate::cure_char!(character))
  }
}
