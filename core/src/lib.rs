#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "std")]
mod bidi;
mod codepoints;
mod similar;
#[cfg(feature = "std")]
mod string;
#[cfg(test)]
mod test;
mod translation;
mod util;

#[cfg(feature = "std")]
use bidi::{Class, Level, Paragraph};
#[cfg(feature = "std")]
pub use string::CuredString;
pub use translation::Translation;

use codepoints::{
  Codepoint, CASE_SENSITIVE_CODEPOINTS_COUNT, CASE_SENSITIVE_CODEPOINTS_OFFSET, CODEPOINTS_COUNT,
};
use core::cmp::Ordering;

const fn translate(code: u32, offset: i32, mut end: i32) -> Option<Translation> {
  let mut start = 0;

  while start <= end {
    let mid = (start + end) / 2;
    let codepoint = Codepoint::at(offset + (mid * 5));

    match codepoint.matches(code) {
      Ordering::Equal => return Some(codepoint.translation(code)),
      Ordering::Greater => start = mid + 1,
      Ordering::Less => end = mid - 1,
    };
  }

  None
}

const fn is_none(code: u32) -> bool {
  matches!(code, 0..=9 | 14..=31 | 127 | 0xd800..=0xf8ff | 0xe0100..)
}

fn cure_char_inner(code: u32) -> Translation {
  // SAFETY: even if there is no lowercase mapping for some codepoints, it would just return itself.
  // therefore, the first iteration and/or codepoint always exists.
  let code_lowercased = unsafe {
    char::from_u32_unchecked(code)
      .to_lowercase()
      .next()
      .unwrap_unchecked() as _
  };

  if code_lowercased < 0x80 {
    return Translation::character(code_lowercased);
  } else if code != code_lowercased {
    if let Some(translation) = translate(
      code,
      CASE_SENSITIVE_CODEPOINTS_OFFSET as _,
      CASE_SENSITIVE_CODEPOINTS_COUNT as _,
    ) {
      return translation;
    }
  }

  translate(code_lowercased, 6, CODEPOINTS_COUNT as _)
    .unwrap_or(Translation::character(code_lowercased))
}

/// Cures a single character/unicode codepoint. Output will always be in lowercase and equality methods provided by [`Translation`] is case-insensitive.
///
/// # Examples
///
/// Most of the time, this would yield only a single unicode character:
///
/// ```rust
/// use decancer::Translation;
///
/// assert_eq!(decancer::cure_char('Ｅ'), Translation::Character('e'));
/// ```
///
/// However, for several special cases, it would yield an [ASCII](https://en.wikipedia.org/wiki/ASCII) [`&'static str`][prim@str]:
///
/// ```rust
/// use decancer::Translation;
///
/// assert_eq!(decancer::cure_char('æ'), Translation::String("ae"));
/// assert_eq!(decancer::cure_char('ĳ'), Translation::String("ij"));
/// assert_eq!(decancer::cure_char('œ'), Translation::String("oe"));
/// assert_eq!(decancer::cure_char('🆐'), Translation::String("dj"));
/// assert_eq!(decancer::cure_char('🆑'), Translation::String("cl"));
/// assert_eq!(decancer::cure_char('🆔'), Translation::String("id"));
/// assert_eq!(decancer::cure_char('🆖'), Translation::String("ng"));
/// assert_eq!(decancer::cure_char('🆗'), Translation::String("ok"));
/// assert_eq!(decancer::cure_char('🆚'), Translation::String("vs"));
/// assert_eq!(decancer::cure_char('🜀'), Translation::String("qe"));
/// assert_eq!(decancer::cure_char('🜇'), Translation::String("ar"));
///
/// assert_eq!(decancer::cure_char('⅓'), Translation::String("1/3"));
/// assert_eq!(decancer::cure_char('⅔'), Translation::String("2/3"));
/// assert_eq!(decancer::cure_char('⅕'), Translation::String("1/5"));
/// assert_eq!(decancer::cure_char('⅖'), Translation::String("2/5"));
/// assert_eq!(decancer::cure_char('⅗'), Translation::String("3/5"));
/// assert_eq!(decancer::cure_char('⅘'), Translation::String("4/5"));
/// assert_eq!(decancer::cure_char('㋍'), Translation::String("erg"));
/// assert_eq!(decancer::cure_char('㋏'), Translation::String("ltd"));
///
/// assert_eq!(decancer::cure_char('㍴'), Translation::String("bar"));
/// assert_eq!(decancer::cure_char('㎈'), Translation::String("cal"));
/// assert_eq!(decancer::cure_char('㎭'), Translation::String("rad"));
/// assert_eq!(decancer::cure_char('㏇'), Translation::String("co."));
/// assert_eq!(decancer::cure_char('㏒'), Translation::String("log"));
/// assert_eq!(decancer::cure_char('㏕'), Translation::String("mil"));
/// assert_eq!(decancer::cure_char('㏖'), Translation::String("mol"));
/// assert_eq!(decancer::cure_char('㏙'), Translation::String("ppm"));
/// ```
///
/// If your unicode character is a [control character](https://en.wikipedia.org/wiki/Control_character), [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates), [combining character](https://en.wikipedia.org/wiki/Script_(Unicode)#Special_script_property_values) (e.g diacritics), [private use character](https://en.wikipedia.org/wiki/Private_Use_Areas), [byte order character](https://en.wikipedia.org/wiki/Byte_order_mark), or any invalid unicode value (e.g beyond [`char::MAX`]), you would get [`None`][Translation::None]:
///
/// ```rust
/// use decancer::Translation;
///
/// assert_eq!(decancer::cure_char(0xD800u32), Translation::None);
/// assert_eq!(decancer::cure_char(char::REPLACEMENT_CHARACTER), Translation::None);
/// assert_eq!(decancer::cure_char((char::MAX as u32) + 1), Translation::None);
/// ```
pub fn cure_char<C: Into<u32>>(code: C) -> Translation {
  let code = code.into();

  if is_none(code) {
    Translation::None
  } else {
    cure_char_inner(code)
  }
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
        if class == Class::WS {
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
          }

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
              }

              None => {
                if paragraph_level.is_none() {
                  paragraph_level.replace(if class == Class::L {
                    Level::ltr()
                  } else {
                    Level::rtl()
                  });
                }
              }
            }
          }

          Class::AN | Class::LRE | Class::RLE | Class::LRO | Class::RLO => {
            pure_ltr = false;
          }

          Class::RLI | Class::LRI | Class::FSI => {
            pure_ltr = false;
            isolate_stack.push(idx);
          }

          Class::PDI => {
            isolate_stack.pop();
          }

          _ => {}
        }

        // SAFETY: the only modification to this codepoint is in the if-statement above.
        refined_input.push(unsafe { char::from_u32_unchecked(codepoint) });

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

