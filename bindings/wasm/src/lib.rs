#![allow(non_snake_case)]
// I'M SORRY FOR MY SINS BUT THIS HAS TO BE DONE

extern crate decancer;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CuredString(decancer::CuredString);

#[wasm_bindgen]
impl CuredString {
  pub fn startsWith(&self, other: &str) -> bool {
    self.0.starts_with(other)
  }

  pub fn endsWith(&self, other: &str) -> bool {
    self.0.ends_with(other)
  }

  pub fn contains(&self, other: &str) -> bool {
    self.0.contains(other)
  }

  pub fn equals(&self, other: &str) -> bool {
    self.0 == other
  }

  pub fn toString(&self) -> String {
    self.0.clone().into_str()
  }
}

#[wasm_bindgen]
pub fn decancer(input: &str) -> CuredString {
  CuredString(decancer::cure(input))
}
