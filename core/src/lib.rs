// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

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
#[cfg(feature = "suggestions")]
pub use string::CureSuggestion;
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

    /// Got a malformed isolating run sequence structure.
    MalformedIsolatingRunSequence,

    /// Got a malformed bidi level override status stack.
    MalformedOverrideStatusStack,
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
  let ascii_only = options.is(25);

  #[cfg(feature = "options")]
  let alphanumeric_only = options.is(26);

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
      CASE_SENSITIVE_CODEPOINTS_OFFSET.into(),
      CASE_SENSITIVE_CODEPOINTS_COUNT.into(),
    ) {
      #[cfg(feature = "options")]
      if retain_capitalization {
        translation.make_uppercase();
      }

      #[cfg(feature = "options")]
      return translation.ensure_stripped_if(ascii_only, alphanumeric_only);

      #[cfg(not(feature = "options"))]
      return translation;
    }
  }

  #[cfg(feature = "options")]
  return options
    .translate(code_lowercased, 6, CODEPOINTS_COUNT.into())
    .map_or_else(
      || {
        if ascii_only || alphanumeric_only {
          Translation::None
        } else {
          Translation::character(default_output)
        }
      },
      |mut translation| {
        if retain_capitalization {
          translation.make_uppercase();
        }

        translation.ensure_stripped_if(ascii_only, alphanumeric_only)
      },
    );

  #[cfg(not(feature = "options"))]
  options
    .translate(code_lowercased, 6, CODEPOINTS_COUNT.into())
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
/// ```rust
/// # let code = 0xFF25u32;
/// #
/// # let cured =
/// decancer::cure_char(code, decancer::Options::default());
/// #
/// # assert_eq!(cured, decancer::cure_char!(code));
/// ```
///
/// For more information, see [the `cure_char` function][cure_char()].
#[macro_export]
macro_rules! cure_char {
  ($code:expr) => {
    $crate::cure_char($code, $crate::Options::default())
  };
}

pub(crate) struct Input {
  pub(crate) code: u32,
  pub(crate) class: Class,
  #[cfg(feature = "suggestions")]
  pub(crate) index: usize,
}

struct FirstPassOutput {
  inputs: Vec<Input>,
  paragraphs: Vec<Paragraph>,
  #[cfg(feature = "suggestions")]
  suggestions: Vec<CureSuggestion>,
}

