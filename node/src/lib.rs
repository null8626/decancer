#![allow(dead_code)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

extern crate napi;
extern crate decancer;

use decancer::Decancer;
use napi::{Result, JsString};

const DECANCER: Decancer = Decancer::new();

#[napi]
fn contains(a: JsString, b: JsString) -> Result<bool> {
  let a_utf16 = a.into_utf16()?;
  let b_utf16 = b.into_utf16()?;

  Ok(DECANCER.contains(a_utf16.as_slice(), b_utf16.as_slice()))
}

#[napi]
fn decancer(input: JsString) -> Result<String> {
  let res = input.into_utf16()?;

  Ok(DECANCER.cure(res.as_slice()))
}
