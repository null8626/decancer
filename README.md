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

## Installation

<details>
<summary>Rust</summary>

In your `Cargo.toml`:

```toml
decancer = "1.5.3"
```
</details>
<details>
<summary>Node.js</summary>

In your shell:

```console
$ npm install decancer
```

In your code:

```js
const decancer = require('decancer')
```
</details>
<details>
<summary>Deno</summary>

In your code:

```ts
import init from "https://deno.land/x/decancer@v1.5.3/mod.ts"

const decancer = await init()
```
</details>
<details>
<summary>Browser</summary>

In your code:

```html
<script type="module">
  import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.5.3/decancer.min.js"

  const decancer = await init()
</script>
```
</details>
<details>
<summary>C/C++</summary>

Prerequisites:

- [Git](https://git-scm.com/)
- [Rust](https://rustup.rs/)

```console
$ git clone https://github.com/null8626/decancer.git --depth 1
$ cd decancer/native
$ cargo build --release
```
</details>

## Examples

> **NOTE:** cured output will ALWAYS be in lowercase.

<details>
<summary>JavaScript</summary>

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
</details>
<details>
<summary>Rust</summary>

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
</details>
<details>
<summary>Web app example</summary>

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
</details>
<details>
<summary>C++11 UTF-8 example</summary>

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
  assert(decancer_contains(cured, "funny", 5), "contains");
  
  // coerce output as a raw UTF-8 pointer and retrieve it's size (in bytes)
  size_t output_size;
  const uint8_t * output_raw = decancer_raw(cured, &output_size);
  
  // free cured string (required)
  decancer_free(cured);
  return 0;
}
```
</details>
<details>
<summary>C UTF-16 example</summary>

```c
#include <decancer.h>

#include <stdlib.h>
#include <stdio.h>

// our quick assert function
static inline void assert(const bool expr, const char * message) {
  if (!expr) {
    fprintf(stderr, "assertion failed (%s)\n", message);
    exit(1);
  }
}

int main(void) {
  uint16_t string[] = L"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣";
  
  // cure string
  decancer_cured_t cured = wdecancer_cure(string, sizeof(string) - sizeof(uint16_t));
  
  // comparisons
  assert(wdecancer_equals(cured, L"very funny text", 15 * sizeof(uint16_t)), "equals");
  assert(wdecancer_starts_with(cured, L"very", 4 * sizeof(uint16_t)), "starts_with");
  assert(wdecancer_ends_with(cured, L"text", 4 * sizeof(uint16_t)), "ends_with");
  assert(wdecancer_contains(cured, L"funny", 5 * sizeof(uint16_t)), "contains");
  
  // coerce output as a raw UTF-16 pointer and retrieve it's size (in bytes)
  size_t output_size;
  wdecancer_raw_cured_t output_raw = wdecancer_raw(cured, &output_size);
  const uint16_t * output_raw_ptr = wdecancer_raw_ptr(output_raw);
  
  // free raw cured UTF-16 string (required)
  wdecancer_raw_free(output_raw);
  
  // free cured string (required)
  decancer_free(cured);
  return 0;
}
```
</details>

## Contributing

If you want to contribute, i appreciate that!!! ❤️❤️❤️<br>
Please [read `CONTRIBUTING.md`](https://github.com/null8626/decancer/blob/main/CONTRIBUTING.md) for more details! ❤️

## Special thanks

These are the primary resources that made this project possible.

- [The Official Unicode Confusables List](https://util.unicode.org/UnicodeJsps/confusables.jsp)
- [The Official Unicode Characters List](https://unicode.org/Public/UNIDATA/UnicodeData.txt)
- [Wikipedia's list of Unicode Characters](https://en.wikipedia.org/wiki/List_of_Unicode_characters)
- [Fancy Text Generator](https://lingojam.com/FancyTextGenerator)
- [Unicode character inspector](https://apps.timwhitlock.info/unicode/inspect)
- [`napi-rs` for integrating Rust into the Node.js ecosystem](https://napi.rs/)
- [`wasm-bindgen` for making the development of WebAssembly modules in Rust easier](https://github.com/rustwasm/wasm-bindgen)
