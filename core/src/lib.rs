//! # decancer
//!
//! A tiny package that removes common confusables from strings.
//!
//! Pros:
//!
//! - BLAZINGLY FAST™ 🚀🚀🚀, no use of regex whatsoever!
//! - No use of any external dependencies.
//! - Very simple to use!
//! - Supports more than **3000 unicode codepoints**. This should cover the vast majority of confusables, including emojis, zalgos, etc.
//!
//! Con:
//!
//! - Remember that this project is not perfect, false-positives may happen.

#![deny(clippy::all)]
#![allow(
  clippy::transmute_int_to_char,
  clippy::or_fun_call,
  clippy::ptr_offset_with_cast,
  clippy::from_over_into,
  dead_code
)]

mod matcher;
mod similar;
mod string;
mod util;

#[cfg(test)]
mod tests;

use std::{cmp::Ordering, mem::transmute};
pub use string::CuredString;

#[inline(always)]
fn to_lowercase(code: u32) -> u32 {
  unsafe {
    transmute::<_, char>(code)
      .to_lowercase()
      .next()
      .unwrap_unchecked() as _
  }
}

const fn invalid_codepoint(x: u32) -> bool {
  x <= 31
    || (x >= 127 && x <= 159)
    || (x >= 0x300 && x <= 0x36F)
    || x == 0x20E3
    || x == 0xFE0F
    || x == 0xFEFF
    || x == 0xFFFD
    || x == 0x489
}

/// Cures a string.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// extern crate decancer;
///  
/// let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
/// assert_eq!(cured, "very funny text");
/// ```
pub fn cure<S: AsRef<str> + ?Sized>(input: &S) -> CuredString {
  let input_s = input.as_ref();
  let mut output = CuredString::with_capacity(input_s.len());

  input_s.chars().for_each(|code| {
    if invalid_codepoint(code as _) {
      return;
    }

    let code_lowercased = to_lowercase(code as _);
    if code_lowercased < 0x80 {
      return output.push_code(code_lowercased); // process of elimination
    }

    let mut start = 0;
    let mut end = matcher::CONFUSABLES_COUNT;
    let mut end_flag = false;

    loop {
      let mid = (((start + end) as f32) / 2f32).floor() as u16;
      let confusable = matcher::Confusable::at(mid);

      match confusable.matches(code as _, code_lowercased) {
        Ordering::Equal => {
          return output.push_translation(confusable.translation(code as _, code_lowercased))
        }
        Ordering::Greater => start = mid + 1,
        _ => end = mid,
      };

      if end_flag {
        return output.push_code(code_lowercased);
      }

      end_flag = start == end;
    }
  });

  output.finishing();
  output
}
