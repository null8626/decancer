use crate::similar;
use core::{cmp::PartialEq, fmt, mem::transmute, ops::Deref};

/// A small wrapper around the [`String`] datatype for comparison purposes.
///
/// This is used because imperfections from translations can happen, thus this is used to provide comparison functions that are not as strict and can detect similar-looking characters (e.g: `i` and `l`)
#[derive(Clone)]
pub struct CuredString(pub(crate) String);

impl CuredString {
  /// Coerces this data to a [`String`].
  /// [`transmuting`][std::mem::transmute] works too.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  /// assert_eq!(cured.into_str(), String::from("very funny text"));
  /// ```
  #[must_use]
  pub const fn into_str(self) -> String {
    unsafe { transmute(self) }
  }

  /// Checks if this string ***similarly*** starts with another string.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  /// assert!(cured.starts_with("very"));
  /// ```
  ///
  /// And since it checks if the strings are similar, please note that this is valid too.
  ///
  /// ```rust
  /// let cured = decancer::cure("vwv (vnt 111"); // assume this has no effect
  /// assert!(cured.starts_with("uwu")); // it assumes that v is similar to u as well
  /// ```
  #[must_use]
  pub fn starts_with<S>(&self, other: &S) -> bool
  where
    S: AsRef<str> + ?Sized,
  {
    let o = other.as_ref();

    if o.len() > self.len() {
      return false;
    }

    let mut other_iter = o.chars();

    for self_char in self.chars() {
      match other_iter.next() {
        Some(other_char) => {
          if !similar::is(self_char as _, other_char as _) {
            return false;
          }
        }

        None => return true,
      };
    }

    false
  }

  /// Checks if this string ***similarly*** ends with another string.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  /// assert!(cured.ends_with("text"));
  /// ```
  ///
  /// And since it checks if the strings are similar, please note that this is valid too.
  ///
  /// ```rust
  /// let cured = decancer::cure("vwv (vnt 111"); // assume this has no effect
  /// assert!(cured.ends_with("lil")); // it assumes that 1 is similar to l and i as well
  /// ```
  #[must_use]
  pub fn ends_with<S>(&self, other: &S) -> bool
  where
    S: AsRef<str> + ?Sized,
  {
    let o = other.as_ref();

    if o.len() > self.len() {
      return false;
    }

    let mut other_iter = o.chars().rev();

    for self_char in self.chars().rev() {
      match other_iter.next() {
        Some(other_char) => {
          if !similar::is(self_char as _, other_char as _) {
            return false;
          }
        }

        None => return true,
      };
    }

    false
  }

  /// Checks if this string ***similarly*** contains another string.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  /// assert!(cured.contains("funny"));
  /// ```
  ///
  /// And since it checks if the strings are similar, please note that this is valid too.
  ///
  /// ```rust
  /// let cured = decancer::cure("vwv (vnt 111"); // assume this has no effect
  /// assert!(cured.contains("cunt")); // it assumes that ( is similar to c and v is similar to u as well
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

    let other_chars = o.chars().collect::<Vec<_>>();
    let mut other_index = 0usize;

    for self_char in self.chars() {
      if similar::is(other_chars[other_index] as _, self_char as _) {
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

/// Checks if this string is ***similar*** to another string.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
/// assert!(cured == "very funny text");
/// ```
///
/// And since it checks if the strings are similar, please note that this is valid too.
///
/// ```rust
/// let cured = decancer::cure("vwv (vnt 111"); // assume this has no effect
/// assert!(cured == "uwu cunt lil");
/// ```
impl<S> PartialEq<S> for CuredString
where
  S: AsRef<str> + ?Sized,
{
  fn eq(&self, other: &S) -> bool {
    let o = other.as_ref();

    if self.len() != o.len() {
      false
    } else {
      let mut other_iter = o.chars();

      for self_char in self.chars() {
        match other_iter.next() {
          Some(other_char) => {
            if !similar::is(self_char as _, other_char as _) {
              return false;
            }
          }

          None => return false,
        };
      }

      true
    }
  }
}

impl fmt::Debug for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.0, f)
  }
}

impl fmt::Display for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.0, f)
  }
}

impl Deref for CuredString {
  type Target = String;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
