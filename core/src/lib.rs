//! # decancer [![npm][npm-image]][npm-url] [![downloads][downloads-image]][downloads-url]
//! 
//! [npm-image]: https://img.shields.io/npm/v/decancer.svg
//! [npm-url]: https://npmjs.org/package/decancer
//! [downloads-image]: https://img.shields.io/npm/dm/decancer.svg
//! [downloads-url]: https://npmjs.org/package/decancer
//! 
//! A tiny package that removes common confusables from strings.
//! 
//! - It's core is written in [Rust](https://www.rust-lang.org) and utilizes a form of **Binary Search** to ensure speed!
//! - It stores it's huge collection of confusables in a [customized binary file](https://github.com/null8626/decancer/blob/main/core/bin/confusables.bin) instead of a huge JSON or text file to optimize it's bundle size!
//! - It supports curing **4,299 different confusables** into cured-lowercased-strings, including but not limited to:
//!   - Accented characters
//!   - [Byte order mark](https://en.wikipedia.org/wiki/Byte_order_mark)
//!   - [Control characters](https://en.wikipedia.org/wiki/Control_character)
//!   - [Most homoglyphs](https://en.wikipedia.org/wiki/Homoglyph)
//!   - Several foreign characters, including but not limited to [Cyrillic](https://en.wikipedia.org/wiki/Cyrillic_script), [Greek](https://en.wikipedia.org/wiki/Greek_alphabet), and [Japanese](https://en.wikipedia.org/wiki/Kanji)
//!   - Several emojis
//!   - [Whitespace characters](https://en.wikipedia.org/wiki/Whitespace_character)
//!   - [Zalgo text](https://en.wikipedia.org/wiki/Zalgo_text)
//! - And it's supported in the following languages:
//!   - [Rust](https://crates.io/crates/decancer)
//!   - JavaScript ([Node.js/Deno/Bun](https://www.npmjs.com/package/decancer)/Browser)
//!   - C/C++
//!   - [Python](https://pypi.org/project/decancer-py) (unofficial)
//! 
//! ## Installation
//! 
//! <details>
//! <summary>Rust</summary>
//! 
//! In your `Cargo.toml`:
//! 
//! ```toml
//! decancer = "1.5.3"
//! ```
//! 
//! </details>
//! <details>
//! <summary>Node.js</summary>
//! 
//! In your shell:
//! 
//! ```console
//! $ npm install decancer
//! ```
//! 
//! In your code:
//! 
//! ```js
//! const decancer = require('decancer')
//! ```
//! 
//! </details>
//! <details>
//! <summary>Deno</summary>
//! 
//! In your code:
//! 
//! ```ts
//! import decancer from 'npm:decancer'
//! ```
//! 
//! </details>
//! <details>
//! <summary>Bun</summary>
//! 
//! In your shell:
//! 
//! ```console
//! $ bun install decancer
//! ```
//! 
//! In your code:
//! 
//! ```js
//! const decancer = require('decancer')
//! ```
//! 
//! </details>
//! <details>
//! <summary>Browser</summary>
//! 
//! In your code:
//! 
//! ```html
//! <script type="module">
//!   import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v1.5.3/bindings/wasm/bin/decancer.min.js'
//! 
//!   const decancer = await init()
//! </script>
//! ```
//! 
//! </details>
//! <details>
//! <summary>C/C++</summary>
//! 
//! ### Download precompiled binaries
//! 
//! - [Download for 64-bit Windows MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-x86_64-pc-windows-msvc.zip)
//! - [Download for 32-bit Windows MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-i686-pc-windows-msvc.zip)
//! - [Download for ARM64 Windows MSVC](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-aarch64-pc-windows-msvc.zip)
//! - [Download for 64-bit macOS (10.7+, Lion+)](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-x86_64-apple-darwin.zip)
//! - [Download for ARM64 macOS (11.0+, Big Sur+)](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-aarch64-apple-darwin.zip)
//! - [Download for 64-bit Linux (kernel 3.2+, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-x86_64-unknown-linux-gnu.zip)
//! - [Download for 64-bit Linux with MUSL](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-x86_64-unknown-linux-musl.zip)
//! - [Download for ARM64 Linux (kernel 4.1, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-aarch64-unknown-linux-gnu.zip)
//! - [Download for ARM64 Linux with MUSL](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-aarch64-unknown-linux-musl.zip)
//! - [Download for ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-armv7-unknown-linux-gnueabihf.zip)
//! - [Download for 64-bit FreeBSD](https://github.com/null8626/decancer/releases/download/v1.5.3/decancer-freebsd.zip)
//! 
//! ### Building from source
//! 
//! Prerequisites:
//! 
//! - [Git](https://git-scm.com/)
//! - [Rust](https://rustup.rs/)
//! 
//! ```console
//! $ git clone https://github.com/null8626/decancer.git --depth 1
//! $ cd decancer/bindings/native
//! $ cargo build --release
//! ```
//! 
//! </details>
//! 
//! ## Examples
//! 
//! > **NOTE:** cured output will ALWAYS be in lowercase.
//! 
//! <details>
//! <summary>JavaScript</summary>
//! 
//! ```js
//! const cured = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')
//! 
//! // cured here is a CuredString object wrapping over the cured string
//! // for comparison purposes, it's more recommended to use the methods provided by the CuredString class.
//! 
//! if (cured.contains('funny')) {
//!   console.log('found the funny')
//! }
//! 
//! if (
//!   cured.equals('very funny text') &&
//!   cured.startsWith('very') &&
//!   cured.endsWith('text')
//! ) {
//!   console.log('it works!')
//! }
//! 
//! console.log(cured.toString()) // 'very funny text'
//! ```
//! 
//! </details>
//! <details>
//! <summary>Rust</summary>
//! 
//! ```rust
//! extern crate decancer;
//! 
//! fn main() {
//!   let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
//! 
//!   // cured here is a decancer::CuredString struct wrapping over the cured string
//!   // for comparison purposes, it's more recommended to use the methods provided by the decancer::CuredString struct.
//! 
//!   assert_eq!(cured, "very funny text");
//!   assert!(cured.starts_with("very"));
//!   assert!(cured.contains("funny"));
//!   assert!(cured.ends_with("text"));
//! 
//!   let _output_str = cured.into_str(); // retrieve the String inside and consume the struct.
//! }
//! ```
//! 
//! </details>
//! <details>
//! <summary>Web app example</summary>
//! 
//! ```html
//! <!DOCTYPE html>
//! <html lang="en">
//!   <head>
//!     <meta charset="utf-8" />
//!     <title>Decancerer!!! (tm)</title>
//!     <style>
//!       textarea {
//!         font-size: 30px;
//!       }
//! 
//!       #cure {
//!         font-size: 20px;
//!         padding: 5px 30px;
//!       }
//!     </style>
//!   </head>
//!   <body>
//!     <h3>Input cancerous text here:</h3>
//!     <textarea rows="10" cols="30"></textarea>
//!     <br />
//!     <button id="cure" onclick="cure()">cure!</button>
//!     <script type="module">
//!       import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v1.5.3/bindings/wasm/bin/decancer.min.js'
//! 
//!       const decancer = await init()
//! 
//!       window.cure = function () {
//!         const textarea = document.querySelector('textarea')
//! 
//!         if (!textarea.value.length) {
//!           return alert("There's no text!!!")
//!         }
//! 
//!         textarea.value = decancer(textarea.value).toString()
//!       }
//!     </script>
//!   </body>
//! </html>
//! ```
//! 
//! </details>
//! <details>
//! <summary>C++11 UTF-8 example</summary>
//! 
//! ```cpp
//! #include <decancer.h>
//! 
//! #include <cstring>
//! #include <cstdlib>
//! #include <cstdio>
//! 
//! // global variable for assertion purposes only
//! decancer_cured_t cured;
//! 
//! // our quick assert function
//! static void assert(const bool expr, char * message) {
//!   if (!expr) {
//!     fprintf(stderr, "assertion failed (%s)\n", message);
//!     decancer_free(cured); // clean things up before exiting
//!     exit(1);
//!   }
//! }
//! 
//! int main(void) {
//!   uint8_t string[] = u8"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
//! 
//!   // cure string
//!   cured = decancer_cure(string, sizeof(string) - sizeof(uint8_t));
//! 
//!   // comparisons
//!   assert(decancer_equals(cured, "very funny text", 15), "equals");
//!   assert(decancer_starts_with(cured, "very", 4), "starts_with");
//!   assert(decancer_ends_with(cured, "text", 4), "ends_with");
//!   assert(decancer_contains(cured, "funny", 5), "contains");
//! 
//!   // coerce output as a raw UTF-8 pointer and retrieve it's size (in bytes)
//!   size_t output_size;
//!   const uint8_t * output_raw = decancer_raw(cured, &output_size);
//! 
//!   // assert raw cured utf-8 size to be 15 bytes (size of "very funny text")
//!   assert(output_size == 15, "raw output size");
//! 
//!   // utf-8 bytes for "very funny text"
//!   const uint8_t expected_raw[] = { 0x76, 0x65, 0x72, 0x79, 0x20,
//!                                    0x66, 0x75, 0x6e, 0x6e, 0x79,
//!                                    0x20, 0x74, 0x65, 0x78, 0x74 };
//! 
//!   char assert_message[38];
//!   for (uint32_t i = 0; i < sizeof(expected_raw); i++) {
//!     sprintf(assert_message, "mismatched utf-8 contents at index %u", i);
//!     assert(output_raw[i] == expected_raw[i], assert_message);
//!   }
//! 
//!   // free cured string (required)
//!   decancer_free(cured);
//!   return 0;
//! }
//! ```
//! 
//! </details>
//! <details>
//! <summary>C UTF-16 example</summary>
//! 
//! ```c
//! #include <decancer.h>
//! 
//! #include <stdlib.h>
//! #include <stdio.h>
//! 
//! // global variable for assertion purposes only
//! decancer_cured_t cured;
//! wdecancer_raw_cured_t output_raw = NULL;
//! 
//! // our quick assert function
//! static void assert(const bool expr, const char * message) {
//!   if (!expr) {
//!     fprintf(stderr, "assertion failed (%s)\n", message);
//! 
//!     // clean things up before exiting
//!     if (output_raw != NULL) {
//!       wdecancer_raw_free(output_raw);
//!       output_raw = NULL;
//!     }
//! 
//!     decancer_free(cured);
//!     exit(1);
//!   }
//! }
//! 
//! int main(void) {
//!   wchar_t string[] = L"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
//! 
//!   // cure string
//!   cured = wdecancer_cure(string, (sizeof(string) - sizeof(wchar_t)) / sizeof(wchar_t));
//! 
//!   // comparisons
//!   assert(wdecancer_equals(cured, L"very funny text", 15), "wide equals");
//!   assert(wdecancer_starts_with(cured, L"very", 4), "wide starts_with");
//!   assert(wdecancer_ends_with(cured, L"text", 4), "wide ends_with");
//!   assert(wdecancer_contains(cured, L"funny", 5), "wide contains");
//! 
//!   // coerce output as a raw UTF-16 pointer and retrieve it's length (in CHARACTERS)
//!   size_t output_length;
//!   output_raw = wdecancer_raw(cured, &output_length);
//!   const wchar_t * output_raw_ptr = wdecancer_raw_ptr(output_raw);
//! 
//!   // assert raw cured utf-16 length to be 15 characters (length of "very funny text", NOT in bytes)
//!   assert(output_length == 15, "wide raw output size");
//! 
//!   // utf-16 bytes for "very funny text"
//!   const wchar_t expected_raw[] = { 0x76, 0x65, 0x72, 0x79, 0x20,
//!                                    0x66, 0x75, 0x6e, 0x6e, 0x79,
//!                                    0x20, 0x74, 0x65, 0x78, 0x74 };
//! 
//!   char assert_message[39];
//!   for (uint32_t i = 0; i < sizeof(expected_raw) / sizeof(wchar_t); i++) {
//!     sprintf(assert_message, "mismatched utf-16 contents at index %u", i);
//!     assert(output_raw_ptr[i] == expected_raw[i], assert_message);
//!   }
//! 
//!   // free raw cured UTF-16 string (required)
//!   wdecancer_raw_free(output_raw);
//! 
//!   // free cured string (required)
//!   decancer_free(cured);
//!   return 0;
//! }
//! ```
//! 
//! </details>
//! 
//! ## Contributing
//! 
//! Please [read `CONTRIBUTING.md`](https://github.com/null8626/decancer/blob/main/CONTRIBUTING.md) for newbie contributors who want to contribute!
//! 
//! ## Special thanks
//! 
//! These are the primary resources that made this project possible.
//! 
//! - [The Official Unicode Confusables List](https://util.unicode.org/UnicodeJsps/confusables.jsp)
//! - [The Official Unicode Characters List](https://unicode.org/Public/UNIDATA/UnicodeData.txt)
//! - [Wikipedia's list of Unicode Characters](https://en.wikipedia.org/wiki/List_of_Unicode_characters)
//! - [Fancy Text Generator](https://lingojam.com/FancyTextGenerator)
//! - [Unicode character inspector](https://apps.timwhitlock.info/unicode/inspect)
//! - [`napi-rs` for integrating Rust into the Node.js ecosystem](https://napi.rs/)
//! - [`wasm-bindgen` for making the development of WebAssembly modules in Rust easier](https://github.com/rustwasm/wasm-bindgen)

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
