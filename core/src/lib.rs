mod confusables;
mod encoding;

use confusables::*;
use encoding::*;

/// A Decancer instance. The instance here stores the supported confusables in pointers instead of arrays.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,norun
/// extern crate decancer;
/// use decancer::Decancer;
///
/// let instance = Decancer::new();
/// let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// assert_eq!(output, String::from("very funny text"));
/// ```
#[derive(Copy, Clone)]
#[must_use]
pub struct Decancer {
  numerical: BinaryArray<u32>,
  misc_case_sensitive: MiscCaseSensitive,
  alphabetical_pattern: BinaryArray<u32>,
  alphabetical: Alphabetical,
  misc: Misc,
  similar: Similar,
}

impl Decancer {
  /// Creates a Decancer instance. The constructor here is const and never fails, which means that you can put it as a constant global variable.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// ```
  ///
  /// Or use it in a global state:
  ///
  /// ```rust,norun
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// const DECANCER: Decancer = Decancer::new();
  /// ```
  pub const fn new() -> Self {
    Self {
      numerical: confusables::numerical(),
      misc_case_sensitive: confusables::misc_case_sensitive(),
      alphabetical_pattern: confusables::alphabetical_pattern(),
      alphabetical: confusables::alphabetical(),
      misc: confusables::misc(),
      similar: confusables::similar(),
    }
  }

  #[inline(always)]
  fn similar(&self, a: u16, b: u16) -> bool {
    a == b
      || ((a <= 0xFF)
        && (b <= 0xFF)
        && self
          .similar
          .iter()
          .any(|x| x.contains(a as _) && x.contains(b as _)))
  }

  /// Checks if string B is in string A. This one is more accurate than A.contains(B) because it also checks if the strings are similar. (e.g `1` with `i`... etc)
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  ///
  /// if instance.contains(output, "badwordhere") {
  ///   println!("LANGUAGE!!!");
  /// }
  /// ```
  #[must_use]
  pub fn contains<A: AsRef<str>, B: AsRef<str>>(&self, a: A, b: B) -> bool {
    let b_len = b.as_ref().encode_utf16().count();

    let mut j = 0usize;
    for (x, y) in a.as_ref().encode_utf16().zip(b.as_ref().encode_utf16()) {
      if self.similar(x, y) {
        j += 1;

        if j == b_len {
          return true;
        }
      } else {
        j = 0;
      }
    }

    false
  }

  fn cure_inner<I: Iterator<Item = u32>>(&self, it: I, min_size: usize) -> String {
    let mut output = String::with_capacity(min_size);

    it.filter(|&x| {
      ((x > 31 && x < 127) || (x > 159 && x < 0x300) || x > 0x36F)
        && x != 0x20E3
        && x != 0xFE0F
        && x != 0x489
    })
    .for_each(|x| {
      for num in self.numerical.iter() {
        if x >= num && x <= (num + 9) {
          return output.push(unsafe { char::from_u32_unchecked(x - num + 0x30) });
        }
      }

      for (key, value) in self.misc_case_sensitive.iter() {
        if value.contains(x) {
          for k in key.iter() {
            output.push(k as char);
          }

          return;
        }
      }

      if let Some(c22) = char::from_u32(x) {
        c22.to_lowercase().for_each(|c2| {
          let c = c2 as u32;

          for pat in self.alphabetical_pattern.iter() {
            if c >= pat && c <= (pat + 25) {
              return output.push(unsafe { char::from_u32_unchecked(c - pat + 0x61) });
            }
          }

          for (i, arr) in self.alphabetical.iter().enumerate() {
            if arr.contains(c) {
              return output.push(unsafe { char::from_u32_unchecked((i as u32) + 0x61) });
            }
          }

          for (a, b) in self.misc.iter() {
            if b.contains(c) {
              return output.push(a as char);
            }
          }

          if let Some(t) = char::from_u32(c) {
            output.push(t);
          }
        });
      }
    });

    output.retain(|c2| {
      let (a, b) = charcodes(c2 as _);

      if a != 0xFFFD && (a < 0xD800 || a > 0xDB7F) && a < 0xFFF0 {
        if let Some(b2) = b {
          b2 != 0xFFFD && (b2 < 0xD800 || b2 > 0xDB7F) && b2 < 0xFFF0
        } else {
          true
        }
      } else {
        false
      }
    });

    output
  }

  /// Cures a list of UTF-16/UCS-2 `u16` character codes.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// let output = instance.cure_utf16([0x0076, 0xFF25, 0x24E1, 0xD835,
  ///                                   0xDD02, 0x0020, 0xD835, 0xDD3D,
  ///                                   0xD835, 0xDD4C, 0x0147, 0x2115,
  ///                                   0xFF59, 0x0020, 0x0163, 0x4E47,
  ///                                   0xD835, 0xDD4F, 0xD835, 0xDCE3]);
  ///
  /// assert_eq!(output, String::from("very funny text"));
  /// ```
  #[must_use]
  pub fn cure_utf16<I: IntoIterator<Item = u16>>(&self, iter: I) -> String {
    let it = iter.into_iter();
    let (lower_bound, _) = it.size_hint();

    self.cure_inner(Codepoints::from(it), lower_bound)
  }

  /// Cures a list of UTF-32 `u32`s or Unicode codepoints.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// let output = instance.cure_utf32([0x76, 0xFF25, 0x24E1, 0x1D502, 0x20,
  ///                                   0x1D53D, 0x1D54C, 0x147, 0x2115, 0xFF59,
  ///                                   0x20, 0x163, 0x4E47, 0x1D54F, 0x1D4E3]);
  ///
  /// assert_eq!(output, String::from("very funny text"));
  /// ```
  #[must_use]
  pub fn cure_utf32<I: IntoIterator<Item = u32>>(&self, iter: I) -> String {
    let it = iter.into_iter();
    let (lower_bound, _) = it.size_hint();

    self.cure_inner(it, lower_bound)
  }

  /// Cures a list of `char`s.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// let output = instance.cure_utf32(['v', 'Ｅ', 'ⓡ', '𝔂', ' ',
  ///                                   '𝔽', '𝕌' , 'Ň', 'ℕ', 'ｙ',
  ///                                   ' ', 'ţ', '乇', '𝕏' , '𝓣']);
  ///
  /// assert_eq!(output, String::from("very funny text"));
  /// ```
  #[must_use]
  pub fn cure_chars<I: IntoIterator<Item = char>>(&self, iter: I) -> String {
    let it = iter.into_iter();
    let (lower_bound, _) = it.size_hint();

    self.cure_inner(it.map(|x| x as u32), lower_bound)
  }

  /// Cures a string.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  ///
  /// assert_eq!(output, String::from("very funny text"));
  /// ```
  #[inline(always)]
  #[must_use]
  pub fn cure<S: AsRef<str>>(&self, s: S) -> String {
    self.cure_utf16(s.as_ref().encode_utf16())
  }
}

impl Default for Decancer {
  /// Creates a Decancer instance. The constructor here is const and never fails.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// ```
  #[inline(always)]
  fn default() -> Self {
    Self::new()
  }
}

#[test]
fn it_works() {
  let instance = Decancer::new();
  let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  assert_eq!(output, String::from("very funny text"));
}