use crate::{cure, similar};
use core::{cmp::PartialEq, fmt, mem::transmute, ops::Deref, str::FromStr};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A small wrapper around the [`String`] datatype for comparison purposes.
///
/// This is used because imperfections from translations can happen, thus this is used to provide comparison functions that are not as strict and can detect similar-looking characters (e.g: `i` and `l`)
#[derive(Clone, Eq)]
#[must_use]
pub struct CuredString(pub(crate) String);

impl CuredString {
  /// Coerces this [`CuredString`] to a [`String`].
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
  pub fn starts_with<S>(&self, other: &S) -> bool
  where
    S: AsRef<str> + ?Sized,
  {
    let o = other.as_ref();

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
  pub fn ends_with<S>(&self, other: &S) -> bool
  where
    S: AsRef<str> + ?Sized,
  {
    let o = other.as_ref();

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
  pub fn contains<S>(&self, other: &S) -> bool
  where
    S: AsRef<str> + ?Sized,
  {
    let o = other.as_ref();

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

/// Extends a [`String`] with an iterator that yields [`CuredString`]s.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let mut text = String::new();
/// text.extend([
///   decancer::cure("vＥⓡ𝔂 "),
///   decancer::cure("𝔽𝕌Ňℕｙ "),
///   decancer::cure("ţ乇𝕏𝓣")
/// ]);
///
/// assert_eq!(text, "very funny text");
/// ```
impl Extend<CuredString> for String {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = CuredString>
  {
    self.extend(iter.into_iter().map(|s| s.into_str()))
  }
}

/// Cures a string. Alias for [`cure`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use decancer::CuredString;
///
/// let cured = CuredString::from("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.ends_with("text"));
/// assert!(cured.contains("funny"));
/// ```
impl<S> From<&S> for CuredString
where
  S: AsRef<str> + ?Sized,
{
  #[inline(always)]
  fn from(s: &S) -> Self {
    cure(s)
  }
}

/// Coerces a [`CuredString`] into a [`String`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured: String = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣").into();
///
/// assert_eq!(cured, "very funny text");
/// ```
impl From<CuredString> for String {
  #[inline(always)]
  fn from(val: CuredString) -> Self {
    val.into_str()
  }
}

/// Alias for [`cure`]. This never errors.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use decancer::CuredString;
///
/// // the function never returns an Err, so we can safely unwrap_unchecked this
/// let cured: CuredString = unsafe { "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣".parse().unwrap_unchecked() };
///
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.ends_with("text"));
/// assert!(cured.contains("funny"));
/// ```
impl FromStr for CuredString {
  type Err = ();

  #[inline(always)]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(cure(s))
  }
}

/// Coerces this [`CuredString`] to a [`&str`][str].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// assert_eq!(cured.as_ref(), "very funny text");
/// ```
impl AsRef<str> for CuredString {
  #[inline(always)]
  fn as_ref(&self) -> &str {
    &self.0
  }
}

/// Checks if this [`CuredString`] is ***similar*** to another string.
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
  #[inline(always)]
  fn eq(&self, other: &S) -> bool {
    let other = other.as_ref();

    self.len() == other.len() && similar::is_str(self, other)
  }
}

/// Formats this `CuredString`. Behaves exactly just like formatting your typical `String`.
impl fmt::Debug for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.0, f)
  }
}

/// Formats this `CuredString`. Behaves exactly just like formatting your typical `String`.
impl fmt::Display for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.0, f)
  }
}

/// A helper implementation for implicitly inheriting [`String`] and subsequently [`&str`][str]'s methods.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// // cured.len() here is str's method!
/// assert_eq!(cured.len(), 15);
/// ```
impl Deref for CuredString {
  type Target = String;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

/// [Serializes][Serialize] this [`CuredString`] into a [`string`][Serializer::serialize_str].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use decancer::CuredString;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Decancered {
///   cured_string: CuredString,
/// }
///
/// let decancered = Decancered {
///   cured_string: decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"),
/// };
///
/// assert_eq!(serde_json::to_string(&decancered).unwrap(), r#"{"cured_string":"very funny text"}"#);
/// ```
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

/// [Deserializes][Deserialize] and [cures][cure] a [`string`][Deserializer::deserialize_str].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use decancer::CuredString;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Decancered {
///   cured_string: CuredString,
/// }
///
/// let json = r#"{"cured_string": "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"}"#;
/// let decancered: Decancered = serde_json::from_str(json).unwrap();
///
/// assert_eq!(decancered.cured_string, "very funny text");
/// ```
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
