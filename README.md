# decancer [![npm][npm-image]][npm-url] [![downloads][downloads-image]][downloads-url]

[npm-image]: https://img.shields.io/npm/v/decancer.svg
[npm-url]: https://npmjs.org/package/decancer
[downloads-image]: https://img.shields.io/npm/dm/decancer.svg
[downloads-url]: https://npmjs.org/package/decancer

A tiny package that removes common confusables from strings.

Pros:

- BLAZINGLY FAST™ 🚀🚀🚀, no use of regex whatsoever!
- No use of any external dependencies.
- Very simple to use!
- Supports more than **3000 unicode codepoints**. This should cover the vast majority of confusables, including emojis, zalgos, etc.

Con:

- Remember that this project is not perfect, false-positives may happen.

This library is available in the following languages:

- [Rust](https://crates.io/crates/decancer)
- JavaScript ([Node.js](https://www.npmjs.com/package/decancer)/[Deno](https://deno.land/x/decancer@v1.5.3)/Browser)
- C/C++
- [Python](https://pypi.org/project/decancer-py/) (unofficial)

## installation

### Rust

In your `Cargo.toml`:

```toml
decancer = "1.5.3"
```

### Node.js

In your shell:

```console
$ npm install decancer
```

In your code:

```js
const decancer = require('decancer')
```

### Deno

In your code:

```ts
import init from "https://deno.land/x/decancer@v1.5.3/mod.ts"

const decancer = await init()
```

### Browser

In your code:

```html
<script type="module">
  import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.5.3/decancer.min.js"

  const decancer = await init()
</script>
```

## C/C++

### Building from source

Prerequisites:

- [Git](https://git-scm.com/)
- [Rust](https://rustup.rs/)

```console
$ git clone https://github.com/null8626/decancer.git --depth 1
$ cd decancer/native
$ cargo build --release
```

## examples

> **NOTE:** cured output will ALWAYS be in lowercase.

### JavaScript

```js
const cured = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')

// cured here is a CuredString object wrapping over the cured string
// for comparison purposes, it's more recommended to use the methods provided by the CuredString class.

if (cured.contains('funny')) {
  console.log('found the funny')
}

if (cured.equals('very funny text') && cured.startsWith('very') && cured.endsWith('text')) {
  console.log('it works!')
}

console.log(cured.toString()); // 'very funny text'
```

### Rust

```rust
extern crate decancer;

fn main() {
  let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  // cured here is a decancer::CuredString struct wrapping over the cured string
  // for comparison purposes, it's more recommended to use the methods provided by the decancer::CuredString struct.
  
  assert_eq!(output, "very funny text");
  assert!(output.starts_with("very"));
  assert!(output.contains("funny"));
  assert!(output.ends_with("text"));

  let _output_str = output.into_str(); // retrieve the String inside and consume the struct.
}
```

### Web app example

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Decancerer!!! (tm)</title>
    <style>
      textarea {
        font-size: 30px;
      }
      
      #cure {
        font-size: 20px;
        padding: 5px 30px;
      }
    </style>
  </head>
  <body>
    <h3>Input cancerous text here:</h3>
    <textarea rows="10" cols="30"></textarea>
    <br />
    <button id="cure" onclick="cure()">cure!</button>
    <script type="module">
      import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.5.3/decancer.min.js"
      
      const decancer = await init()
      
      window.cure = function () {
        const textarea = document.querySelector("textarea")
        
        if (!textarea.value.length) {
          return alert("There's no text!!!")
        }
        
        textarea.value = decancer(textarea.value).toString()
      }
    </script>
  </body>
</html>
```

### C++11 example

> **NOTE:** **ALL** input strings **MUST** be in the ASCII/UTF-8 encoding.

```cpp
#include <decancer.h>
#include <cstdlib>
#include <cstdio>

// our quick assert function
static inline void assert(const bool expr, const char * message) {
  if (!expr) {
    fprintf(stderr, "assertion failed (%s)\n", message);
	exit(1);
  }
}

int main(void) {
  uint8_t string[] = u8"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
  
  // cure string
  decancer_cured_t cured = decancer_cure(string, sizeof(string) - sizeof(uint8_t));
  
  // comparisons
  assert(decancer_equals(cured, "very funny text", 15), "equals");
  assert(decancer_starts_with(cured, "very", 4), "starts_with");
  assert(decancer_ends_with(cured, "text", 4), "ends_with");
  assert(decancer_contains(cured, "funny", 15), "contains");
  
  // coerce output as a raw UTF-8 pointer and retrieve it's size (in bytes)
  uint8_t * output_raw;
  const size_t output_size = decancer_cured_string(cured, &output_raw);
  
  // free cured string (required)
  decancer_free(cured);
  return 0;
}
```

## contributing

If you want to contribute, i appreciate that!!! ❤️❤️❤️<br>
Please [read `CONTRIBUTING.md`](https://github.com/null8626/decancer/blob/main/CONTRIBUTING.md) for more details! ❤️

## special thanks

These are the primary resources that made this project possible.

- [The Official Unicode Confusables List](https://util.unicode.org/UnicodeJsps/confusables.jsp)
- [The Official Unicode Characters List](https://unicode.org/Public/UNIDATA/UnicodeData.txt)
- [Wikipedia's list of Unicode Characters](https://en.wikipedia.org/wiki/List_of_Unicode_characters)
- [Fancy Text Generator](https://lingojam.com/FancyTextGenerator)
- [Unicode character inspector](https://apps.timwhitlock.info/unicode/inspect)
- [`napi-rs` for integrating Rust into the Node.js ecosystem](https://napi.rs/)
- [`wasm-bindgen` for making the development of WebAssembly modules in Rust easier](https://github.com/rustwasm/wasm-bindgen)