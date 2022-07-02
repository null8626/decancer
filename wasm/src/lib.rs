extern crate wasm_bindgen;
extern crate decancer_core;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decancer(raw_input: &str) -> String {
  let input = raw_input.encode_utf16().collect::<Vec<_>>();

  decancer_core::decancer(&input)
}

#[wasm_bindgen]
pub fn contains(input: &str, other: &str) -> bool {
  let a = input.encode_utf16().collect::<Vec<_>>();
  let b = other.encode_utf16().collect::<Vec<_>>();

  decancer_core::contains(&a, &b)
}