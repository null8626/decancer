# decancer

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
- JavaScript ([Node.js](https://www.npmjs.com/package/decancer)/[Deno](https://deno.land/x/decancer@v1.5.1)/Browser)
- [Python](https://pypi.org/project/decancer-py/) (unofficial)

## installation

### Rust

In your `Cargo.toml`:

```toml
decancer = "1.5.1"
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
import init from "https://deno.land/x/decancer@v1.5.1/mod.ts"

const decancer = await init()
```

### Browser

In your code:

```html
<script type="module">
  import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.5.1/decancer.min.js"

  const decancer = await init()
</script>
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
      import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.5.1/decancer.min.js"
      
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