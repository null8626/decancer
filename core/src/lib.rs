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
mod util; // for internal use

#[cfg(test)]
mod tests;

use std::{cmp::Ordering, mem::transmute};
pub use string::CuredString;

#[inline(always)]
fn to_lowercase(code: u32) -> u32 {
  unsafe { transmute::<_, char>(code) }
    .to_lowercase()
    .next()
    .unwrap_or(unsafe { transmute(code) }) as _
}

const fn valid_codepoint(x: u32) -> bool {
  ((x > 31 && x < 127) || (x > 159 && x < 0x300) || x > 0x36F)
    && x != 0x20E3
    && x != 0xFE0F
    && x != 0x489
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
    if !valid_codepoint(code as _) {
      return;
    }

    let code_lowercased = to_lowercase(code as _);
    let mut start = 0;
    let mut end = matcher::CONFUSABLES_COUNT;

    loop {
      let mid = (((start + end) as f32) / 2f32).floor() as u16;
      let confusable = matcher::Confusable::at(mid);

      match confusable.matches(code as _, code_lowercased) {
        Ordering::Equal => {
          output.push_translation(confusable.translation(code as _, code_lowercased));
          return;
        }
        Ordering::Greater => start = mid + 1,
        _ => end = mid, // Ordering::Less
      }

      if start == end {
        let confusable2 = matcher::Confusable::at(start); // end works too

        if confusable2.matches(code as _, code_lowercased) == Ordering::Equal {
          output.push_translation(confusable2.translation(code as _, code_lowercased));
        } else {
          output.push_code(code_lowercased);
        }

        return;
      }
    }
  });

  output.finishing();
  output
}