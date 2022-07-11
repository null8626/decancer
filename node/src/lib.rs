#![allow(dead_code)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate napi;

extern crate decancer;
use decancer::Decancer;

const DECANCER: Decancer = Decancer::new();

#[napi]
fn contains(a: String, b: String) -> bool {
  DECANCER.contains(a, b)
}

#[napi]
fn decancer(input: String) -> String {
  DECANCER.cure(input)
}
