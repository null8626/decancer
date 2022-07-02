# decancer

A portable JavaScript module that removes common confusables from strings written in pure [Rust](https://rust-lang.org). Available for Node.js, Deno, and the Browser.

Pros:

- Extremely fast, no use of regex whatsoever!
- No dependencies.
- Simple to use, just one single function.
- Supports all the way to UTF-32 code-points. Like emojis, zalgos, etc.
- While this project may not be perfect, it should cover the vast majority of confusables.

Con:

- Remember that this project is not perfect, false-positives may happen.

**As of version 1.3.3, This package supports 2,526 different code-points.**

## installation

### Node.js (npm)

In your shell:

```console
$ npm install decancer
```

In your code:

```js
const decancer = require('decancer');
```

### Node.js (`.node` binary)

`.node` binaries can be retrieved from the [GitHub Releases page](https://github.com/null8626/decancer/releases).

> **IMPORTANT NOTE:** The usage differs a bit from the others because `decancer` here is a standalone function.

```js
const { decancer, contains } = require('/path/to/decancer.node');
```

### Deno

In your code:

```ts
import init from "https://deno.land/x/decancer@v1.3.3/mod.ts";

const decancer = await init();
```

### Browser

In your code:

```js
import init from "https://cdn.jsdelivr.net/gh/null8626/decancer@v1.3.3/decancer.min.js";

const decancer = await init();
```

## example

```js
const noCancer = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣');

console.log(noCancer); // 'very funny text'
```

> **NOTE:** output will ALWAYS be in lowercase.

If you want to check if the decancered string contains a certain keyword, i recommend using this instead of using `.includes`, since mistranslations can happen (e.g mistaking the number 0 with the letter O)

```js
const noCancer = decancer(someString);

if (decancer.contains(noCancer, 'no-no-word')) console.log('LANGUAGE!!!');
```

## contributions

All contributions are welcome. Feel free to fork the project at GitHub! &lt;3

If you want to add, remove, modify, or view the list of supported confusables, you can clone the [GitHub repository](https://github.com/null8626/decancer), and open the Node.js REPL:

```js
const reader = await import('./contrib/index.mjs');
const data = reader.default('./core/bin/confusables.bin');
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