use crate::{util::unwrap_or_ret, Matcher};
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
  cmp::PartialEq,
  fmt::{self, Debug, Display, Formatter},
  mem::transmute,
  ops::Deref,
};

/// A small wrapper around the [`String`] data type for comparison purposes.
///
/// This is used because imperfections from translations can happen, thus this is used to provide comparison functions that are not as strict and can detect similar-looking characters (e.g: `i` and `l`)
#[must_use]
#[derive(Clone, Eq)]
pub struct CuredString(pub(crate) String);

impl CuredString {
  /// Coerces this cured string into a [`String`].
  ///
  /// **NOTE:** It's highly **NOT** recommended to use Rust's comparison methods after calling this, and since the string output is laid out in memory the same way as it were to be displayed graphically, displaying it **may not display correctly** since some right-to-left characters are reversed.
  #[must_use]
  pub const fn into_str(self) -> String {
    // SAFETY: see definition of CuredString
    unsafe { transmute(self) }
  }

  /// Iterates throughout this string and yields every similar-looking match.
  ///
  /// This comparison is case-insensitive.
  ///
  /// ```rust
  /// let cured = decancer::cure!("wow hello wow heellllo!").unwrap();
  /// let mut matcher = cured.find("hello");
  ///
  /// assert_eq!(matcher.next(), Some(4..9));
  /// assert_eq!(matcher.next(), Some(14..22));
  /// assert_eq!(matcher.next(), None);
  /// ```
  #[inline(always)]
  pub fn find<'a, 'b>(&'a self, other: &'b str) -> Matcher<'a, 'b> {
    Matcher::new(self, other)
  }

  /// Censors every match of a string with a repetition of a character in-place.
  ///
  /// This comparison is case-insensitive.
  ///
  /// ```rust
  /// let mut cured = decancer::cure!("wow heellllo wow hello wow!").unwrap();
  /// cured.censor("hello", '*');
  ///
  /// assert_eq!(cured, "wow ******** wow ***** wow!");
  /// ```
  pub fn censor(&mut self, other: &str, with: char) {
    let original = self.0.clone();
    let mut char_diff = 0isize;

    for mat in Matcher::new(&original, other) {
      // SAFETY: mat is always within the bounds of self
      let chars = unsafe { original.get_unchecked(mat.clone()) }
        .chars()
        .count();
      let mut with_str = String::with_capacity(chars);

      for _ in 0..chars {
        with_str.push(with);
      }

      self.0.replace_range(
        (mat.start as isize + char_diff) as usize..(mat.end as isize + char_diff) as _,
        &with_str,
      );

      char_diff += (with.len_utf8() * chars) as isize - mat.len() as isize;
    }
  }

  /// Replaces every match of a string with another string in-place.
  ///
  /// This comparison is case-insensitive.
  ///
  /// ```rust
  /// let mut cured = decancer::cure!("wow hello wow heellllo!").unwrap();
  /// cured.replace("hello", "world");
  ///
  /// assert_eq!(cured, "wow world wow world!");
  /// ```
  pub fn replace(&mut self, other: &str, with: &str) {
    let original = self.0.clone();
    let mut char_diff = 0isize;

    for mat in Matcher::new(&original, other) {
      self.0.replace_range(
        (mat.start as isize + char_diff) as usize..(mat.end as isize + char_diff) as _,
        with,
      );

      char_diff += with.len() as isize - mat.len() as isize;
    }
  }

  /// Checks if this cured string similarly starts with another string.
  ///
  /// This comparison is case-insensitive.
  #[must_use]
  pub fn starts_with(&self, other: &str) -> bool {
    let mut iter = self.find(other);
    let mat = unwrap_or_ret!(iter.next(), false);

    mat.start == 0
  }

  /// Checks if this cured string similarly ends with another string.
  ///
  /// This comparison is case-insensitive.
  #[must_use]
  pub fn ends_with(&self, other: &str) -> bool {
    let last = unwrap_or_ret!(self.find(other).last(), false);

    last.end == self.len()
  }

  /// Checks if this cured string similarly contains another string.
  ///
  /// This comparison is case-insensitive.
  #[must_use]
  pub fn contains(&self, other: &str) -> bool {
    let mut iter = self.find(other);

    iter.next().is_some()
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

/// Checks if this cured string is similar with another string.
///
/// This comparison is case-insensitive.
impl<S> PartialEq<S> for CuredString
where
  S: AsRef<str> + ?Sized,
{
  #[must_use]
  #[inline(always)]
  fn eq(&self, other: &S) -> bool {
    Matcher::is_equal(self, other.as_ref())
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
      .and_then(|s: &str| crate::cure!(s).map_err(de::Error::custom))
  }
}
