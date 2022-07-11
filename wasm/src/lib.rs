extern crate decancer;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use decancer::Decancer;

const DECANCER: Decancer = Decancer::new();

#[wasm_bindgen]
pub fn decancer(raw_input: &str) -> String {
  DECANCER.cure(raw_input)
}

#[wasm_bindgen]
pub fn contains(input: &str, other: &str) -> bool {
  DECANCER.contains(input, other)
}
