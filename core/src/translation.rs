use crate::{
  codepoints::CODEPOINTS,
  similar::{self, SIMILAR_END as STRINGS_OFFSET},
};
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Translation {
  /// A single unicode character.
  Character(char),
  /// An [ASCII](https://en.wikipedia.org/wiki/ASCII) string slice.
  String(&'static str),
  /// This suggests that the translation is an empty string. You can get this when the input character is a [control character](https://en.wikipedia.org/wiki/Control_character), [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates), [combining character](https://en.wikipedia.org/wiki/Script_(Unicode)#Special_script_property_values) (e.g diacritics), [private use character](https://en.wikipedia.org/wiki/Private_Use_Areas), [byte order character](https://en.wikipedia.org/wiki/Byte_order_mark), or any invalid unicode value (e.g beyond [`char::MAX`]).
  None,
}

impl Translation {
  pub(crate) const fn string(integer: u32, second_byte: u8) -> Self {
    unsafe {
      Self::String(str::from_utf8_unchecked(slice::from_raw_parts(
        CODEPOINTS.offset(
          (STRINGS_OFFSET + (((((integer >> 20) as u16) & 0x07) << 8) | (second_byte as u16))) as _,
        ),
        ((integer >> 23) & 0x1f) as _,
      )))
    }
  }

  pub(crate) const fn character(code: u32) -> Self {
    Self::Character(unsafe { transmute(code) })
  }

  #[cfg(feature = "std")]
  pub(crate) fn add_to(self, other: &mut String) {
    match self {
      Self::Character(ch) => other.push(ch),
      Self::String(s) => other.push_str(s),
      Self::None => {}
    }
  }
}

/// Checks if this [`Translation`] is ***similar*** into another string.
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
    char::deserialize(deserializer).map(crate::cure_char)
  }
}
