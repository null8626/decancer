use crate::{codepoints::Codepoint, Translation};
use paste::paste;
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
pub struct Options(pub(crate) u32);

macro_rules! options {
  ($(
    $(#[$extra_meta:meta])*
    $idx:literal: $name:ident,
  )*) => {
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

macro_rules! retain {
  ($(
    $idx:literal: $name:ident,
  )*) => {
    paste! {
      options! {
        $(
          #[doc = concat!("Prevents decancer from curing all ", stringify!($name), " characters.")]
          $idx: [<retain_ $name>],
        )*
      }
    }
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

    /// Prevents decancer from curing all katakana and hiragana characters.
    ///
    /// **NOTE:** To also provent decancer from curing kanji characters, use [`retain_chinese`][Options::retain_chinese].
    18: retain_japanese,

    /// Prevents decancer from curing all emojis.
    21: retain_emojis,

    /// Prevents decancer from curing all turkish characters.
    ///
    /// **NOTE:** To also prevent decancer from curing [the uppercase dotted i character](https://en.wikipedia.org/wiki/İ) (`İ`), use [`retain_capitalization`][Options::retain_capitalization].
    22: retain_turkish,

    /// Removes all non-ASCII characters from the result.
    23: ascii_only,

    /// Removes all non-alphanumeric characters from the result.
    24: alphanumeric_only,
  }

  retain! {
    3: greek,
    4: cyrillic,
    5: hebrew,
    6: arabic,
    7: devanagari,
    8: bengali,
    9: armenian,
    10: gujarati,
    11: tamil,
    12: thai,
    13: lao,
    14: burmese,
    15: khmer,
    16: mongolian,
    17: chinese,
    19: korean,
    20: braille,
  }

  #[cfg(feature = "options")]
  pub(crate) const fn is(self, attribute_idx: u8) -> bool {
    (self.0 & (1 << attribute_idx as u32)) != 0
  }

  #[cfg(feature = "options")]
  pub(crate) const fn refuse_cure(self, attributes: u8) -> bool {
    let locale = attributes >> 2;

    ((attributes & 1) != 0 && self.is(2))
      || ((attributes & 2) != 0 && self.is(22))
      || locale > 2 && self.is(locale)
  }

  pub(crate) fn translate(self, code: u32, offset: i32, mut end: i32) -> Option<Translation> {
    let mut start = 0;

    while start <= end {
      let mid = (start + end) / 2;
      let codepoint = Codepoint::at(offset + (mid * 6));
      #[cfg(feature = "options")]
      let mat = codepoint.matches(code, self);

      #[cfg(not(feature = "options"))]
      let mat = codepoint.matches(code);

      match mat {
        Some(ord) => match ord {
          Ordering::Equal => return Some(codepoint.translation(code)),
          Ordering::Greater => start = mid + 1,
          Ordering::Less => end = mid - 1,
        },

        // could've just used ? but Rust doesn't allow it in a const fn
        None => break,
      };
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
