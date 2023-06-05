use crate::{cure_char, similar, translation::Translation};
use core::{
  cmp::PartialEq,
  fmt::{self, Debug, Display, Formatter},
  mem::transmute,
  ops::Deref,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A small wrapper around the [`String`] datatype for comparison purposes.
///
/// This is used because imperfections from translations can happen, thus this is used to provide comparison functions that are not as strict and can detect similar-looking characters (e.g: `i` and `l`)
#[must_use]
#[derive(Clone, Eq)]
pub struct CuredString(pub(crate) String);

impl CuredString {
  /// Coerces this [`CuredString`] into a [`String`].
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣").into_str();
  ///
  /// assert_eq!(cured, "very funny text");
  /// ```
  #[must_use]
  pub const fn into_str(self) -> String {
    unsafe { transmute(self) }
  }

  /// Checks if this [`CuredString`] ***similarly*** starts with another string.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  ///
  /// assert!(cured.starts_with("very"));
  /// ```
  ///
  /// And since it checks if the strings are similar, please note that this is valid too:
  ///
  /// ```rust
  /// // assume this has no effect
  /// let cured = decancer::cure("vwv (vnt 1l1");
  ///
  /// // it assumes that v is similar to u as well
  /// assert!(cured.starts_with("uwu"));
  /// ```
  #[must_use]
  #[inline(always)]
  pub fn starts_with(&self, o: &str) -> bool {
    self.len() >= o.len() && similar::is_str(self, o)
  }

  /// Checks if this [`CuredString`] ***similarly*** ends with another string.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  ///
  /// assert!(cured.ends_with("text"));
  /// ```
  ///
  /// And since it checks if the strings are similar, please note that this is valid too:
  ///
  /// ```rust
  /// // assume this has no effect
  /// let cured = decancer::cure("vwv (vnt 1l1");
  ///
  /// // it assumes that 1 is similar to l and i as well
  /// assert!(cured.ends_with("lil"));
  /// ```
  #[must_use]
  #[inline(always)]
  pub fn ends_with(&self, o: &str) -> bool {
    self.len() >= o.len() && similar::is_iter(self.chars().rev(), o.chars().rev())
  }

  /// Checks if this [`CuredString`] ***similarly*** contains another string.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  ///
  /// assert!(cured.contains("funny"));
  /// ```
  ///
  /// And since it checks if the strings are similar, please note that this is valid too;
  ///
  /// ```rust
  /// // assume this has no effect
  /// let cured = decancer::cure("vwv cvnt 1l1");
  ///
  /// // it assumes that v is similar to u
  /// assert!(cured.contains("cunt"));
  /// ```
  #[must_use]
  pub fn contains(&self, o: &str) -> bool {
    if o.len() > self.len() {
      return false;
    }

    let other_chars: Vec<_> = o.chars().collect();
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

impl Extend<CuredString> for String {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = CuredString>,
  {
    self.extend(iter.into_iter().map(|s| s.into_str()))
  }
}

impl From<CuredString> for String {
  #[inline(always)]
  fn from(val: CuredString) -> Self {
    val.into_str()
  }
}

impl<C> FromIterator<C> for CuredString
where
  C: Into<u32>,
{
  #[inline(always)]
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = C>,
  {
    iter.into_iter().map(cure_char).collect()
  }
}

impl FromIterator<Translation> for CuredString {
  #[inline(always)]
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = Translation>,
  {
    Self(iter.into_iter().collect())
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
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// assert_eq!(cured, "very funny text");
/// ```
///
/// And since it checks if the strings are similar, please note that this is valid too:
///
/// ```rust
/// // assume this has no effect
/// let cured = decancer::cure("vwv cvnt 1l1");
///
/// // it assumes that v is similar to u
/// assert_eq!(cured, "uwu cunt lil");
/// ```
impl<S> PartialEq<S> for CuredString
where
  S: AsRef<str> + ?Sized,
{
  #[must_use]
  #[inline(always)]
  fn eq(&self, o: &S) -> bool {
    let o = o.as_ref();

    self.len() == o.len() && similar::is_str(self, o)
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
    Deserialize::deserialize(deserializer).map(|s: &str| cure(s))
  }
}
