use crate::{codepoints::Codepoint, Translation};
use paste::paste;
use std::{cmp::Ordering, mem::transmute};

/// A configuration struct where you can customize decancer's behavior.
///
/// By default, decancer cures as much characters as possible and turns all of the output characters to lowercase.
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
        Self(self.0 | (1 << $idx))
      }
    )*
  };
}

macro_rules! retain {
  ($(
    $idx:literal: $name:ident($input:literal, $cured:literal),
  )*) => {
    paste! {
      options! {
        $(
          #[doc = concat!("Prevents decancer from curing all ", stringify!($name), " characters.")]
          ///
          /// ```rust
          /// use decancer::Options;
          ///
          #[doc = concat!(" assert_eq!(decancer::cure!(\"", $input, "\").unwrap(), \"", $cured, "\");")]
          ///
          #[doc = concat!(" let options = Options::default().retain_", stringify!($name), "();")]
          ///
          #[doc = concat!(" assert_eq!(decancer::cure(\"", $input, "\", options).unwrap(), \"", $input, "\");")]
          /// ```
          $idx: [<retain_ $name>],
        )*
      }
    }
  };
}

impl Options {
  /// Creates a new configuration where every option is enabled. This is useful if you want to use decancer solely for formatting.
  pub const fn formatter() -> Self {
    Self((1 << 21) - 1)
  }

  /// Creates a new configuration that prevents decancer from curing characters from major foreign writing systems.
  pub const fn pure_homoglyph() -> Self {
    Self(((1 << 21) - 1) ^ 0b11)
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
    /// ```rust
    /// use decancer::Options;
    ///
    /// assert_eq!(decancer::cure!("àáâãäåèéêëìíîïñòóôõöùúûü").unwrap(), "aaaaaaeeeeiiiinooooouuuu");
    ///
    /// let options = Options::default().retain_diacritics();
    ///
    /// assert_eq!(decancer::cure("àáâãäåèéêëìíîïñòóôõöùúûü", options).unwrap(), "àáâãäåèéêëìíîïñòóôõöùúûü");
    /// ```
    ///
    /// **NOTE:** Decancer can still cure standalone diacritic characters, which is used in [Zalgo texts](https://en.wikipedia.org/wiki/Zalgo_text).
    2: retain_diacritics,

    /// Prevents decancer from curing all katakana and hiragana characters.
    ///
    /// ```rust
    /// use decancer::Options;
    ///
    /// assert_eq!(decancer::cure!("のひびぴるろん゜ァィ").unwrap(), "9uuu33hopt");
    ///
    /// let options = Options::default().retain_japanese();
    ///
    /// assert_eq!(decancer::cure("のひびぴるろん゜ァィ", options).unwrap(), "のひびぴるろん゜ァィ");
    /// ```
    ///
    /// **NOTE:** To also provent decancer from curing kanji characters, use [`retain_chinese`][Options::retain_chinese].
    18: retain_japanese,
  }

  retain! {
    3: greek("βγδεηθικλμνξοπρςστυ", "by6en0ikauveonpcotu"),
    4: cyrillic("абвгдежзийклмхцчшщъыь", "a6braex3nnknmxu4wwbbib"),
    5: hebrew("׀׆אבגדהוזחטךכםמןנסעפץצקרשתװ", "icxda7niino7dooijovgyyprwnii"),
    6: arabic("١٤٥٦٧٨", "ieo7va"),
    7: devanagari("टभऽ।॥०१२३५६७८॰", "c4siiioqr34e9co"),
    8: bengali("ঀৎ০২৪৭৮৷৸", "qeo28qbih"),
    9: armenian("աբգդեզէթժիլխծկհձղ", "wpqntqtpdhlno4han"),
    10: gujarati("કઙટડપમરવષઽ૦૧૨૩૫૬૭૮૯૰૱", "ss2su4rq4soqr34e9ceo30"),
    11: tamil("உஎடணபயறளஶௗ௦௭௰௱", "96tl6ootuwm6nuo6no6twm"),
    12: thai("กคงฑดตถทนบปผฝพฟ", "navnaannuuuwwww"),
    13: lao("ກຂງຊຍຖນບປຝພຟມເແ໐໓໗", "n2g2unuuuwwwucccodn"),
    14: burmese("ဗဘယလဝသဟဢဿ၀၁၂၃", "oooooooooooooooooocjr"),
    15: khmer("ឞសអឣឤឧឨឩឪឫឬឱ", "unhhhi2222yy2"),
    16: mongolian("᠃᠉᠎", "\\\"\\\" "),
    17: chinese("之乍乏乙乚乜九乞亅了", "zezzlenzj7"),
    19: korean("ᄂᄄᄅᄆᄊᄋᄏᄐᄔᄕ", "lcc2omofelllc"),
    20: braille("⡅⡇⡖⡟⡯⡸⡹⢗⢨⣖⣫⣻", "ilrpfj2ticsg"),
  }

  pub(crate) const fn is(self, attribute_idx: u8) -> bool {
    (self.0 & (1 << attribute_idx as u32)) != 0
  }

  pub(crate) const fn refuse_cure(self, attributes: u8) -> bool {
    let locale = attributes >> 1;

    (unsafe { transmute(attributes & 1) } && self.is(2)) || (locale > 2 && self.is(locale))
  }

  pub(crate) const fn translate(self, code: u32, offset: i32, mut end: i32) -> Option<Translation> {
    let mut start = 0;

    while start <= end {
      let mid = (start + end) / 2;
      let codepoint = Codepoint::at(offset + (mid * 6));

      match codepoint.matches(code, self) {
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