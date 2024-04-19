#![doc = include_str!("../README.md")]
#![allow(clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod bidi;
mod codepoints;
#[cfg(feature = "leetspeak")]
mod leetspeak;
mod options;
mod similar;
mod string;
#[cfg(test)]
mod tests;
mod translation;
mod util;

use bidi::{Class, Level, Paragraph};
pub use options::Options;
pub use similar::Matcher;
pub use string::CuredString;
pub use translation::Translation;

use codepoints::{
  CASE_SENSITIVE_CODEPOINTS_COUNT, CASE_SENSITIVE_CODEPOINTS_OFFSET, CODEPOINTS_COUNT,
};

use util::{error_enum, is_none};
#[cfg(feature = "options")]
use util::{is_alphanumeric, is_special_rtl};

error_enum! {
  /// An error enum for unicode bidi errors caused by malformed string inputs.
  #[repr(u8)]
  #[derive(Copy, Clone, Debug)]
  pub enum Error {
    /// Attempted to create a unicode bidi level that exceeds `MAX_EXPLICIT_DEPTH` (125).
    LevelExplicitOverflow,
    /// Attempted to create a unicode bidi level that exceeds `MAX_IMPLICIT_DEPTH` (126).
    LevelImplicitOverflow,
    /// Attempted to lower a unicode bidi level that is already zero.
    LevelModificationUnderflow,
    /// Attempted to raise a unicode bidi level that is already at `MAX_IMPLICIT_DEPTH` (126).
    LevelModificationOverflow,
  }
}

fn cure_char_inner(code: u32, options: Options) -> Translation {
  let code_lowercased = char::from_u32(code)
    .and_then(|character| character.to_lowercase().next())
    .unwrap() as _;

  let is_case_sensitive = code != code_lowercased;

  #[cfg(feature = "options")]
  let retain_capitalization = options.is(0);

  #[cfg(feature = "options")]
  let ascii_only = options.is(22);

  #[cfg(feature = "options")]
  let alphanumeric_only = options.is(23);

  #[cfg(feature = "options")]
  let default_output = if is_case_sensitive && retain_capitalization {
    code
  } else {
    code_lowercased
  };

  #[cfg(not(feature = "options"))]
  let default_output = code_lowercased;

  if default_output < 0x80 {
    #[cfg(feature = "options")]
    if alphanumeric_only && !is_alphanumeric(default_output) {
      return Translation::None;
    }

    return Translation::character(default_output);
  } else if is_case_sensitive {
    #[cfg_attr(not(feature = "options"), allow(unused_mut))]
    if let Some(mut translation) = options.translate(
      code,
      CASE_SENSITIVE_CODEPOINTS_OFFSET as _,
      CASE_SENSITIVE_CODEPOINTS_COUNT as _,
    ) {
      #[cfg(feature = "options")]
      if retain_capitalization {
        translation = translation.into_uppercase();
      }

      #[cfg(feature = "options")]
      return translation.ensure_stripped_if(ascii_only, alphanumeric_only);

      #[cfg(not(feature = "options"))]
      return translation;
    }
  }

  #[cfg(feature = "options")]
  match options.translate(code_lowercased, 6, CODEPOINTS_COUNT as _) {
    Some(translation) => translation.ensure_stripped_if(ascii_only, alphanumeric_only),
    None => {
      if ascii_only || alphanumeric_only {
        Translation::None
      } else {
        Translation::character(default_output)
      }
    },
  }

  #[cfg(not(feature = "options"))]
  options
    .translate(code_lowercased, 6, CODEPOINTS_COUNT as _)
    .unwrap_or_else(|| Translation::character(default_output))
}

/// Cures a single character/unicode codepoint with the specified [`Options`].
///
/// To use this function with decancer's default options, use [the `cure_char` macro][cure_char!] instead.
pub fn cure_char<C: Into<u32>>(code: C, options: Options) -> Translation {
  let code = code.into();

  if is_none(code) {
    Translation::None
  } else {
    match Class::new(code) {
      Some(Class::WS) => Translation::character(if code > 0x7f { 0x20 } else { code }),
      None => Translation::None,
      _ => cure_char_inner(code, options),
    }
  }
}

