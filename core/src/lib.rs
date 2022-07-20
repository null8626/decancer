//! # decancer
//!
//! A portable module that removes common confusables from strings without the use of Regexes. Available for Rust, Node.js, Deno, and the Browser.
//!
//! Pros:
//!
//! - Extremely fast, no use of regex whatsoever!
//! - No dependencies.
//! - Simple to use, just one single function.
//! - Supports all the way to UTF-32 code-points. Like emojis, zalgos, etc.
//! - While this project may not be perfect, it should cover the vast majority of confusables.
//!
//! Con:
//!
//! - Remember that this project is not perfect, false-positives may happen.
//!
//! ## installation
//!
//! ### Rust
//!
//! In your `Cargo.toml`:
//!
//! ```toml
//! decancer = "1.4.2"
//! ```
//!
//! ### Node.js
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
//! const decancer = require('decancer');
//! ```
//!
//! ### Deno
//!
//! In your code:
//!
//! ```ts
//! import init from "https://deno.land/x/decancer@v1.4.2/mod.ts";
//!
//! const decancer = await init();
//! ```
//!
//! ### Browser
//!
//! In your code:
//!
//! ```js
//! import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.4.2/decancer.min.js";
//!
//! const decancer = await init();
//! ```
//!
//! ## examples
//!
//! > **NOTE:** cured output will ALWAYS be in lowercase.
//!
//! ### JavaScript
//!
//! ```js
//! const noCancer = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣');
//!
//! console.log(noCancer); // 'very funny text'
//! ```
//!
//! ### Rust
//!
//! ```rust
//! extern crate decancer;
//! use decancer::Decancer;
//!
//! fn main() {
//!   let instance = Decancer::new();
//!   let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
//!
//!   assert_eq!(output, String::from("very funny text"));
//! }
//! ```
//!
//! If you want to check if the decancered string contains a certain keyword, i recommend using this instead since mistranslations can happen (e.g mistaking the number 0 with the letter O)
//!
//! ### JavaScript
//!
//! ```js
//! const noCancer = decancer(someString);
//!
//! if (decancer.contains(noCancer, 'no-no-word')) console.log('LANGUAGE!!!');
//! ```
//!
//! ### Rust
//!
//! ```rust
//! extern crate decancer;
//! use decancer::Decancer;
//!
//! fn main() {
//!   let instance = Decancer::new();
//!   let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
//!   
//!   if instance.contains(&output, "funny") {
//!     println!("i found the funny");
//!   }
//! }
//! ```
//!
//! ### Web app example
//!
//! ```html
//! <!DOCTYPE html>
//! <html lang="en">
//!   <head>
//!     <meta charset="utf-8">
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
//!       import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.4.2/decancer.min.js";
//!       
//!       const decancer = await init();
//!       
//!       window.cure = function () {
//!         const textarea = document.querySelector("textarea");
//!         
//!         if (!textarea.value.length) {
//!           return alert("There's no text!!!");
//!         }
//!         
//!         textarea.value = decancer(textarea.value);
//!       }
//!     </script>
//!   </body>
//! </html>
//! ```
//!
//! ## contributions
//!
//! All contributions are welcome. Feel free to fork the project at GitHub! &lt;3
//!
//! If you want to add, remove, modify, or view the list of supported confusables, you can clone the [GitHub repository](https://github.com/null8626/decancer), and modify it directly with Node.js. Either through a script or directly from the REPL.
//!
//! ```js
//! const reader = await import('./contrib/index.mjs');
//! const data = reader.default('./core/bin/confusables.bin');
//!
//! // do something with data...
//!
//! data.save('./core/bin/confusables.bin');
//! ```
//!
//! ## special thanks
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

mod confusables;
mod contains;
mod utf16;
mod utf32;
mod utf8;

use confusables::*;
use contains::contains;
pub use utf32::ToCodepoints;

/// A Decancer instance. The instance here stores the supported confusables in pointers instead of arrays.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,no_run
/// extern crate decancer;
/// use decancer::Decancer;
///
/// let instance = Decancer::new();
/// let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
///
/// assert_eq!(output, String::from("very funny text"));
/// ```
#[derive(Copy, Clone)]
#[must_use]
pub struct Decancer {
  numerical: BinaryArray<u32>,
  misc_case_sensitive: ConfusablesMap,
  alphabetical_pattern: BinaryArray<u32>,
  alphabetical: Alphabetical,
  misc: ConfusablesMap,
  similar: Similar,
}

impl Decancer {
  /// Creates a Decancer instance. The constructor here is const and never fails, which means that you can put it as a constant global variable.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,no_run
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// ```
  ///
  /// Or use it in a global state:
  ///
  /// ```rust,no_run
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// const DECANCER: Decancer = Decancer::new();
  /// ```
  pub const fn new() -> Self {
    Self {
      numerical: confusables::numerical(),
      misc_case_sensitive: confusables::misc_case_sensitive(),
      alphabetical_pattern: confusables::alphabetical_pattern(),
      alphabetical: confusables::alphabetical(),
      misc: confusables::misc(),
      similar: confusables::similar(),
    }
  }

