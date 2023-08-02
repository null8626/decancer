#![allow(clippy::inherent_to_string)]

#[macro_use]
extern crate napi_derive;

use napi::{Env, JsString, Result};

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
fn decancer(input: String) -> CuredString {
  CuredString(decancer::cure(&input))
}