const fn bounded_sub(a: usize, b: usize) -> usize {
  if b >= a {
    0
  } else {
    a - b
  }
}

/// Cures a string. Output will always be in lowercase and all overridden comparison methods provided by [`CuredString`] is case-insensitive.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// assert_eq!(cured, "very funny text");
/// assert!(cured.contains("FuNny"));
/// assert_eq!(cured.into_str(), String::from("very funny text"));
///
/// assert_eq!(decancer::cure("v̵̨̟̩͕̭̼͍̜͊̎̽̅͊̍́̏̓̕ͅe̴̡͙̳̭͚͕͕̞̦̱͊͗̈̓̑̈́̀͘ͅr̵̡̢̫̞͕͎̱͇̠͕͎̺̱̭̪̈͜ͅy̴̧̯͈̥͔̣̫̮̦̪͎̮͑̄̏̂̽͘̚͘̚͜͜͠ ̸̨̛̬͈̲̗͕̜͚̟̈̔́̾͝f̷̪̺͓̽̃̽̀̀̓̽́̾͗̋̇̀̀͐u̴͕̜̗͛̈͆̐n̸̡͙̣̙̳̥͕̥̼̪̻̪̋̀̀̽̈́͜n̷̨̗͖̗̹̜͈̗̲͔͕͉̗̻͓̟̓̽̾͗͑̾̈͜ỹ̶̧̧̩̜̹̩̩̠̦͉̮̳̦̀͛͗̒͑̅̿͌͋͠ ̴̛̠͕̥͇͉̙̯͙̠͇̝̍̃̓̆̈́̐͊̈́͘͝͠t̴̨̰̜̟͓̬͊̂̽̃͌́͂̓̊̅̃̕̚ͅȩ̵̛̬͈͔̮͙͇̫̄̽͒̊́́̀͒̚x̸̖͖̜͍̣̹̺̟̬̞̝͇̐̇̽̒͋̒̑̃̒̄̐͘͝t̸̥̅̓̉̽͑̔̑̿̇"), "very funny text");
/// assert_eq!(decancer::cure("foo ㍴ ㎈ls console.㏒"), "foo bar calls console.log");
/// assert_eq!(decancer::cure("you 🆚 w3ird un1c0de ch4rs"), "you vs weird unicode chars");
/// ```
#[cfg(feature = "std")]
pub fn cure(input: &str) -> Option<CuredString> {
  let (refined_input, original_classes, paragraphs) = first_cure_pass(input);

  let mut levels = Vec::with_capacity(refined_input.len());
  let mut processing_classes = original_classes.clone();
  let mut output = String::with_capacity(refined_input.len() + bounded_sub(paragraphs.len(), 1));
  let end = bounded_sub(paragraphs.len(), 1);

  for (idx, paragraph) in paragraphs.iter().enumerate() {
    levels.resize(levels.len() + paragraph.range.len(), paragraph.level);

    if paragraph.level.level() != 0 || !paragraph.pure_ltr {
      paragraph.compute_explicit(
        &refined_input,
        &original_classes,
        &mut processing_classes,
        &mut levels,
      );

      for sequence in paragraph.isolating_run_sequences(&levels, &original_classes) {
        sequence.resolve_implicit_weak(&refined_input, &mut processing_classes);
        sequence.resolve_implicit_neutral(&refined_input, &mut processing_classes, &levels);
      }

      for j in 0..levels.len() {
        match (levels[j].is_rtl(), original_classes[j]) {
          (false, Class::AN) | (false, Class::EN) => levels[j].raise(2)?,
          (false, Class::R) | (true, Class::L) | (true, Class::EN) | (true, Class::AN) => {
            levels[j].raise(1)?
          }
          (_, _) => {}
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

    let (revised_levels, runs) =
      paragraph.visual_runs(&refined_input, &original_classes, &levels)?;

    for run in runs {
      let text = &refined_input[run.clone()];

      if revised_levels[run.start].is_rtl() {
        for c in text.chars().rev() {
          cure_char_inner(c as _).add_to(&mut output);
        }
      } else {
        for c in text.chars() {
          cure_char_inner(c as _).add_to(&mut output);
        }
      }

      if idx != end {
        output.push('\n');
      }
    }
  }

  Some(CuredString(output))
}
