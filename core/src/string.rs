use crate::{cure, cure_char, similar, translation::Translation};
use core::{
  cmp::PartialEq,
  fmt::{self, Debug, Display, Formatter, Write},
  mem::transmute,
  num::NonZeroU32,
  ops::{Add, AddAssign, Deref},
  str::{self, FromStr},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io;

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

/// A helper implementation for [curing][cure] and [appending][Add] a [`char`] into a [`CuredString`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = decancer::cure(text);
///
/// for character in text.chars() {
///   cured = cured + character;
/// }
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl Add<char> for CuredString {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: char) -> Self::Output {
    self.add(cure_char(rhs))
  }
}

/// A helper implementation for [curing][cure] and [appending][Add] a [`NonZeroU32`] into a [`CuredString`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use core::num::NonZeroU32;
///
/// let mut cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// // SAFETY: this literal is obviously NOT zero.
/// let other = unsafe { NonZeroU32::new_unchecked(0xD800) };
/// cured = cured + other;
///
/// assert_eq!(cured, "very funny text");
/// ```
impl Add<NonZeroU32> for CuredString {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: NonZeroU32) -> Self::Output {
    self.add(cure_char(rhs))
  }
}

/// A helper implementation for [curing][cure] and [appending][Add] a [`&str`][prim@str] into a [`CuredString`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = decancer::cure(text);
/// cured = cured + text;
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl Add<&str> for CuredString {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: &str) -> Self::Output {
    Self(self.0.add(cure(rhs).as_str()))
  }
}

/// A helper implementation for [appending][Add] a [`Translation`] into a [`CuredString`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = decancer::cure(text);
///
/// for cured_char in text.chars().map(decancer::cure_char) {
///   cured = cured + cured_char;
/// }
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl Add<Translation> for CuredString {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: Translation) -> Self::Output {
    Self(self.0.add(rhs))
  }
}

/// A helper implementation for [curing][cure] and [appending][Add] a [`u32`] into a [`CuredString`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let mut cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
/// cured = cured + 0xD800u32;
///
/// assert_eq!(cured, "very funny text");
/// ```
impl Add<u32> for CuredString {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: u32) -> Self::Output {
    self.add(cure_char(rhs))
  }
}

/// A helper implementation for [curing][cure] and [appending][AddAssign] a [`char`] into a [`CuredString`] in-place.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = decancer::cure(text);
///
/// for character in text.chars() {
///   cured += character;
/// }
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl AddAssign<char> for CuredString {
  #[inline(always)]
  fn add_assign(&mut self, rhs: char) {
    self.add_assign(cure_char(rhs))
  }
}

/// A helper implementation for [curing][cure] and [appending][AddAssign] a [`NonZeroU32`] into a [`CuredString`] in-place.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use core::num::NonZeroU32;
///
/// let mut cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// // SAFETY: this literal is obviously NOT zero.
/// let other = unsafe { NonZeroU32::new_unchecked(0xD800) };
/// cured += other;
///
/// assert_eq!(cured, "very funny text");
/// ```
impl AddAssign<NonZeroU32> for CuredString {
  #[inline(always)]
  fn add_assign(&mut self, rhs: NonZeroU32) {
    self.add_assign(cure_char(rhs))
  }
}

/// A helper implementation for [curing][cure] and [appending][AddAssign] a [`&str`][prim@str] into a [`CuredString`] in-place.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = decancer::cure(text);
///
/// cured += text;
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl AddAssign<&str> for CuredString {
  #[inline(always)]
  fn add_assign(&mut self, rhs: &str) {
    self.0.add_assign(cure(rhs).as_str())
  }
}

/// A helper implementation for [appending][AddAssign] a [`Translation`] into a [`CuredString`] in-place.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = decancer::cure(text);
///
/// for cured_char in text.chars().map(decancer::cure_char) {
///   cured += cured_char;
/// }
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl AddAssign<Translation> for CuredString {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Translation) {
    self.0.add_assign(rhs)
  }
}

/// A helper implementation for [curing][cure] and [appending][AddAssign] a [`u32`] into a [`CuredString`] in-place.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use core::num::NonZeroU32;
///
/// let mut cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
/// cured += 0xD800;
///
/// assert_eq!(cured, "very funny text");
/// ```
impl AddAssign<u32> for CuredString {
  #[inline(always)]
  fn add_assign(&mut self, rhs: u32) {
    self.add_assign(cure_char(rhs))
  }
}

