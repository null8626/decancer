#![allow(non_snake_case)]

use std::{convert::AsRef, mem::transmute};
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
pub fn cure(input: &str, options: u32) -> Result<CuredString, JsError> {
  match decancer::cure(input, unsafe { transmute(options) }) {
    Ok(output) => Ok(CuredString(output)),
    Err(err) => Err(JsError::new(<decancer::Error as AsRef<str>>::as_ref(&err))),
  }
}

#[wasm_bindgen]
pub fn format(input: &str) -> String {
  decancer::format!(input)
}
