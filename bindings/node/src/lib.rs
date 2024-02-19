#![allow(clippy::inherent_to_string)]

#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{Error, FromNapiValue},
  Env, JsString, JsUnknown, Result, Status, ValueType,
};
use std::mem::transmute;

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
      $(
        pub $key_name: Option<bool>,
      )*
      $(
        pub $override_name: Option<bool>,
      )*
    }

    impl Into<u32> for Options {
      fn into(self) -> u32 {
        let mut options = 0;

        $(
          if self.$key_name.unwrap_or_default() {
            options |= (1 << $key_idx);
          }
        )*

        $(
          if self.$override_name.unwrap_or_default() {
            options = $override_value;
          }
        )*

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
  }

  overrides {
    pure_homoglyph: ((1 << 21) - 1) ^ 0b11,
  }
}

#[napi]
pub struct CuredString(decancer::CuredString);

#[napi]
impl CuredString {
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

  match decancer::cure(&input, unsafe { transmute(options) }) {
    Ok(output) => Ok(CuredString(output)),
    Err(err) => Err(Error::new(Status::InvalidArg, err)),
  }
}

#[napi]
fn format(input: String) -> String {
  decancer::format!(&input)
}
