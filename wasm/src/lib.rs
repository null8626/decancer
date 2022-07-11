extern crate decancer;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn decancer(raw_input: &str) -> String {
  decancer::cure(raw_input)
}

#[wasm_bindgen]
pub fn contains(input: &str, other: &str) -> bool {
  decancer::contains(input, other)
}
