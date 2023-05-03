use crate::{
  confusables::CONFUSABLES,
  cure_char,
  similar::{self, SIMILAR_END as STRINGS_OFFSET},
};
use core::{cmp::PartialEq, fmt, mem::transmute, slice, str};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The translation for a single character/confusable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Translation {
  /// A single unicode character.
  Character(char),
  /// A multi-character [ASCII](https://en.wikipedia.org/wiki/ASCII) string.
  String(&'static str),
  /// This suggests that the translation is an empty string.
  /// You can get this when the input character is a [control character](https://en.wikipedia.org/wiki/Control_character), [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates), [combining character](https://en.wikipedia.org/wiki/Script_(Unicode)#Special_script_property_values), [private use character](https://en.wikipedia.org/wiki/Private_Use_Areas), [byte order character](https://en.wikipedia.org/wiki/Byte_order_mark), or any invalid unicode value (e.g beyond [`char::MAX`]).
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

/// Checks if this [`Translation`] is ***similar*** to another string.
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
  fn eq(&self, other: &S) -> bool {
    let o = other.as_ref();

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

/// Alias for [`cure_char`].
impl<C> From<C> for Translation
where
  C: Into<u32>,
{
  #[inline(always)]
  fn from(ch: C) -> Self {
    cure_char(ch)
  }
}

/// Formats this `Translation`. Behaves like formatting your typical `String`.
impl fmt::Display for Translation {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Character(ch) => fmt::Display::fmt(ch, f),
      Self::String(s) => fmt::Display::fmt(s, f),
      _ => fmt::Result::Ok(()),
    }
  }
}

/// A helper implementation for appending a [`Translation`] to a [`String`].
///
/// # Examples
///
/// Basic usage:
///
/// > **note:** this is for demonstration purposes only - in production, it's more recommended to use [`decancer::cure`][crate::cure].
///
/// ```rust
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = String::with_capacity(text.len());
///
/// for cured_char in text.chars().map(decancer::cure_char) {
///   cured += cured_char;
/// }
///
/// // note: direct comparisons like this are not recommended, please use a decancer::CuredString struct.
/// assert_eq!(cured, "very funny text");
/// ```
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl core::ops::AddAssign<Translation> for String {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Translation) {
    match rhs {
      Translation::Character(c) => self.push(c),
      Translation::String(s) => self.push_str(s),
      _ => {}
    }
  }
}

/// Coerces this [`Translation`] to an [`Option<String>`][Option].
///
/// # Examples
///
/// A non-[`Translation::None`] value would yield a [`Some(String)`][Option::Some]:
///
/// ```rust
/// use decancer::Translation;
///
/// let cured_e = decancer::cure_char('Ｅ');
/// assert!(matches!(cured_e, Translation::Character('e')));
///
/// let cured_e: Option<String> = cured_e.into();
/// assert_eq!(cured_e, Some(String::from("e")));
/// ```
///
/// Otherwise, a [`Translation::None`] value would yield a [`None`][Option::None]:
///
/// ```rust
/// use decancer::Translation;
///
/// let cured_surrogate = decancer::cure_char(0xD800u32);
/// assert!(matches!(cured_surrogate, Translation::None));
///
/// let cured_surrogate: Option<String> = cured_surrogate.into();
/// assert!(cured_surrogate.is_none());
/// ```
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

/// A helper implementation for joining several [`Translation`]s into one [`String`].
///
/// # Examples
///
/// Basic usage:
///
/// > **note:** this is for demonstration purposes only - in production, it's more recommended to use [`decancer::cure`][crate::cure].
///
/// ```rust
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let cured = text.chars().map(decancer::cure_char).collect::<String>();
///
/// // note: direct comparisons like this are not recommended, please use a decancer::CuredString struct.
/// assert_eq!(cured, "very funny text");
/// ```
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl FromIterator<Translation> for String {
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = Translation>,
  {
    let iter = iter.into_iter();
    let (size_hint, _) = iter.size_hint();

    iter.fold(String::with_capacity(size_hint), |mut res, acc| {
      res += acc;
      res
    })
  }
}

/// A helper implementation for joining several [`Translation`]s into one [`CuredString`][crate::CuredString].
///
/// # Examples
///
/// Basic usage:
///
/// > **note:** this is for demonstration purposes only - in production, it's more recommended to use [`decancer::cure`][crate::cure].
///
/// ```rust
/// use decancer::CuredString;
///
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let cured = text.chars().map(decancer::cure_char).collect::<CuredString>();
///
/// assert_eq!(cured, "very funny text");
/// ```
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl FromIterator<Translation> for crate::CuredString {
  #[inline(always)]
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = Translation>,
  {
    Self(iter.into_iter().collect())
  }
}

/// Serializes this [`Translation`].
///
/// - A [`Translation::Character`] would serialize into a [`character`][Serializer::serialize_char].
/// - A [`Translation::String`] would serialize into a [`string`][Serializer::serialize_str].
/// - A [`Translation::None`] would serialize into a [`unit`][Serializer::serialize_unit].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use decancer::Translation;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Decancered {
///   translation: Translation,
/// }
///
/// let decancered = Decancered {
///   translation: decancer::cure_char('ӕ')
/// };
///
/// assert_eq!(serde_json::to_string(&decancered).unwrap(), r#"{"translation":"ae"}"#);
/// ```
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

/// Deserializes and [cures][cure_char] a [`character`][Deserializer::deserialize_char].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use decancer::Translation;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Decancered {
///   translation: Translation,
/// }
///
/// let json = r#"{"translation": "ӕ"}"#;
/// let decancered: Decancered = serde_json::from_str(json).unwrap();
///
/// assert!(matches!(decancered.translation, Translation::String("ae")));
/// ```
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
