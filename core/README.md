# decancer [![npm][npm-image]][npm-url] [![crates.io][crates-io-image]][crates-io-url] [![npm downloads][downloads-image]][downloads-url] [![crates.io downloads][crates-io-downloads-image]][crates-io-url] [![code style: prettier][prettier-image]][prettier-url] [![Build Status][ci-image]][ci-url] [![license][github-license-image]][github-license-url] [![BLAZINGLY FAST!!!][blazingly-fast-image]][blazingly-fast-url]

[crates-io-image]: https://img.shields.io/crates/v/decancer?style=flat-square
[crates-io-downloads-image]: https://img.shields.io/crates/d/decancer?style=flat-square
[crates-io-url]: https://crates.io/crates/decancer
[npm-image]: https://img.shields.io/npm/v/decancer.svg?style=flat-square
[npm-url]: https://npmjs.org/package/decancer
[downloads-image]: https://img.shields.io/npm/dt/decancer.svg?style=flat-square
[downloads-url]: https://npmjs.org/package/decancer
[prettier-image]: https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square
[prettier-url]: https://github.com/prettier/prettier
[ci-image]: https://github.com/null8626/decancer/workflows/CI/badge.svg
[ci-url]: https://github.com/null8626/decancer/actions/workflows/CI.yml
[github-license-image]: https://img.shields.io/github/license/null8626/decancer?style=flat-square
[github-license-url]: https://github.com/null8626/decancer/blob/main/LICENSE
[blazingly-fast-image]: https://img.shields.io/badge/speed-BLAZINGLY%20FAST!!!%20%F0%9F%94%A5%F0%9F%9A%80%F0%9F%92%AA%F0%9F%98%8E-brightgreen.svg?style=flat-square
[blazingly-fast-url]: https://twitter.com/acdlite/status/974390255393505280

[crates-io-image]: https://img.shields.io/crates/v/decancer?style=flat-square
[crates-io-downloads-image]: https://img.shields.io/crates/d/decancer?style=flat-square
[crates-io-url]: https://crates.io/crates/decancer
[npm-image]: https://img.shields.io/npm/v/decancer.svg?style=flat-square
[npm-url]: https://npmjs.org/package/decancer
[downloads-image]: https://img.shields.io/npm/dt/decancer.svg?style=flat-square
[downloads-url]: https://npmjs.org/package/decancer
[prettier-image]: https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square
[prettier-url]: https://github.com/prettier/prettier
[ci-image]: https://github.com/null8626/decancer/workflows/CI/badge.svg
[ci-url]: https://github.com/null8626/decancer/actions/workflows/CI.yml
[github-license-image]: https://img.shields.io/github/license/null8626/decancer?style=flat-square
[github-license-url]: https://github.com/null8626/decancer/blob/main/LICENSE
[blazingly-fast-image]: https://img.shields.io/badge/speed-BLAZINGLY%20FAST!!!%20%F0%9F%94%A5%F0%9F%9A%80%F0%9F%92%AA%F0%9F%98%8E-brightgreen.svg?style=flat-square
[blazingly-fast-url]: https://twitter.com/acdlite/status/974390255393505280

A tiny package that removes common unicode confusables/homoglyphs from strings.

