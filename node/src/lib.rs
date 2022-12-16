#![allow(dead_code)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

extern crate decancer;
extern crate napi;

use napi::{JsString, Result};

/*#[napi]
fn contains(a: JsString, b: JsString) -> Result<bool> {
  let a_utf16 = a.into_utf16()?;
  let b_utf16 = b.into_utf16()?;

  Ok(DECANCER.contains(a_utf16.as_slice(), b_utf16.as_slice()))
}*/

#[napi]
fn decancer(input: String) -> Result<String {
  decancer::cure(&input).into_str()
}