/// Cures a single character/unicode codepoint with decancer's default options.
///
/// Output will always be in lowercase.
///
/// If you plan on only using this macro, it's recommended to disable the default `options` feature flag to optimize away unnecessary option checks.
///
/// This macro expands to:
///
/// ```rust,ignore
/// decancer::cure_char(code, decancer::Options::default());
/// ```
///
/// For more information, see [the `cure_char` function][cure_char()].
#[macro_export]
macro_rules! cure_char {
  ($code:expr) => {
    $crate::cure_char($code, $crate::Options::default())
  };
}

/// Formats a single character/unicode codepoint by only removing [pure homoglyphs][Options::pure_homoglyph] while still [retaining capitalization][Options::retain_capitalization].
///
/// This macro expands to:
///
/// ```rust,ignore
/// decancer::cure_char(code, decancer::Options::formatter());
/// ```
///
/// For more information, see [`Options::formatter`].
#[macro_export]
macro_rules! format_char {
  ($code:expr) => {
    $crate::cure_char($code, $crate::Options::formatter())
  };
}

fn first_cure_pass(input: &str) -> (String, Vec<Class>, Vec<Paragraph>) {
  let mut refined_input = String::with_capacity(input.len());

  let mut original_classes = Vec::with_capacity(input.len());
  let mut isolate_stack = Vec::new();

  let mut paragraphs = Vec::new();
  let mut paragraph_start = 0;
  let mut paragraph_level: Option<Level> = None;
  let mut pure_ltr = true;

  let mut idx = 0;

  for codepoint in input.chars() {
    let mut character_len = codepoint.len_utf8();
    let mut codepoint = codepoint as u32;

    if !is_none(codepoint) {
      if let Some(class) = Class::new(codepoint) {
        if class == Class::WS && codepoint > 0x7f {
          character_len = 1;
          codepoint = 0x20;
        }

        original_classes.resize(original_classes.len() + character_len, class);

        match class {
          Class::B => {
            let paragraph_end = idx + character_len;

            paragraphs.push(Paragraph {
              range: paragraph_start..paragraph_end,
              level: paragraph_level.unwrap_or(Level::ltr()),
              pure_ltr,
            });

            paragraph_start = paragraph_end;
            pure_ltr = true;
            isolate_stack.clear();
            paragraph_level = None;
          },

          Class::L | Class::R | Class::AL => {
            if class != Class::L {
              pure_ltr = false;
            }

            match isolate_stack.last() {
              Some(&start_idx) => {
                if original_classes[start_idx] == Class::FSI {
                  let new_class = if class == Class::L {
                    Class::LRI
                  } else {
                    Class::RLI
                  };

                  for j in 0..3 {
                    original_classes[start_idx + j] = new_class;
                  }
                }
              },

              None => {
                if paragraph_level.is_none() {
                  paragraph_level.replace(if class == Class::L {
                    Level::ltr()
                  } else {
                    Level::rtl()
                  });
                }
              },
            }
          },

          Class::AN | Class::LRE | Class::RLE | Class::LRO | Class::RLO => {
            pure_ltr = false;
          },

          Class::RLI | Class::LRI | Class::FSI => {
            pure_ltr = false;
            isolate_stack.push(idx);
          },

          Class::PDI => {
            isolate_stack.pop();
          },

          _ => {},
        }

        refined_input.push(char::from_u32(codepoint).unwrap());

        idx += character_len;
      }
    }
  }

  if paragraph_start < idx {
    paragraphs.push(Paragraph {
      range: paragraph_start..idx,
      level: paragraph_level.unwrap_or(Level::ltr()),
      pure_ltr,
    });
  }

  (refined_input, original_classes, paragraphs)
}

