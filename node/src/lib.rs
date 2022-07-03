#![allow(dead_code)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate napi;

extern crate decancer_core;

use napi::{bindgen_prelude::Error, JsString};

#[napi]
fn contains(input: JsString, other: JsString) -> Result<bool, Error> {
  let a = input.into_utf16()?;
  let b = other.into_utf16()?;
  let a_slice = a.as_slice();
  let b_slice = b.as_slice();

  Ok(decancer_core::contains(
    &a_slice[0..a_slice.len() - 1],
    &b_slice[0..b_slice.len() - 1],
  ))
}

#[napi]
fn decancer(raw_input: JsString) -> Result<String, Error> {
  let input = raw_input.into_utf16()?;
  let input_slice = input.as_slice();

  Ok(decancer_core::decancer(
    &input_slice[0..input_slice.len() - 1],
  ))
}
