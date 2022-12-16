# decancer

A portable module that removes common confusables from strings without the use of Regexes. Available for Rust, Node.js, Deno, and the Browser.

Pros:

- BLAZINGLY FAST™, no use of regex whatsoever!
- No use of any external dependencies.
- Simple to use, just one single function.
- Supports more than **2000 unicode codepoints**. This should cover the vast majority of confusables. Including emojis, zalgos, etc.

Con:

- Remember that this project is not perfect, false-positives may happen.

## installation

### Rust

In your `Cargo.toml`:

```toml
decancer = "1.5.0"
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

fn main() {
  let output = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  assert_eq!(output, "very funny text");

  // decancer::cure returns a CuredString struct. To coerce it to a String, use .into_str().
  let _output_str = output.into_str();
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

fn main() {
  let output = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  
  if output.contains("funny") { // TODO!
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

## special thanks

These are the primary resources that made this project possible.

- [The Official Unicode Confusables List](https://util.unicode.org/UnicodeJsps/confusables.jsp)
- [The Official Unicode Characters List](https://unicode.org/Public/UNIDATA/UnicodeData.txt)
- [Wikipedia's list of Unicode Characters](https://en.wikipedia.org/wiki/List_of_Unicode_characters)
- [Fancy Text Generator](https://lingojam.com/FancyTextGenerator)
- [Unicode character inspector](https://apps.timwhitlock.info/unicode/inspect)
- [`napi-rs` for integrating Rust into the Node.js ecosystem](https://napi.rs/)
- [`wasm-bindgen` for making the development of WebAssembly modules in Rust easier](https://github.com/rustwasm/wasm-bindgen)
