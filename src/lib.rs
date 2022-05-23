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

const SIMILARITIES: [&[u16]; 8] = [
  &[0x31, 0x69, 0x7c, 0x6C],
  &[0x6f, 0x30],
  &[0x63, 0x28],
  &[0x69, 0x76],
  &[0x73, 0x35, 0x24],
  &[0x34, 0x61],
  &[0x37, 0x74],
  &[0x36, 0x62]
];

fn similar(a: u16, b: u16) -> bool {
  if a == b {
    return true;
  }
  
  for elem in SIMILARITIES {
    if elem.contains(&a) && elem.contains(&b) {
      return true;
    }
  }

  false
}

fn contains_inner(a: &[u16], b: &[u16], a_len: usize, b_len: usize) -> bool {
  let mut j = 0usize;

  for i in 0usize..a_len {
    if similar(a[i], b[j]) {
      j += 1;

      if j == b_len {
        return true;
      }
    } else {
      j = 0usize;
    }
  }

  false
}

#[napi]
fn contains(input: JsString, other: JsString) -> Result<bool, Error> {
  let a = input.utf16_len()?;
  let b = other.utf16_len()?;

  if a == 0 || a < b {
    Ok(false)
  } else {
    Ok(contains_inner(input.into_utf16()?.as_slice(), other.into_utf16()?.as_slice(), a, b))
  }
}

#[napi]
fn decancer(raw_input: JsString) -> Result<String, Error> {
  let mut bytes = parse_case_sensitive(raw_input.into_utf16()?.as_slice());
  alphabet::parse(&mut bytes);
  
  bytes.retain(|&x| (x < 0xD800 || x > 0xDB7F) && x < 0xFFF0);
  
  Ok(String::from_utf16_lossy(&bytes[..]).replace(char::REPLACEMENT_CHARACTER, ""))
}