pub(crate) fn cure_reordered(input: &str, options: Options) -> Result<String, Error> {
  let (refined_input, original_classes, paragraphs) = first_cure_pass(input);

  let mut levels = Vec::with_capacity(refined_input.len());
  let mut processing_classes = original_classes.clone();
  let mut output = String::with_capacity(refined_input.len());

  for paragraph in paragraphs.iter() {
    levels.resize(levels.len() + paragraph.range.len(), paragraph.level);

    if paragraph.level.0 != 0 || !paragraph.pure_ltr {
      let input = paragraph.sliced(&refined_input);
      let original_classes = paragraph.sliced(&original_classes);
      let processing_classes = paragraph.sliced_mut(&mut processing_classes);
      let levels = paragraph.sliced_mut(&mut levels);

      paragraph.compute_explicit(input, original_classes, processing_classes, levels);

      for sequence in paragraph.isolating_run_sequences(levels, original_classes) {
        sequence.resolve_implicit_weak(input, processing_classes);
        sequence.resolve_implicit_neutral(input, processing_classes, levels);
      }

      for j in 0..levels.len() {
        match (levels[j].is_rtl(), processing_classes[j]) {
          (false, Class::AN) | (false, Class::EN) => levels[j].raise(2)?,
          (false, Class::R) | (true, Class::L) | (true, Class::EN) | (true, Class::AN) => {
            levels[j].raise(1)?
          },
          _ => {},
        }

        if original_classes[j].removed_by_x9() {
          levels[j] = if j > 0 {
            levels[j - 1]
          } else {
            paragraph.level
          };
        }
      }
    }
  }

  for paragraph in paragraphs {
    let (revised_levels, runs) =
      paragraph.visual_runs(&refined_input, &original_classes, &levels)?;

    for run in runs {
      let text = &refined_input[run.clone()];

      if revised_levels[run.start].is_rtl() {
        for c in text.chars().rev() {
          output += cure_char_inner(c as _, options);
        }
      } else {
        for c in text.chars() {
          output += cure_char_inner(c as _, options);
        }
      }
    }
  }

  Ok(output)
}

/// Cures a string with the specified [`Options`].
///
/// To use this function with decancer's default options, use [the `cure` macro][cure!] instead.
///
/// # Errors
///
/// Errors if the string is malformed to the point where it's not possible to apply unicode's [bidirectional algorithm](https://en.wikipedia.org/wiki/Bidirectional_text) to it. This error is possible if [`Options::disable_bidi`] is disabled.
pub fn cure(input: &str, options: Options) -> Result<CuredString, Error> {
  Ok(CuredString({
    #[cfg(feature = "options")]
    if options.is(1) {
      input
        .chars()
        .filter(|&character| !is_special_rtl(character as _))
        .fold(
          String::with_capacity(input.len()),
          |mut output, character| {
            output += cure_char(character, options);
            output
          },
        )
    } else {
      cure_reordered(input, options)?
    }

    #[cfg(not(feature = "options"))]
    cure_reordered(input, options)?
  }))
}

/// Cures a string with decancer's default options.
///
/// Output will always be in lowercase and [bidirectionally reordered](https://en.wikipedia.org/wiki/Bidirectional_text) in order to treat right-to-left characters. Therefore, the string output is laid out in memory the same way as it were to be displayed graphically, but **may break if displayed graphically** since some right-to-left characters are reversed.
///
/// If you plan on only using this macro, it's recommended to disable the default `options` feature flag to optimize away unnecessary option checks.
///
/// This macro expands to:
///
/// ```rust,ignore
/// decancer::cure(string, decancer::Options::default());
/// ```
///
/// For more information, see [the `cure` function][cure()].
///
/// # Errors
///
/// Errors if the string is malformed to the point where it's not possible to apply unicode's [bidirectional algorithm](https://en.wikipedia.org/wiki/Bidirectional_text) to it.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured = decancer::cure!("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣").unwrap();
///
/// assert_eq!(cured, "very funny text");
/// assert!(cured.contains("FuNny"));
/// ```
#[macro_export]
macro_rules! cure {
  ($string:expr) => {
    $crate::cure($string, $crate::Options::default())
  };
}

/// Formats a string by only removing [pure homoglyphs][Options::pure_homoglyph] while still [retaining capitalization][Options::retain_capitalization].
///
/// This macro immediately returns a [`String`] type.
///
/// For more information, see [`Options::formatter`].
#[macro_export]
macro_rules! format {
  ($string:expr) => {
    String::from($crate::cure($string, $crate::Options::formatter()).unwrap())
  };
}
