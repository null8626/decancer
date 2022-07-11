#![allow(dead_code)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate napi;

extern crate decancer;

#[napi]
fn contains(a: String, b: String) -> bool {
  decancer::contains(a, b)
}

#[napi]
fn decancer(input: String) -> String {
  decancer::cure(input)
}