/// [Extends][Extend] a [`String`] with an [iterator][Iterator] that yields [`CuredString`]s.
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
    I: IntoIterator<Item = CuredString>,
  {
    self.extend(iter.into_iter().map(|s| s.into_str()))
  }
}

/// [Extends][Extend] a [`CuredString`] with an [iterator][Iterator] that yields [`char`]s.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = decancer::cure(text);
/// cured.extend(text.chars());
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl Extend<char> for CuredString {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = char>,
  {
    self.extend(iter.into_iter().map(cure_char))
  }
}

/// [Extends][Extend] a [`CuredString`] with an [iterator][Iterator] that yields [`NonZeroU32`]s.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use core::num::NonZeroU32;
///
/// let mut cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// // SAFETY: these numbers are obviously above zero.
/// cured.extend([
///   0x76, 0xFF25, 0x24E1, 0x1D502, 0x20,
///   0x1D53D, 0x1D54C, 0x147, 0x2115, 0xFF59,
///   0x20, 0x163, 0x4E47, 0x1D54F, 0x1D4E3
/// ].map(|c| unsafe { NonZeroU32::new_unchecked(c) }));
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl Extend<NonZeroU32> for CuredString {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = NonZeroU32>,
  {
    self.extend(iter.into_iter().map(cure_char))
  }
}

/// [Extends][Extend] a [`CuredString`] with an [iterator][Iterator] that yields [`String`]s.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let mut cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
/// cured.extend([
///   String::from("vＥⓡ𝔂 "),
///   String::from("𝔽𝕌Ňℕｙ "),
///   String::from("ţ乇𝕏𝓣")
/// ]);
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl Extend<String> for CuredString {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = String>,
  {
    self.extend(iter.into_iter().map(|s| cure(&s)))
  }
}

/// [Extends][Extend] a [`CuredString`] with an [iterator][Iterator] that yields [`&str`][prim@str]s.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let mut cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
/// cured.extend(["vＥⓡ𝔂 ", "𝔽𝕌Ňℕｙ ", "ţ乇𝕏𝓣"]);
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl<'a> Extend<&'a str> for CuredString {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = &'a str>,
  {
    self.extend(iter.into_iter().map(cure))
  }
}

/// [Extends][Extend] a [`CuredString`] with an [iterator][Iterator] that yields another [`CuredString`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let mut text = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
/// text.extend([
///   decancer::cure("vＥⓡ𝔂 "),
///   decancer::cure("𝔽𝕌Ňℕｙ "),
///   decancer::cure("ţ乇𝕏𝓣")
/// ]);
///
/// assert_eq!(text, "very funny textvery funny text");
/// ```
impl Extend<CuredString> for CuredString {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = CuredString>,
  {
    self.0.extend(iter)
  }
}

/// [Extends][Extend] a [`CuredString`] with an [iterator][Iterator] that yields [`Translation`]s.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let mut text = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
/// text.extend([decancer::cure_char('Ｅ'), decancer::cure_char('Ｅ')]);
///
/// assert_eq!(text, "very funny textee");
/// ```
impl Extend<Translation> for CuredString {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = Translation>,
  {
    self.0.extend(iter)
  }
}

/// [Extends][Extend] a [`CuredString`] with an [iterator][Iterator] that yields [`u32`]s.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let mut cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
/// cured.extend([
///   0x76, 0xFF25, 0x24E1, 0x1D502, 0x20,
///   0x1D53D, 0x1D54C, 0x147, 0x2115, 0xFF59,
///   0x20, 0x163, 0x4E47, 0x1D54F, 0x1D4E3
/// ]);
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl Extend<u32> for CuredString {
  #[inline(always)]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = u32>,
  {
    self.extend(iter.into_iter().map(cure_char))
  }
}

/// [Cures][cure] a string.
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
/// // cured here is a decancer::CuredString struct wrapping over the cured string
/// // for comparison purposes, it's more recommended to use the methods provided by the decancer::CuredString struct.
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.contains("funny"));
/// assert!(cured.ends_with("text"));
///
/// // retrieve the String inside and consume the struct.
/// let _output_str = cured.into_str();
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

/// A helper implementation for curing and joining several characters into one [`CuredString`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use decancer::CuredString;
///
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let cured: CuredString = text.chars().collect();
///
/// // cured here is a CuredString struct wrapping over the cured string
/// // for comparison purposes, it's more recommended to use the methods provided by the CuredString struct.
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.contains("funny"));
/// assert!(cured.ends_with("text"));
///
/// // retrieve the String inside and consume the struct.
/// let _output_str = cured.into_str();
/// ```
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

