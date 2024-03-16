use crate::{codepoints::Codepoint, Translation};
use paste::paste;
use std::cmp::Ordering;
#[cfg(feature = "customization")]
use std::mem::transmute;

/// A configuration struct where you can customize decancer's behavior.
///
/// By default, decancer cures as much characters as possible and turns all of the output characters to lowercase.
///
/// If you don't plan on using this struct and only using decancer's defaults, it's recommended to disable the default `customization` feature flag to optimize away unnecessary option checks.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Options(pub(crate) u32);

macro_rules! options {
  ($(
    $(#[$extra_meta:meta])*
    $idx:literal: $name:ident,
  )*) => {
    $(
      $(#[$extra_meta])*
      pub const fn $name(self) -> Self {
        #[cfg(feature = "customization")]
        return Self(self.0 | (1 << $idx));
        
        #[cfg(not(feature = "customization"))]
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
  /// Creates a new configuration where every option is enabled. This is useful if you want to use decancer solely for formatting.
  pub const fn formatter() -> Self {
    #[cfg(feature = "customization")]
    return Self((1 << 22) - 1);
    
    #[cfg(not(feature = "customization"))]
    return Self(0);
  }

  /// Creates a new configuration that prevents decancer from curing characters from major foreign writing systems.
  pub const fn pure_homoglyph() -> Self {
    #[cfg(feature = "customization")]
    return Self(((1 << 22) - 1) ^ 0x200003);
    
    #[cfg(not(feature = "customization"))]
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

  #[cfg(feature = "customization")]
  pub(crate) const fn is(self, attribute_idx: u8) -> bool {
    (self.0 & (1 << attribute_idx as u32)) != 0
  }

  #[cfg(feature = "customization")]
  #[allow(clippy::transmute_int_to_bool)]
  pub(crate) const fn refuse_cure(self, attributes: u8) -> bool {
    let locale = attributes >> 1;

    (unsafe { transmute(attributes & 1) } && self.is(2)) || (locale > 2 && self.is(locale))
  }

  pub(crate) const fn translate(self, code: u32, offset: i32, mut end: i32) -> Option<Translation> {
    let mut start = 0;

    while start <= end {
      let mid = (start + end) / 2;
      let codepoint = Codepoint::at(offset + (mid * 6));
      #[cfg(feature = "customization")]
      let mat = codepoint.matches(code, self);
      
      #[cfg(not(feature = "customization"))]
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