#[allow(clippy::too_many_lines)]
fn first_cure_pass(input: &str) -> FirstPassOutput {
  let mut refined_inputs: Vec<Input> = Vec::with_capacity(input.len());
  let mut isolate_stack = vec![];

  let mut paragraphs = vec![];
  let mut paragraph_start = 0;
  let mut paragraph_level = None;
  let mut pure_ltr = true;
  let mut has_isolate_controls = false;

  let mut idx = 0;
  #[cfg(feature = "suggestions")]
  let mut suggestions = vec![];

  #[cfg(feature = "suggestions")]
  let char_iterator = input.char_indices();
  #[cfg(not(feature = "suggestions"))]
  let char_iterator = input.chars();

  for codepoint in char_iterator {
    #[cfg(feature = "suggestions")]
    let (old_index, codepoint) = codepoint;
    let mut codepoint = codepoint as u32;

    if !is_none(codepoint)
      && let Some(class) = Class::new(codepoint)
    {
      if class == Class::WS && codepoint > 0x7f {
        codepoint = 0x20;
      }

      refined_inputs.push(Input {
        code: codepoint,
        class,
        #[cfg(feature = "suggestions")]
        index: old_index,
      });

      match class {
        Class::B => {
          let paragraph_end = idx + 1;

          paragraphs.push(Paragraph {
            range: paragraph_start..paragraph_end,
            level: paragraph_level.unwrap_or(Level::LTR),
            pure_ltr,
            has_isolate_controls,
          });

          paragraph_start = paragraph_end;
          pure_ltr = true;
          has_isolate_controls = false;
          isolate_stack.clear();
          paragraph_level = None;
        },

        Class::L | Class::R | Class::AL => {
          if class != Class::L {
            pure_ltr = false;
          }

          match isolate_stack.last() {
            Some(&start_idx) => {
              let input: &mut Input = &mut refined_inputs[start_idx];

              if input.class == Class::FSI {
                #[cfg(not(tarpaulin_include))]
                let new_class = if class == Class::L {
                  Class::LRI
                } else {
                  Class::RLI
                };

                input.class = new_class;
              }
            },

            None => {
              if paragraph_level.is_none() {
                paragraph_level.replace(if class == Class::L {
                  Level::LTR
                } else {
                  Level::RTL
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
          has_isolate_controls = true;
          isolate_stack.push(idx);
        },

        Class::PDI => {
          isolate_stack.pop();
        },

        _ => {},
      }

      idx += 1;
    } else {
      #[cfg(feature = "suggestions")]
      suggestions.push(CureSuggestion {
        old_index,
        new_index: None,
        translation: Translation::None,
      });
    }
  }

  if paragraph_start < idx {
    paragraphs.push(Paragraph {
      range: paragraph_start..idx,
      level: paragraph_level.unwrap_or(Level::LTR),
      pure_ltr,
      has_isolate_controls,
    });
  }

  FirstPassOutput {
    inputs: refined_inputs,
    paragraphs,
    #[cfg(feature = "suggestions")]
    suggestions,
  }
}

/// Cures a string with the specified [`Options`].
///
/// To use this function with decancer's default options, use [the `cure` macro][cure!] instead.
///
/// # Errors
///
/// Errors if the string is malformed to the point where it's not possible to apply unicode's [bidirectional algorithm](https://en.wikipedia.org/wiki/Bidirectional_text) to it. This error is possible if [`Options::disable_bidi`] is disabled.
#[allow(clippy::too_many_lines)]
pub fn cure(input: &str, options: Options) -> Result<CuredString, Error> {
  #[cfg(feature = "options")]
  if options.is(1) {
    #[cfg(feature = "suggestions")]
    let mut suggestions = vec![];
    #[cfg(feature = "suggestions")]
    let char_iterator = input.char_indices();
    #[cfg(not(feature = "suggestions"))]
    let char_iterator = input.chars();

    let mut output = String::with_capacity(input.len());

    for codepoint in char_iterator {
      #[cfg(feature = "suggestions")]
      let (old_index, codepoint) = codepoint;
      let codepoint = codepoint as u32;

      if is_special_rtl(codepoint) {
        #[cfg(feature = "suggestions")]
        suggestions.push(CureSuggestion {
          old_index,
          new_index: None,
          translation: Translation::None,
        });
      } else {
        let translation = cure_char(codepoint, options);

        #[cfg(feature = "suggestions")]
        suggestions.push(CureSuggestion {
          old_index,
          new_index: Some(output.len()),
          translation: translation.clone(),
        });

        output += translation;
      }
    }

    return Ok(CuredString::new(
      output.into(),
      #[cfg(feature = "suggestions")]
      suggestions,
      #[cfg(feature = "leetspeak")]
      options,
    ));
  }

  #[cfg(feature = "suggestions")]
  let mut first_pass_output = first_cure_pass(input);
  #[cfg(not(feature = "suggestions"))]
  let first_pass_output = first_cure_pass(input);

  let mut levels = Vec::with_capacity(first_pass_output.inputs.len());
  let mut level_runs = vec![];
  let mut processing_classes = first_pass_output
    .inputs
    .iter()
    .map(|entry| entry.class)
    .collect::<Vec<_>>();
  let mut output = String::with_capacity(first_pass_output.inputs.len());
  let mut sequences = vec![];

  let mut feed = |entry: &Input| {
    let translation = cure_char_inner(entry.code, options);

    #[cfg(feature = "suggestions")]
    first_pass_output.suggestions.push(CureSuggestion {
      old_index: entry.index,
      new_index: Some(output.len()),
      translation: translation.clone(),
    });

    output += translation;
  };

  for paragraph in &first_pass_output.paragraphs {
    levels.resize(levels.len() + paragraph.range.len(), paragraph.level);

    if paragraph.level.0 != 0 || !paragraph.pure_ltr {
      let inputs = &first_pass_output.inputs[paragraph.range.clone()];
      let processing_classes = &mut processing_classes[paragraph.range.clone()];
      let levels = &mut levels[paragraph.range.clone()];
      level_runs.clear();

      paragraph.compute_explicit(inputs, processing_classes, levels, &mut level_runs)?;

      sequences.clear();
      paragraph.isolating_run_sequences(levels, &level_runs, inputs, &mut sequences)?;

      for sequence in &sequences {
        sequence.resolve_implicit_weak(processing_classes);
        sequence.resolve_implicit_neutral(inputs, processing_classes, levels);
      }

      for j in 0..levels.len() {
        {
          let level = &mut levels[j];

          match (level.is_rtl(), processing_classes[j]) {
            (false, Class::AN | Class::EN) => level.raise(2)?,

            (false, Class::R) | (true, Class::L | Class::EN | Class::AN) => {
              level.raise(1)?;
            },

            _ => {},
          }
        }

        if inputs[j].class.removed_by_x9() {
          levels[j] = if j > 0 {
            levels[j - 1]
          } else {
            paragraph.level
          };
        }
      }
    }
  }

  for paragraph in first_pass_output.paragraphs {
    let (revised_levels, runs) = paragraph.visual_runs(&first_pass_output.inputs, &levels)?;

    for run in runs {
      let input_slice = &first_pass_output.inputs[run.clone()];

      if revised_levels[run.start].is_rtl() {
        for entry in input_slice.iter().rev() {
          feed(entry);
        }
      } else {
        for entry in input_slice {
          feed(entry);
        }
      }
    }
  }

  Ok(CuredString::new(
    output.into(),
    #[cfg(feature = "suggestions")]
    first_pass_output.suggestions,
    #[cfg(feature = "leetspeak")]
    options,
  ))
}

/// Cures a string with decancer's default options.
///
/// Output will always be in lowercase and [bidirectionally reordered](https://en.wikipedia.org/wiki/Bidirectional_text) in order to treat right-to-left characters. Therefore, the string output is laid out in memory the same way as it were to be displayed graphically, but **may break if displayed graphically** since some right-to-left characters are reversed.
///
/// If you plan on only using this macro, it's recommended to disable the default `options` feature flag to optimize away unnecessary option checks.
///
/// This macro expands to:
///
/// ```rust
/// # let string = "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
/// #
/// # let cured =
/// decancer::cure(string, decancer::Options::default());
/// #
/// # assert_eq!(cured.unwrap(), decancer::cure!(string).unwrap());
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