/// A helper implementation for [joining][Iterator::collect] several [`Translation`]s into one [`CuredString`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use decancer::CuredString;
///
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let cured: CuredString = text.chars().map(decancer::cure_char).collect();
///
/// // cured here is a CuredString struct wrapping over the cured string
/// // for comparison purposes, it's more recommended to use the methods provided by the CuredString struct.
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.contains("funny"));
/// assert!(cured.ends_with("text"));
///
/// // retrieve the String inside and consume the struct.
/// let _output_str = cured.into_str();
/// ```
impl FromIterator<Translation> for CuredString {
  #[inline(always)]
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = Translation>,
  {
    Self(iter.into_iter().collect())
  }
}

/// [Cures][cure] a string.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use decancer::CuredString;
///
/// // SAFETY: the function will never return an Err.
/// let cured: CuredString = unsafe { "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣".parse().unwrap_unchecked() };
///
/// // cured here is a decancer::CuredString struct wrapping over the cured string
/// // for comparison purposes, it's more recommended to use the methods provided by the decancer::CuredString struct.
/// assert_eq!(cured, "very funny text");
/// assert!(cured.starts_with("very"));
/// assert!(cured.contains("funny"));
/// assert!(cured.ends_with("text"));
///
/// // retrieve the String inside and consume the struct.
/// let _output_str = cured.into_str();
/// ```
impl FromStr for CuredString {
  type Err = ();

  #[inline(always)]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(cure(s))
  }
}

/// Coerces this [`CuredString`] into a [`&str`][prim@str].
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
  #[inline(always)]
  fn eq(&self, other: &S) -> bool {
    let other = other.as_ref();

    self.len() == other.len() && similar::is_str(self, other)
  }
}

/// [Formats][Debug] this `CuredString`. Behaves exactly just like formatting your typical `String`.
impl Debug for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&self.0, f)
  }
}

/// [Formats][Display] this `CuredString`. Behaves exactly just like formatting your typical `String`.
impl Display for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&self.0, f)
  }
}

/// A helper implementation for implicitly [inheriting][Deref] [`String`] and subsequently [`&str`][prim@str]'s methods.
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

/// A helper implementation for [curing][cure] and [appending][Write] either a [`char`] or a [`&str`][prim@str] into a [`CuredString`].
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use core::fmt::Write;
///
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = decancer::cure(text);
///
/// // SAFETY: the implementation of these functions will never return an Err.
/// unsafe {
///   cured.write_str(text).unwrap_unchecked();
///
///   for character in text.chars() {
///     cured.write_char(character).unwrap_unchecked();
///   }
/// }
///
/// assert_eq!(cured, "very funny textvery funny textvery funny text");
/// ```
impl Write for CuredString {
  #[inline(always)]
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.add_assign(s);

    Ok(())
  }

  #[inline(always)]
  fn write_char(&mut self, c: char) -> fmt::Result {
    self.add_assign(c);

    Ok(())
  }
}

/// A helper implementation for [curing][cure] and [appending][io::Write] a [UTF-8](https://en.wikipedia.org/wiki/UTF-8) [slice] into a [`CuredString`].
///
/// # Errors
///
/// [Errors][io::Error] if the [slice] contains [invalid][io::ErrorKind::InvalidData] [UTF-8](https://en.wikipedia.org/wiki/UTF-8) bytes. (See [`Utf8Error`][str::Utf8Error])
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use std::io::Write;
///
/// let text = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// let mut cured = decancer::cure(text);
///
/// write!(cured, "{text}").unwrap();
///
/// assert_eq!(cured, "very funny textvery funny text");
/// ```
impl io::Write for CuredString {
  #[inline(always)]
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    match str::from_utf8(buf) {
      Ok(s) => {
        self.0.add_assign(cure(s).as_str());

        Ok(buf.len())
      }
      Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
  }

  #[inline(always)]
  fn flush(&mut self) -> io::Result<()> {
    Ok(())
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
/// // decancered.cured_string here is a CuredString struct wrapping over the cured string
/// // for comparison purposes, it's more recommended to use the methods provided by the CuredString struct.
/// assert_eq!(decancered.cured_string, "very funny text");
/// assert!(decancered.cured_string.starts_with("very"));
/// assert!(decancered.cured_string.contains("funny"));
/// assert!(decancered.cured_string.ends_with("text"));
///
/// // retrieve the String inside and consume the struct.
/// let _output_str = decancered.cured_string.into_str();
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