- It's core is written in [Rust](https://www.rust-lang.org) and utilizes a form of **Binary Search** to ensure speed!
- It virtually has **no third-party dependencies** - it only depends on itself.
- It stores it's huge collection of codepoints in an [optimized 25.72 KB binary file](https://github.com/null8626/decancer/blob/main/core/bin/codepoints.bin) instead of a huge JSON or text file to optimize it's bundle size!
- It's capable of filtering **149,434 (13.41%) different unicode codepoints** including **9,628 different confusables**, like:
  - All [whitespace characters](https://en.wikipedia.org/wiki/Whitespace_character)
  - All [diacritics](https://en.wikipedia.org/wiki/Diacritic), this also eliminates all forms of [Zalgo text](https://en.wikipedia.org/wiki/Zalgo_text)
  - Most [homoglyphs](https://en.wikipedia.org/wiki/Homoglyph)
  - Most characters from foreign scripts, like [Chinese](https://en.wikipedia.org/wiki/Chinese_characters), [Cyrillic](https://en.wikipedia.org/wiki/Cyrillic_script), [Devanagari](https://en.wikipedia.org/wiki/Devanagari), [Greek](https://en.wikipedia.org/wiki/Greek_alphabet), [Hiragana](https://en.wikipedia.org/wiki/Hiragana), [Hangul](https://en.wikipedia.org/wiki/Hangul), [Kanji](https://en.wikipedia.org/wiki/Kanji), [Katakana](https://en.wikipedia.org/wiki/Katakana), etc.
  - Several emojis
- And it's available in the following languages:
  - [Rust](https://crates.io/crates/decancer)
  - JavaScript ([Node.js/Deno/Bun](https://www.npmjs.com/package/decancer)/Browser)
  - C/C++
  - [Python](https://pypi.org/project/decancer-py) (unofficial)

## Installation

<details>
<summary><b>Rust (v1.64 or later)</b></summary>

In your `Cargo.toml`:

```toml
decancer = "1.6.2"
```

</details>
<details>
<summary><b>JavaScript (Node.js v16 or later)</b></summary>

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
<summary><b>JavaScript (Deno v1.28 or later)</b></summary>

In your code:

```ts
import decancer from 'npm:decancer'
```

</details>
<details>
<summary><b>JavaScript (Bun)</b></summary>

In your shell:

```console
$ bun install decancer
```

In your code:

```js
const decancer = require('decancer')
```

</details>
<details>
<summary><b>JavaScript (Browser)</b></summary>

In your code:

```html
<script type="module">
  import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v1.6.2/bindings/wasm/bin/decancer.min.js'

  const decancer = await init()
</script>
```

</details>
<details>
<summary><b>C/C++</b></summary>

### Download precompiled binaries

- [Download for 64-bit Windows MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-x86_64-pc-windows-msvc.zip)
- [Download for 32-bit Windows MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-i686-pc-windows-msvc.zip)
- [Download for ARM64 Windows MSVC](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-aarch64-pc-windows-msvc.zip)
- [Download for 64-bit macOS (10.7+, Lion+)](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-x86_64-apple-darwin.zip)
- [Download for ARM64 macOS (11.0+, Big Sur+)](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-aarch64-apple-darwin.zip)
- [Download for 64-bit Linux (kernel 3.2+, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-x86_64-unknown-linux-gnu.zip)
- [Download for 64-bit Linux with MUSL](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-x86_64-unknown-linux-musl.zip)
- [Download for ARM64 Linux (kernel 4.1, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-aarch64-unknown-linux-gnu.zip)
- [Download for ARM64 Linux with MUSL](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-aarch64-unknown-linux-musl.zip)
- [Download for ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-armv7-unknown-linux-gnueabihf.zip)
- [Download for 64-bit FreeBSD](https://github.com/null8626/decancer/releases/download/v1.6.2/decancer-freebsd.zip)

### Building from source

Prerequisites:

- [Git](https://git-scm.com/)
- [Rust v1.64 or later](https://rustup.rs/)

```console
$ git clone https://github.com/null8626/decancer.git --depth 1
$ cd decancer/bindings/native
$ cargo build --release
```

And the binary files should be generated in the `target/release` directory.

</details>

## Examples

> **note:** cured output will always be in lowercase.

<details>
<summary><b>Rust</b></summary>

```rust
let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

// cured here is a decancer::CuredString struct wrapping over the cured string
// for comparison purposes, it's more recommended to use the methods provided by the decancer::CuredString struct.

assert_eq!(cured, "very funny text");
assert!(cured.starts_with("very"));
assert!(cured.contains("funny"));
assert!(cured.ends_with("text"));

let _output_str = cured.into_str(); // retrieve the String inside and consume the struct.
```

</details>
<details>
<summary><b>JavaScript (Node.js/Deno/Bun)</b></summary>

```js
const cured = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')

// cured here is a CuredString object wrapping over the cured string
// for comparison purposes, it's more recommended to use the methods provided by the CuredString class.

if (cured.contains('funny')) {
  console.log('found the funny')
}

if (
  cured.equals('very funny text') &&
  cured.startsWith('very') &&
  cured.endsWith('text')
) {
  console.log('it works!')
}

console.log(cured.toString())
// => 'very funny text'
```

</details>
<details>
<summary><b>JavaScript (Browser)</b></summary>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
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
      import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v1.6.2/bindings/wasm/bin/decancer.min.js'

      const decancer = await init()

      window.cure = function () {
        const textarea = document.querySelector('textarea')

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
<summary><b>C/C++</b></summary>

> **note:** `decancer.h` can be [retrieved](https://github.com/null8626/decancer/blob/main/bindings/native/decancer.h) from the `bindings/native` directory.

```c
#include <decancer.h>

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

// global variable for assertion purposes only
decancer_cured_t cured;

// our quick assert function
static void assert(const bool expr, const char *message)
{
    if (!expr)
    {
        fprintf(stderr, "assertion failed (%s)\n", message);
        decancer_free(cured);
        
        exit(1);
    }
}

int main(void) {
    // utf-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
    uint8_t string[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
                        0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
                        0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};

    cured = decancer_cure(string, sizeof(string));

    // comparisons
    assert(decancer_equals(cured, (uint8_t *)("very funny text"), 15), "equals");
    assert(decancer_starts_with(cured, (uint8_t *)("very"), 4), "starts_with");
    assert(decancer_ends_with(cured, (uint8_t *)("text"), 4), "ends_with");
    assert(decancer_contains(cured, (uint8_t *)("funny"), 5), "contains");

    // coerce output as a raw UTF-8 pointer and retrieve it's size (in bytes)
    size_t output_size;
    const uint8_t *output_raw = decancer_raw(cured, &output_size);

    // assert raw cured utf-8 size to be 15 bytes (size of "very funny text")
    assert(output_size == 15, "raw output size");

    // utf-8 bytes for "very funny text"
    const uint8_t expected_raw[] = {0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e,
                                    0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74};

    char assert_message[38];
    for (uint32_t i = 0; i < sizeof(expected_raw); i++)
    {
        sprintf(assert_message, "mismatched utf-8 contents at index %u", i);
        assert(output_raw[i] == expected_raw[i], assert_message);
    }

    // free cured string (required)
    decancer_free(cured);
    
    return 0;
}
```

</details>

## Contributing

Please read [`CONTRIBUTING.md`](https://github.com/null8626/decancer/blob/main/CONTRIBUTING.md) for newbie contributors who want to contribute!