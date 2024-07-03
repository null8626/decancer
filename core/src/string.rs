use crate::{
  util::{merge_ranges, unwrap_or_ret},
  Matcher,
};
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
  fmt::{self, Debug, Display, Formatter},
  ops::{Deref, Range},
};

/// A small wrapper around the [`String`] data type for comparison purposes.
///
/// This is used because imperfections from translations can happen, thus this is used to provide comparison functions that are not as strict and can detect similar-looking characters (e.g: `i` and `l`)
#[derive(Clone, Eq, Hash)]
pub struct CuredString(pub(crate) String);

impl CuredString {
  /// Iterates throughout this string and yields every similar-looking match.
  ///
  /// If you plan on using this method with an array of strings, use [`find_multiple`][CuredString::find_multiple].
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

  /// Iterates throughout this string and returns a [`Vec`] of every similar-looking match. Unlike [`find`][CuredString::find], this method also takes note of overlapping matches and merges them together.
  ///
  /// This comparison is case-insensitive.
  ///
  /// ```rust
  /// let cured = decancer::cure!("ｈꡩ𝔏┕⊕𝚑ᅠΎ⫕ᣲ𑀜").unwrap();
  /// let matches = cured.find_multiple(["hello", "oh yeah"]);
  ///
  /// assert_eq!(matches, [0..11]);
  /// ```
  ///
  /// Usage with the [`censor`](https://docs.rs/censor) crate:
  ///
  /// ```rust
  /// let censor = censor::Standard + censor::Sex;
  ///
  /// let cured = decancer::cure!("𝑺ꡘ꡶イ↥⢗ㄒ❘⋶ᔚ").unwrap();
  /// let matches = cured.find_multiple(censor.set());
  ///
  /// assert_eq!(matches, [0..10]);  
  /// ```
  pub fn find_multiple<S, O>(&self, other: O) -> Vec<Range<usize>>
  where
    S: AsRef<str>,
    O: IntoIterator<Item = S>,
  {
    let other = other.into_iter();
    let mut ranges = Vec::with_capacity(other.size_hint().0);

    for o in other {
      ranges.extend(self.find(o.as_ref()));
    }

    merge_ranges(&mut ranges);
    ranges
  }

  fn censor_inner<I>(&mut self, original: &str, matches: I, with: char)
  where
    I: IntoIterator<Item = Range<usize>>,
  {
    let mut with_str = String::new();
    let mut char_diff = 0isize;

    for mat in matches {
      let chars = original[mat.clone()].chars().count();
      let cap = chars * with.len_utf8();

      with_str.reserve_exact(cap);

      for _ in with_str.len() / with.len_utf8()..chars {
        with_str.push(with);
      }

      self.0.replace_range(
        (mat.start as isize + char_diff) as usize..(mat.end as isize + char_diff) as _,
        &with_str,
      );

      char_diff += cap as isize - mat.len() as isize;
    }
  }

  /// Censors every match of a string with a repetition of a character in-place.
  ///
  /// If you plan on using this method with an array of strings, use [`censor_multiple`][CuredString::censor_multiple].
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
    let original = self.clone();

    self.censor_inner(&original, original.find(other), with);
  }

  /// Censors every matches from an array of strings with a repetition of a character in-place.
  ///
  /// This comparison is case-insensitive.
  ///
  /// ```rust
  /// let mut cured = decancer::cure!("ꀡৎレレ⌽ⴙᅠ𝓎ȩ㆟ҥ").unwrap();
  /// cured.censor_multiple(["hello", "oh yeah"], '*');
  ///
  /// assert_eq!(cured, "***********");
  /// ```
  ///
  /// Usage with the [`censor`](https://docs.rs/censor) crate:
  ///
  /// ```rust
  /// let censor = censor::Standard + censor::Sex;
  ///
  /// let mut cured = decancer::cure!("𝑺ꡘ꡶イ↥⢗ㄒ❘⋶ᔚ").unwrap();
  /// cured.censor_multiple(censor.set(), '*');
  ///
  /// assert_eq!(cured, "**********");
  /// ```
  pub fn censor_multiple<S, O>(&mut self, other: O, with: char)
  where
    S: AsRef<str>,
    O: IntoIterator<Item = S>,
  {
    let original = self.clone();

    self.censor_inner(&original, original.find_multiple(other), with);
  }

  fn replace_inner<I>(&mut self, matches: I, with: &str)
  where
    I: IntoIterator<Item = Range<usize>>,
  {
    let mut char_diff = 0isize;

    for mat in matches {
      self.0.replace_range(
        (mat.start as isize + char_diff) as usize..(mat.end as isize + char_diff) as _,
        with,
      );

      char_diff += with.len() as isize - mat.len() as isize;
    }
  }

  /// Replaces every match of a string with another string in-place.
  ///
  /// If you plan on using this method with an array of strings, use [`replace_multiple`][CuredString::replace_multiple].
  ///
  /// This comparison is case-insensitive.
  ///
  /// ```rust
  /// let mut cured = decancer::cure!("wow hello wow heellllo!").unwrap();
  /// cured.replace("hello", "world");
  ///
  /// assert_eq!(cured, "wow world wow world!");
  /// ```
  #[inline(always)]
  pub fn replace(&mut self, other: &str, with: &str) {
    self.replace_inner(self.clone().find(other), with);
  }

  /// Replaces every matches from an array of strings with another string in-place.
  ///
  /// This comparison is case-insensitive.
  ///
  /// ```rust
  /// let mut cured = decancer::cure!("ꀡৎレレ⌽ⴙᅠ𝓎ȩ㆟ҥ").unwrap();
  /// cured.replace_multiple(["hello", "oh yeah"], "world");
  ///
  /// assert_eq!(cured, "world");
  /// ```
  ///
  /// Usage with the [`censor`](https://docs.rs/censor) crate:
  ///
  /// ```rust
  /// let censor = censor::Standard + censor::Sex;
  ///
  /// let mut cured = decancer::cure!("𝑺ꡘ꡶イ↥⢗ㄒ❘⋶ᔚ").unwrap();
  /// cured.replace_multiple(censor.set(), "no :)");
  ///
  /// assert_eq!(cured, "no :)");
  /// ```
  #[inline(always)]
  pub fn replace_multiple<S, O>(&mut self, other: O, with: &str)
  where
    S: AsRef<str>,
    O: IntoIterator<Item = S>,
  {
    self.replace_inner(self.clone().find_multiple(other), with);
  }

  /// Checks if this cured string similarly starts with another string.
  ///
  /// This comparison is case-insensitive.
  pub fn starts_with(&self, other: &str) -> bool {
    let mut iter = self.find(other);
    let mat = unwrap_or_ret!(iter.next(), false);

    mat.start == 0
  }

  /// Checks if this cured string similarly ends with another string.
  ///
  /// This comparison is case-insensitive.
  pub fn ends_with(&self, other: &str) -> bool {
    let last = unwrap_or_ret!(self.find(other).last(), false);

    last.end == self.len()
  }

  /// Checks if this cured string similarly contains another string.
  ///
  /// This comparison is case-insensitive.
  pub fn contains(&self, other: &str) -> bool {
    let mut iter = self.find(other);

    iter.next().is_some()
  }
}

/// Coerces this cured string to a [`String`].
///
/// **NOTE:** It's highly **NOT** recommended to use Rust's comparison methods after calling this, and since the string output is laid out in memory the same way as it were to be displayed graphically, displaying it **may not display correctly** since some right-to-left characters are reversed.  
impl From<CuredString> for String {
  #[inline(always)]
  fn from(val: CuredString) -> Self {
    val.0
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
