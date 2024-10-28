use crate::similar;
use core::{
  cmp::PartialEq,
  fmt::{self, Debug, Display, Formatter},
  mem::transmute,
  ops::Deref,
};
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// A small wrapper around the [`String`] data type for comparison purposes.
///
/// This is used because imperfections from translations can happen, thus this is used to provide comparison functions that are not as strict and can detect similar-looking characters (e.g: `i` and `l`)
#[must_use]
#[derive(Clone, Eq)]
pub struct CuredString(pub(crate) String);

impl CuredString {
  /// Coerces this [`CuredString`] into a [`String`].
  ///
  /// > **NOTE:** It's highly **NOT** recommended to use Rust's comparison methods after calling this.
  /// > The string output is **NOT** meant to be displayed visually.
  #[must_use]
  pub const fn into_str(self) -> String {
    // SAFETY: see definition of CuredString
    unsafe { transmute(self) }
  }

  /// Checks if this [`CuredString`] ***similarly*** starts with another string.
  ///
  /// This comparison is *case-insensitive*.
  #[must_use]
  #[inline(always)]
  pub fn starts_with(&self, other: &str) -> bool {
    self.len() >= other.len() && similar::is_str(self, other)
  }

  /// Checks if this [`CuredString`] ***similarly*** ends with another string.
  ///
  /// This comparison is *case-insensitive*.
  #[must_use]
  #[inline(always)]
  pub fn ends_with(&self, other: &str) -> bool {
    self.len() >= other.len() && similar::is_iter(self.chars().rev(), other.chars().rev())
  }

  /// Checks if this [`CuredString`] ***similarly*** contains another string.
  ///
  /// This comparison is *case-insensitive*.
  #[must_use]
  pub fn contains(&self, other: &str) -> bool {
    if other.len() > self.len() {
      return false;
    }

    let other_chars: Vec<_> = other.chars().collect();
    let mut other_index = 0usize;

    for self_char in self.chars() {
      if similar::is(self_char as _, other_chars[other_index]) {
        other_index += 1;

        if other_index == other_chars.len() {
          return true;
        }
      } else {
        other_index = 0;
      }
    }

    false
  }
}

impl From<CuredString> for String {
  #[inline(always)]
  fn from(val: CuredString) -> Self {
    val.into_str()
  }
}

impl AsRef<str> for CuredString {
  #[inline(always)]
  fn as_ref(&self) -> &str {
    &self.0
  }
}

/// Checks if this [`CuredString`] is ***similar*** into another string.
///
/// This comparison is *case-insensitive*.
impl<S> PartialEq<S> for CuredString
where
  S: AsRef<str> + ?Sized,
{
  #[must_use]
  #[inline(always)]
  fn eq(&self, other: &S) -> bool {
    let other = other.as_ref();

    self.len() == other.len() && similar::is_str(self, other)
  }
}

impl Debug for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&self.0, f)
  }
}

impl Display for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&self.0, f)
  }
}

impl Deref for CuredString {
  type Target = String;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for CuredString {
  #[inline(always)]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self)
  }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[allow(clippy::redundant_closure)]
impl<'de> Deserialize<'de> for CuredString {
  #[inline(always)]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Deserialize::deserialize(deserializer)
      .and_then(|s: &str| crate::cure(s).map_err(de::Error::custom))
  }
}
