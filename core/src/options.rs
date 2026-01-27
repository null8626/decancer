// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use super::{Translation, codepoints::Codepoint};
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
#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Options(pub(super) u32);

macro_rules! options {
  ($(
    $(#[$extra_meta:meta])*
    $idx:literal: $name:ident
  ),*) => {
    $(
      $(#[$extra_meta])*
      #[cfg(feature = "options")]
      pub const fn $name(self) -> Self {
        Self(self.0 | (1 << $idx))
      }
    )*
  };
}

impl Options {
  /// A configuration where every option is enabled.
  #[cfg(feature = "options")]
  pub const ALL: Self = Self(0x3ffffff);

  /// A configuration that prevents decancer from curing characters from major foreign writing systems, including diacritics.
  #[cfg(feature = "options")]
  pub const PURE_HOMOGLYPH: Self = Self(0x7ffff8);

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
    ///
    /// match decancer::cure_char('🆐', options) {
    ///   Translation::String(dj) => assert_eq!(dj, "dj"),
    ///
    ///   _ => unreachable!("cure_char 🆐 should always return a Translation::String")
    /// }
    /// ```
    0: retain_capitalization,

    /// Prevents decancer from applying the [Unicode Bidirectional Algorithm](https://en.wikipedia.org/wiki/Bidirectional_text). Use this **only** when you don't expect any right-to-left characters. Enabling this option has no effect if it's called on [`cure_char`][super::cure_char()].
    ///
    /// **NOTE:** This speeds up the function call, but **can break [right-to-left characters](https://en.wikipedia.org/wiki/Bidirectional_text)**. It's highly recommended to also use [`retain_arabic`][Options::retain_arabic] and [`retain_hebrew`][Options::retain_hebrew].
    1: disable_bidi,

    /// Prevents decancer from applying leetspeak comparisons in [`CuredString`][super::CuredString]'s comparison methods.
    #[cfg(feature = "leetspeak")]
    2: disable_leetspeak,

    /// Prevents decancer from curing characters *with* diacritics or accents.
    ///
    /// **NOTE:** Decancer can still cure standalone diacritic characters, which is used in [Zalgo texts](https://en.wikipedia.org/wiki/Zalgo_text).
    3: retain_diacritics,

    /// Prevents decancer from curing all greek characters.
    4: retain_greek,

    /// Prevents decancer from curing all cyrillic characters.
    5: retain_cyrillic,

    /// Prevents decancer from curing all hebrew characters.
    6: retain_hebrew,

    /// Prevents decancer from curing all arabic characters.
    7: retain_arabic,

    /// Prevents decancer from curing all devanagari characters.
    8: retain_devanagari,

    /// Prevents decancer from curing all bengali characters.
    9: retain_bengali,

    /// Prevents decancer from curing all armenian characters.
    10: retain_armenian,

    /// Prevents decancer from curing all gujarati characters.
    11: retain_gujarati,

    /// Prevents decancer from curing all tamil characters.
    12: retain_tamil,

    /// Prevents decancer from curing all thai characters.
    13: retain_thai,

    /// Prevents decancer from curing all lao characters.
    14: retain_lao,

    /// Prevents decancer from curing all burmese characters.
    15: retain_burmese,

    /// Prevents decancer from curing all khmer characters.
    16: retain_khmer,

    /// Prevents decancer from curing all mongolian characters.
    17: retain_mongolian,

    /// Prevents decancer from curing all chinese characters.
    18: retain_chinese,

    /// Prevents decancer from curing all katakana and hiragana characters.
    ///
    /// **NOTE:** To also prevent decancer from curing kanji characters, use [`retain_chinese`][Options::retain_chinese].
    19: retain_japanese,

    /// Prevents decancer from curing all korean characters.
    20: retain_korean,

    /// Prevents decancer from curing all braille characters.
    21: retain_braille,

    /// Prevents decancer from curing all emojis.
    22: retain_emojis,

    /// Prevents decancer from curing all turkish characters.
    ///
    /// **NOTE:** To also prevent decancer from curing [the uppercase dotted i character](https://en.wikipedia.org/wiki/İ) (`İ`), use [`retain_capitalization`][Options::retain_capitalization].
    23: retain_turkish,

    /// Removes all non-ASCII characters from the result.
    24: ascii_only,

    /// Removes all non-alphanumeric characters from the result.
    25: alphanumeric_only
  }

  #[cfg(feature = "options")]
  pub(super) const fn is(self, attribute_idx: u8) -> bool {
    (self.0 & (1 << attribute_idx as u32)) != 0
  }

  #[cfg(feature = "options")]
  pub(super) const fn refuse_cure(self, attributes: u8) -> bool {
    let locale = attributes >> 2;

    ((attributes & 1) != 0 && self.is(3))
      || ((attributes & 2) != 0 && self.is(23))
      || locale > 3 && self.is(locale)
  }

  pub(super) fn translate(self, code: u32, offset: i32, mut end: i32) -> Option<Translation> {
    let mut start = 0;

    while start <= end {
      let mid = start.midpoint(end);
      let codepoint = Codepoint::at(offset + (mid * 6));
      #[cfg(feature = "options")]
      let ord = codepoint.matches(code, self)?;

      #[cfg(not(feature = "options"))]
      let ord = codepoint.matches(code)?;

      match ord {
        Ordering::Equal => {
          return Some(codepoint.translation(
            code,
            #[cfg(all(feature = "leetspeak", feature = "options"))]
            self.is(2),
          ));
        },

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
