#![allow(dead_code)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

extern crate decancer;
extern crate napi;

#[napi]
pub struct CuredString(decancer::CuredString);

#[napi]
impl CuredString {
  #[napi(js_name = "startsWith")]
  pub fn starts_with(&self, other: String) -> bool {
    self.0.starts_with(&other)
  }

  #[napi(js_name = "endsWith")]
  pub fn ends_with(&self, other: String) -> bool {
    self.0.ends_with(&other)
  }

  #[napi]
  pub fn contains(&self, other: String) -> bool {
    self.0.contains(&other)
  }

  #[napi]
  pub fn equals(&self, other: String) -> bool {
    self.0 == &other
  }

  #[napi(js_name = "toString")]
  pub fn to_string(&self) -> String {
    self.0.clone().into_str()
  }
}

#[napi]
fn decancer(input: String) -> CuredString {
  CuredString(decancer::cure(&input))
}
