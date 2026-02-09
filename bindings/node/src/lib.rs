// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

#![allow(dead_code, clippy::inherent_to_string)]

use std::ops::Range;

use napi::{Either, Result, Status, bindgen_prelude::Error};

#[macro_use]
extern crate napi_derive;

macro_rules! options {
  (
    keys {
      $($key_idx:literal: $key_name:ident),*
    }

    overrides {
      $($override_name:ident: $override_value:expr),*
    }
  ) => {
    #[napi(object)]
    #[derive(Default)]
    pub struct Options {
      $(pub $key_name: Option<bool>,)*
      $(pub $override_name: Option<bool>,)*
    }

    impl From<Options> for u32 {
      fn from(value: Options) -> u32 {
        let mut options = 0;

        $(if value.$key_name.unwrap_or_default() {
          options |= (1 << $key_idx);
        })*

        $(if value.$override_name.unwrap_or_default() {
          options = $override_value;
        })*

        options
      }
    }
  };
}

options! {
  keys {
    0: retain_capitalization,
    1: disable_bidi,
    2: disable_leetspeak,
    3: retain_diacritics,
    4: retain_greek,
    5: retain_cyrillic,
    6: retain_hebrew,
    7: retain_arabic,
    8: retain_devanagari,
    9: retain_bengali,
    10: retain_armenian,
    11: retain_gujarati,
    12: retain_tamil,
    13: retain_thai,
    14: retain_lao,
    15: retain_burmese,
    16: retain_khmer,
    17: retain_mongolian,
    18: retain_chinese,
    19: retain_japanese,
    20: retain_korean,
    21: retain_braille,
    22: retain_emojis,
    23: retain_turkish,
    24: ascii_only,
    25: alphanumeric_only
  }

  overrides {
    all: 0x3ff_ffff,
    pure_homoglyph: 0x7f_fff8
  }
}

#[napi]
pub struct Match {
  range: Range<usize>,
  portion: String,
}

#[napi]
impl Match {
  #[napi(getter)]
  pub const fn start(&self) -> i64 {
    self.range.start.cast_signed() as _
  }

  #[napi(getter)]
  pub const fn end(&self) -> i64 {
    self.range.end.cast_signed() as _
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.portion.clone()
  }
}

#[napi]
pub struct CuredString(decancer::CuredString);

#[napi]
impl CuredString {
  fn new_match(&self, mat: Range<usize>) -> Match {
    Match {
      range: mat.clone(),
      portion: String::from(&self.0[mat]),
    }
  }

  #[napi]
  pub const fn disable_leetspeak(&mut self, switch: bool) {
    self.0.disable_leetspeak(switch);
  }

  #[napi]
  pub fn find(&self, other: String) -> Vec<Match> {
    self.0.find(&other).map(|mat| self.new_match(mat)).collect()
  }

  #[napi]
  pub fn find_multiple(&self, other: Vec<String>) -> Vec<Match> {
    self
      .0
      .find_multiple(other)
      .into_iter()
      .map(|mat| self.new_match(mat))
      .collect()
  }

  #[napi]
  pub fn censor(&mut self, other: String, with: String) -> Result<()> {
    match with.chars().next() {
      Some(with_char) => {
        self.0.censor(&other, with_char);

        Ok(())
      },

      None => Err(Error::new(
        Status::InvalidArg,
        "Replacement string is empty.",
      )),
    }
  }

  #[napi]
  pub fn censor_multiple(&mut self, other: Vec<String>, with: String) -> Result<()> {
    match with.chars().next() {
      Some(with_char) => {
        self.0.censor_multiple(&other, with_char);

        Ok(())
      },

      None => Err(Error::new(
        Status::InvalidArg,
        "Replacement string is empty.",
      )),
    }
  }

  #[napi]
  pub fn replace(&mut self, other: String, with: String) {
    self.0.replace(&other, &with);
  }

  #[napi]
  pub fn replace_multiple(&mut self, other: Vec<String>, with: String) {
    self.0.replace_multiple(&other, &with);
  }

  #[napi]
  pub fn starts_with(&self, other: String) -> bool {
    self.0.starts_with(&other)
  }

  #[napi]
  pub fn ends_with(&self, other: String) -> bool {
    self.0.ends_with(&other)
  }

  #[napi]
  pub fn contains(&self, other: String) -> bool {
    self.0.contains(&other)
  }

  #[napi]
  pub fn equals(&self, other: String) -> bool {
    self.0 == other
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

#[napi]
fn options(options: Option<Options>) -> u32 {
  options.unwrap_or_default().into()
}

#[napi]
fn cure(input: String, maybe_options: Option<Either<u32, Options>>) -> Result<CuredString> {
  let options = match maybe_options {
    Some(Either::A(number)) => number,

    Some(Either::B(opt)) => opt.into(),

    None => 0,
  };

  match decancer::cure(&input, options.into()) {
    Ok(output) => Ok(CuredString(output)),

    Err(err) => Err(Error::new(Status::InvalidArg, err)),
  }
}
