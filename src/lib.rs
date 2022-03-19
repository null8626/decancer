#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::Error;
use napi::JsString;

pub(crate) mod parser;
use parser::Parser;

mod alphabet;
mod emojis;
mod misc;

fn parse_case_sensitive(input: &[u16]) -> Vec<u16> {
  let mut parser = Parser::new(&input[0..input.len() - 1]);
  
  loop {
    if !emojis::parse(&mut parser) && !misc::parse(&mut parser) {
      if parser.end() {
        break;
      }
      
      parser.push_byte(parser.get());
      parser.advance(1);
    }
  }
  
  parser
    .output()
    .to_lowercase()
    .encode_utf16()
    .filter(|&x| (x < 0x300 || x > 0x36F) && x != 0x489)
    .collect::<Vec<_>>()
}

#[napi]
fn decancer(raw_input: JsString) -> Result<String, Error> {
  let mut bytes = parse_case_sensitive(raw_input.into_utf16()?.as_slice());
  alphabet::parse(&mut bytes);
  
  bytes.retain(|&x| (x < 0xD800 || x > 0xDB7F) && x < 0xFFF0);
  
  Ok(String::from_utf16_lossy(&bytes[..]).replace(char::REPLACEMENT_CHARACTER, ""))
}