  #[inline(always)]
  fn similar(&self, a: u32, b: u32) -> bool {
    a == b
      || ((a <= 0xFF)
        && (b <= 0xFF)
        && self
          .similar
          .iter()
          .any(|x| x.contains(a as _) && x.contains(b as _)))
  }

  /// Checks if string B is in string A. This one is more accurate than A.contains(B) because it also checks if the strings are similar. (e.g `1` with `i`... etc)
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  ///
  /// assert_eq!(true, instance.contains(&output, "funny"));
  /// ```
  #[inline(always)]
  #[must_use]
  pub fn contains<'a, A, B>(&self, a: &'a A, b: &'a B) -> bool
  where
    A: ToCodepoints<'a> + ?Sized,
    B: ToCodepoints<'a> + ?Sized,
  {
    contains(a.to_codepoints(), b.to_codepoints(), |a, b| {
      self.similar(a, b)
    })
  }

  // similar to char::is_whitespace, except ignores the default space
  // and also supports more codepoints.
  const fn is_whitespace(c: u32) -> bool {
    match c {
      0x09
      | 0x0A..=0x0D
      | 0x85
      | 0xA0
      | 0x1680
      | 0x2000..=0x200A
      | 0x2028
      | 0x2029
      | 0x202F
      | 0x205F
      | 0x3000
      | 0x180E
      | 0x200B..=0x200D
      | 0x2060
      | 0xFEFF => true,
      _ => false,
    }
  }

  const fn filter_codepoint(x: u32) -> bool {
    ((x > 31 && x < 127) || (x > 159 && x < 0x300) || x > 0x36F)
      && x != 0x20E3
      && x != 0xFE0F
      && x != 0x489
  }

  const fn valid_codepoint(c: u16) -> bool {
    (c < 0xD800 || c > 0xDB7F) && c < 0xFFF0
  }

  /// Cures a string.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance = Decancer::new();
  /// let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  ///
  /// assert_eq!(output, String::from("very funny text"));
  /// ```
  #[must_use]
  pub fn cure<'a, S>(&self, s: &'a S) -> String
  where
    S: ToCodepoints<'a> + ?Sized,
  {
    let mut output = String::with_capacity(s.approximate_chars());

    s.to_codepoints()
      .filter(|&x| Self::filter_codepoint(x))
      .map(|x| {
        if Self::is_whitespace(x) {
          0x20 // char code for ' '
        } else {
          x
        }
      })
      .for_each(|x| {
        for num in self.numerical.iter() {
          if x >= num && x <= (num + 9) {
            return output.push(unsafe { char::from_u32_unchecked(x - num + 0x30) });
          }
        }

        for (key, value) in self.misc_case_sensitive.iter() {
          if value.contains(x) {
            for k in key.iter() {
              output.push(k as char);
            }

            return;
          }
        }

        if let Some(c22) = char::from_u32(x) {
          c22.to_lowercase().for_each(|c2| {
            let c = c2 as u32;

            for pat in self.alphabetical_pattern.iter() {
              if c >= pat && c <= (pat + 25) {
                return output.push(unsafe { char::from_u32_unchecked(c - pat + 0x61) });
              }
            }

            for (i, arr) in self.alphabetical.iter().enumerate() {
              if arr.contains(c) {
                return output.push(unsafe { char::from_u32_unchecked((i as u32) + 0x61) });
              }
            }

            for (key, value) in self.misc.iter() {
              if value.contains(c) {
                for k in key.iter() {
                  output.push(k as char);
                }

                return;
              }
            }

            output.push(unsafe { char::from_u32_unchecked(c) });
          });
        }
      });

    output.retain(|c2| {
      let (a, b) = utf16::from(c2 as _);

      if Self::valid_codepoint(a) {
        match b {
          Some(b2) => Self::valid_codepoint(b2),
          None => true,
        }
      } else {
        false
      }
    });

    output
  }
}

impl Default for Decancer {
  /// Creates a Decancer instance. The constructor here is const and never fails.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust
  /// extern crate decancer;
  /// use decancer::Decancer;
  ///
  /// let instance: Decancer = Default::default();
  /// ```
  #[inline(always)]
  fn default() -> Self {
    Self::new()
  }
}

// this is safe to use in multiple threads!
unsafe impl Send for Decancer {}
unsafe impl Sync for Decancer {}

#[test]
fn it_works() {
  let instance = Decancer::new();
  let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  assert_eq!(output, String::from("very funny text"));
  assert_eq!(true, instance.contains(&output, "funny"));
}
