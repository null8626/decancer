# decancer

A portable module that removes common confusables from strings without the use of Regexes. Available for Rust, Node.js, Deno, and the Browser.

Pros:

- Extremely fast, no use of regex whatsoever!
- No dependencies.
- Simple to use, just one single function.
- Supports all the way to UTF-32 code-points. Like emojis, zalgos, etc.
- While this project may not be perfect, it should cover the vast majority of confusables.

Con:

- Remember that this project is not perfect, false-positives may happen.

## installation

### Rust

In your `Cargo.toml`:

```toml
decancer = "1.4.2"
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
import init from "https://deno.land/x/decancer@v1.4.2/mod.ts"

const decancer = await init()
```

### Browser

In your code:

```html
<script type="module">
  import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.4.2/decancer.min.js"

  const decancer = await init()
</script>
```

## examples

> **NOTE:** cured output will ALWAYS be in lowercase.

### JavaScript

```js
const noCancer = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')

console.log(noCancer) // 'very funny text'
```

### Rust

```rust
extern crate decancer;
use decancer::Decancer;

fn main() {
  let instance = Decancer::new();
  let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  assert_eq!(output, String::from("very funny text"));
}
```

If you want to check if the decancered string contains a certain keyword, i recommend using this instead since mistranslations can happen (e.g mistaking the number 0 with the letter O)

### JavaScript

```js
const noCancer = decancer(someString)

if (decancer.contains(noCancer, 'no-no-word')) console.log('LANGUAGE!!!')
```

### Rust

```rust
extern crate decancer;
use decancer::Decancer;

fn main() {
  let instance = Decancer::new();
  let output = instance.cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  
  if instance.contains(&output, "funny") {
    println!("i found the funny");
  }
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
      import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.4.2/decancer.min.js"
      
      const decancer = await init()
      
      window.cure = function () {
        const textarea = document.querySelector("textarea")
        
        if (!textarea.value.length) {
          return alert("There's no text!!!")
        }
        
        textarea.value = decancer(textarea.value)
      }
    </script>
  </body>
</html>
```

## contributions

All contributions are welcome. Feel free to fork the project at GitHub! &lt3

If you want to add, remove, modify, or view the list of supported confusables, you can clone the [GitHub repository](https://github.com/null8626/decancer), and modify it directly with Node.js. Either through a script or directly from the REPL.

```js
const reader = await import('./contrib/index.mjs')
const data = reader.default('./core/bin/confusables.bin')

// do something with data...

data.save('./core/bin/confusables.bin')
```

## special thanks

These are the primary resources that made this project possible.

- [The Official Unicode Confusables List](https://util.unicode.org/UnicodeJsps/confusables.jsp)
- [The Official Unicode Characters List](https://unicode.org/Public/UNIDATA/UnicodeData.txt)
- [Wikipedia's list of Unicode Characters](https://en.wikipedia.org/wiki/List_of_Unicode_characters)
- [Fancy Text Generator](https://lingojam.com/FancyTextGenerator)
- [Unicode character inspector](https://apps.timwhitlock.info/unicode/inspect)
- [`napi-rs` for integrating Rust into the Node.js ecosystem](https://napi.rs/)
- [`wasm-bindgen` for making the development of WebAssembly modules in Rust easier](https://github.com/rustwasm/wasm-bindgen)
