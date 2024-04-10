#![allow(non_snake_case)]
#![forbid(unsafe_code)]

use std::{convert::AsRef, ops::Range};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Match {
  pub start: usize,
  pub end: usize,
  portion: String,
}

#[wasm_bindgen]
impl Match {
  pub fn toString(&self) -> String {
    self.portion.clone()
  }
}

#[wasm_bindgen]
pub struct CuredString(decancer::CuredString);

#[wasm_bindgen]
impl CuredString {
  #[inline(always)]
  fn new_match(&self, mat: Range<usize>) -> Match {
    Match {
      start: mat.start,
      end: mat.end,
      portion: String::from(&self.0[mat]),
    }
  }

  pub fn find(&self, other: &str) -> Vec<Match> {
    self.0.find(other).map(|mat| self.new_match(mat)).collect()
  }

  pub fn findMultiple(&self, other: Vec<String>) -> Vec<Match> {
    self
      .0
      .find_multiple(other)
      .into_iter()
      .map(|mat| self.new_match(mat))
      .collect()
  }

  pub fn censor(&mut self, other: &str, with: char) {
    self.0.censor(other, with)
  }

  pub fn censorMultiple(&mut self, other: Vec<String>, with: char) {
    self.0.censor_multiple(other, with)
  }

  pub fn replace(&mut self, other: &str, with: &str) {
    self.0.replace(other, with)
  }

  pub fn replaceMultiple(&mut self, other: Vec<String>, with: &str) {
    self.0.replace_multiple(other, with)
  }

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
    self.0.clone().into()
  }
}

#[wasm_bindgen]
pub fn cure(input: &str, options: u32) -> Result<CuredString, JsError> {
  match decancer::cure(input, options.into()) {
    Ok(output) => Ok(CuredString(output)),
    Err(err) => Err(JsError::new(<decancer::Error as AsRef<str>>::as_ref(&err))),
  }
}

#[wasm_bindgen]
pub fn format(input: &str) -> String {
  decancer::format!(input)
}
