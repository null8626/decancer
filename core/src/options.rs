// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use super::{codepoints::Codepoint, Translation};
use std::cmp::Ordering;

/// A configuration struct where you can customize decancer's behavior.
///
/// By default, decancer cures as much characters as possible and turns all the output characters to lowercase.
///
/// If you don't plan on using this struct and only using decancer's defaults, it's recommended to disable the default `options` feature flag to optimize away unnecessary option checks.
///
/// ```rust
/// use decancer::Options;
///
/// // by default, all options are disabled
/// let _options = Options::default();
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct Options(pub(super) u32);

macro_rules! options {
  ($(
    $(#[$extra_meta:meta])*
    $idx:literal: $name:ident
  ),*) => {
    $(
      $(#[$extra_meta])*
      #[cfg_attr(not(feature = "options"), cold)]
      pub const fn $name(self) -> Self {
        #[cfg(feature = "options")]
        return Self(self.0 | (1 << $idx));

        #[cfg(not(feature = "options"))]
        return self;
      }
    )*
  };
}

impl Options {
  /// Creates a new configuration where every option is enabled.
  #[cfg_attr(not(feature = "options"), cold)]
  pub const fn all() -> Self {
    #[cfg(feature = "options")]
    return Self(0x1ffffff);

    #[cfg(not(feature = "options"))]
    return Self(0);
  }

  /// Creates a new configuration that prevents decancer from curing characters from major foreign writing systems, including diacritics.
  #[cfg_attr(not(feature = "options"), cold)]
  pub const fn pure_homoglyph() -> Self {
    #[cfg(feature = "options")]
    return Self(0x3ffffc);

    #[cfg(not(feature = "options"))]
    return Self(0);
  }

  options! {
    /// Prevents decancer from changing all characters to lowercase. Therefore, if the input character is in uppercase, the output character will be in uppercase as well.
    ///
    /// **NOTE:** Many confusables are neither an uppercase or a lowercase character. Therefore, the decancer defaults to displaying the translation **in lowercase**:
    ///
    /// ```rust
    /// use decancer::{Translation, Options};
    /// use std::borrow::Cow;
    ///
    /// let options = Options::default()
    ///   .retain_capitalization();
    ///
    /// assert_eq!('🆐'.to_lowercase().collect::<String>(), '🆐'.to_uppercase().collect::<String>());
    /// assert_eq!(decancer::cure_char('🆐', options), Translation::String(Cow::Borrowed("dj")));
    /// ```
    0: retain_capitalization,

    /// Prevents decancer from applying the [Unicode Bidirectional Algorithm](https://en.wikipedia.org/wiki/Bidirectional_text). Use this **only** when you don't expect any right-to-left characters. Enabling this option has no effect if it's called on [`cure_char`][crate::cure_char()].
    ///
    /// **NOTE:** This speeds up the function call, but **can break [right-to-left characters](https://en.wikipedia.org/wiki/Bidirectional_text)**. It's highly recommended to also use [`retain_arabic`][Options::retain_arabic] and [`retain_hebrew`][Options::retain_hebrew].
    1: disable_bidi,

    /// Prevents decancer from curing characters *with* diacritics or accents.
    ///
    /// **NOTE:** Decancer can still cure standalone diacritic characters, which is used in [Zalgo texts](https://en.wikipedia.org/wiki/Zalgo_text).
    2: retain_diacritics,

    /// Prevents decancer from curing all greek characters.
    3: retain_greek,

    /// Prevents decancer from curing all cyrillic characters.
    4: retain_cyrillic,

    /// Prevents decancer from curing all hebrew characters.
    5: retain_hebrew,

    /// Prevents decancer from curing all arabic characters.
    6: retain_arabic,

    /// Prevents decancer from curing all devanagari characters.
    7: retain_devanagari,

    /// Prevents decancer from curing all bengali characters.
    8: retain_bengali,

    /// Prevents decancer from curing all armenian characters.
    9: retain_armenian,

    /// Prevents decancer from curing all gujarati characters.
    10: retain_gujarati,

    /// Prevents decancer from curing all tamil characters.
    11: retain_tamil,

    /// Prevents decancer from curing all thai characters.
    12: retain_thai,

    /// Prevents decancer from curing all lao characters.
    13: retain_lao,

    /// Prevents decancer from curing all burmese characters.
    14: retain_burmese,

    /// Prevents decancer from curing all khmer characters.
    15: retain_khmer,

    /// Prevents decancer from curing all mongolian characters.
    16: retain_mongolian,

    /// Prevents decancer from curing all chinese characters.
    17: retain_chinese,

    /// Prevents decancer from curing all katakana and hiragana characters.
    ///
    /// **NOTE:** To also prevent decancer from curing kanji characters, use [`retain_chinese`][Options::retain_chinese].
    18: retain_japanese,

    /// Prevents decancer from curing all korean characters.
    19: retain_korean,

    /// Prevents decancer from curing all braille characters.
    20: retain_braille,

    /// Prevents decancer from curing all emojis.
    21: retain_emojis,

    /// Prevents decancer from curing all turkish characters.
    ///
    /// **NOTE:** To also prevent decancer from curing [the uppercase dotted i character](https://en.wikipedia.org/wiki/İ) (`İ`), use [`retain_capitalization`][Options::retain_capitalization].
    22: retain_turkish,

    /// Removes all non-ASCII characters from the result.
    23: ascii_only,

    /// Removes all non-alphanumeric characters from the result.
    24: alphanumeric_only
  }

  #[cfg(feature = "options")]
  pub(super) const fn is(self, attribute_idx: u8) -> bool {
    (self.0 & (1 << attribute_idx as u32)) != 0
  }

  #[cfg(feature = "options")]
  pub(super) const fn refuse_cure(self, attributes: u8) -> bool {
    let locale = attributes >> 2;

    ((attributes & 1) != 0 && self.is(2))
      || ((attributes & 2) != 0 && self.is(22))
      || locale > 2 && self.is(locale)
  }

  pub(super) fn translate(self, code: u32, offset: i32, mut end: i32) -> Option<Translation> {
    let mut start = 0;

    while start <= end {
      let mid = (start + end) / 2;
      let codepoint = Codepoint::at(offset + (mid * 6));
      #[cfg(feature = "options")]
      let ord = codepoint.matches(code, self)?;

      #[cfg(not(feature = "options"))]
      let ord = codepoint.matches(code)?;

      match ord {
        Ordering::Equal => return Some(codepoint.translation(code)),

        Ordering::Greater => start = mid + 1,

        Ordering::Less => end = mid - 1,
      }
    }

    None
  }
}

#[doc(hidden)]
#[cfg(feature = "options")]
impl From<u32> for Options {
  #[inline(always)]
  fn from(value: u32) -> Self {
    Self(value)
  }
}
