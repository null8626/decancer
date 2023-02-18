# decancer [![npm][npm-image]][npm-url] [![downloads][downloads-image]][downloads-url]

[npm-image]: https://img.shields.io/npm/v/decancer.svg
[npm-url]: https://npmjs.org/package/decancer
[downloads-image]: https://img.shields.io/npm/dm/decancer.svg
[downloads-url]: https://npmjs.org/package/decancer

A tiny package that removes common confusables from strings.

- It's core is written in [Rust](https://www.rust-lang.org) and utilizes a form of **Binary Search** to ensure speed!
- It virtually has **no third-party dependencies** - it only depends on itself.
- It stores it's huge collection of confusables in a [customized binary file](https://github.com/null8626/decancer/blob/main/core/bin/confusables.bin) instead of a huge JSON or text file to optimize it's bundle size!
- It supports curing **5,417 different confusables** into cured-lowercased-strings, including:
  - Accented characters
  - [Most homoglyphs](https://en.wikipedia.org/wiki/Homoglyph)
  - Several foreign characters, including [Arabic](https://en.wikipedia.org/wiki/Arabic), [Chinese](https://en.wikipedia.org/wiki/Chinese_characters), [Cyrillic](https://en.wikipedia.org/wiki/Cyrillic_script), [Greek](https://en.wikipedia.org/wiki/Greek_alphabet), [Japanese](https://en.wikipedia.org/wiki/Kanji), [Korean](https://en.wikipedia.org/wiki/Hangul), etc.
  - Several emojis
  - [Whitespace characters](https://en.wikipedia.org/wiki/Whitespace_character)
  - [Zalgo text](https://en.wikipedia.org/wiki/Zalgo_text)
- And it's supported in the following languages:
  - [Rust](https://crates.io/crates/decancer)
  - JavaScript ([Node.js/Deno/Bun](https://www.npmjs.com/package/decancer)/Browser)
  - C/C++
  - [Python](https://pypi.org/project/decancer-py) (unofficial)

## Installation

<details>
<summary>Rust</summary>

In your `Cargo.toml`:

```toml
decancer = "1.5.4"
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
import decancer from 'npm:decancer'
```

</details>
<details>
<summary>Bun</summary>

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
<summary>Browser</summary>

In your code:

```html
<script type="module">
  import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v1.5.4/bindings/wasm/bin/decancer.min.js'

  const decancer = await init()
</script>
```

</details>
<details>
<summary>C/C++</summary>

### Download precompiled binaries

- [Download for 64-bit Windows MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-x86_64-pc-windows-msvc.zip)
- [Download for 32-bit Windows MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-i686-pc-windows-msvc.zip)
- [Download for ARM64 Windows MSVC](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-aarch64-pc-windows-msvc.zip)
- [Download for 64-bit macOS (10.7+, Lion+)](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-x86_64-apple-darwin.zip)
- [Download for ARM64 macOS (11.0+, Big Sur+)](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-aarch64-apple-darwin.zip)
- [Download for 64-bit Linux (kernel 3.2+, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-x86_64-unknown-linux-gnu.zip)
- [Download for 64-bit Linux with MUSL](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-x86_64-unknown-linux-musl.zip)
- [Download for ARM64 Linux (kernel 4.1, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-aarch64-unknown-linux-gnu.zip)
- [Download for ARM64 Linux with MUSL](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-aarch64-unknown-linux-musl.zip)
- [Download for ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-armv7-unknown-linux-gnueabihf.zip)
- [Download for 64-bit FreeBSD](https://github.com/null8626/decancer/releases/download/v1.5.4/decancer-freebsd.zip)

### Building from source

Prerequisites:

- [Git](https://git-scm.com/)
- [Rust](https://rustup.rs/)

```console
$ git clone https://github.com/null8626/decancer.git --depth 1
$ cd decancer/bindings/native
$ cargo build --release
```

And the binary files should be generated in the `target/release` directory.

</details>

## Examples

<details>
<summary>JavaScript (Node.js/Deno/Bun)</summary>

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

console.log(cured.toString()) // 'very funny text'
```

</details>
<details>
<summary>JavaScript (Browser)</summary>

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
      import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v1.5.4/bindings/wasm/bin/decancer.min.js'

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
<summary>Rust</summary>

```rust
extern crate decancer;

fn main() {
  let cured_e = decancer::cure_char('Ｅ');
  
  match cured_e {
    decancer::Translation::Character(e) => assert_eq!(e, 'e'),
    _ => unreachable!(),
  }
  
  let cured_ae = decancer::cure_char('ӕ');
  
  match cured_ae {
    decancer::Translation::String(ae) => assert_eq!(ae, "ae"),
    _ => unreachable!(),
  }
  
  // control characters
  let cured_nothing = decancer::cure_char('\0'); 
  
  assert!(matches!(cured_nothing, decancer::Translation::None));

  let cured = decancer::cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  // cured here is a decancer::CuredString struct wrapping over the cured string
  // for comparison purposes, it's more recommended to use the methods provided by the decancer::CuredString struct.

  assert_eq!(cured, "very funny text");
  assert!(cured.starts_with("very"));
  assert!(cured.contains("funny"));
  assert!(cured.ends_with("text"));

  let _output_str = cured.into_str(); // retrieve the String inside and consume the struct.
}
```

</details>
<details>
<summary>C/C++</summary>

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
    decancer_translation_t char_translation;

    // cure the unicode character 'Ｅ' (U+FF25)
    decancer_cure_char(0xFF25, &char_translation);
    
    assert(char_translation.kind == DECANCER_TRANSLATION_KIND_CHARACTER, "char translation is a character");
    assert(char_translation.contents.character == 0x65, "char translation is 'e' (0x65)");

    // cure the unicode character 'ӕ' (U+04D5)
    decancer_cure_char(0x04D5, &char_translation);
    
    assert(char_translation.kind == DECANCER_TRANSLATION_KIND_STRING, "char translation is an ASCII string");
    assert(char_translation.contents.string.length == 2,
           "char translation is an ASCII string with the length of 2 bytes");
    assert(char_translation.contents.string.contents[0] == 'a' && char_translation.contents.string.contents[1] == 'e',
           "char translation is the ASCII string \"ae\".");

    // try to cure the null terminator (\0)
    decancer_cure_char(0, &char_translation);
    
    assert(char_translation.kind == DECANCER_TRANSLATION_KIND_NONE, "char translation is an empty string ('')");

    // utf-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
    uint8_t string[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
                        0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
                        0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};

    // cure string
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

Please [read `CONTRIBUTING.md`](https://github.com/null8626/decancer/blob/main/CONTRIBUTING.md) for newbie contributors who want to contribute!