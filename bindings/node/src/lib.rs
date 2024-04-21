#![forbid(unsafe_code)]

#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{Error, FromNapiValue},
  Env, JsNumber, JsString, JsUnknown, Result, Status, ValueType,
};
use std::ops::Range;

macro_rules! options {
  (
    keys {
      $($key_idx:literal: $key_name:ident,)*
    }

    overrides {
      $($override_name:ident: $override_value:expr,)*
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
    2: retain_diacritics,
    3: retain_greek,
    4: retain_cyrillic,
    5: retain_hebrew,
    6: retain_arabic,
    7: retain_devanagari,
    8: retain_bengali,
    9: retain_armenian,
    10: retain_gujarati,
    11: retain_tamil,
    12: retain_thai,
    13: retain_lao,
    14: retain_burmese,
    15: retain_khmer,
    16: retain_mongolian,
    17: retain_chinese,
    18: retain_japanese,
    19: retain_korean,
    20: retain_braille,
    21: retain_emojis,
    22: ascii_only,
    23: alphanumeric_only,
  }

  overrides {
    pure_homoglyph: ((1 << 24) - 1) ^ 0xe00003,
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
  pub fn start(&self, env: Env) -> Result<JsNumber> {
    env.create_int64(self.range.start as _)
  }

  #[napi(getter)]
  pub fn end(&self, env: Env) -> Result<JsNumber> {
    env.create_int64(self.range.end as _)
  }

  #[napi]
  pub fn to_string(&self, env: Env) -> Result<JsString> {
    env.create_string(&self.portion)
  }
}

#[napi]
pub struct CuredString(decancer::CuredString);

#[napi]
impl CuredString {
  #[inline(always)]
  fn new_match(&self, mat: Range<usize>) -> Match {
    Match {
      range: mat.clone(),
      portion: String::from(&self.0[mat]),
    }
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
  pub fn to_string(&self, env: Env) -> Result<JsString> {
    env.create_string(&self.0)
  }
}

#[napi]
fn options(options: Option<Options>) -> u32 {
  options.unwrap_or_default().into()
}

#[napi]
fn cure(input: String, maybe_options: JsUnknown) -> Result<CuredString> {
  let options = if maybe_options.get_type()? == ValueType::Number {
    maybe_options
      .coerce_to_number()
      .and_then(|idx| idx.get_uint32())
  } else {
    <Option<Options> as FromNapiValue>::from_unknown(maybe_options).map(options)
  }?;

  match decancer::cure(&input, options.into()) {
    Ok(output) => Ok(CuredString(output)),
    Err(err) => Err(Error::new(Status::InvalidArg, err)),
  }
}

#[napi]
fn format(input: String) -> String {
  decancer::format!(&input)
}