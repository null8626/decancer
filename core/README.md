# decancer [![npm][npm-image]][npm-url] [![crates.io][crates-io-image]][crates-io-url] [![jitpack.io][jitpack-io-image]][jitpack-io-url] [![npm downloads][npm-downloads-image]][npm-downloads-url] [![crates.io downloads][crates-io-downloads-image]][crates-io-url]

[crates-io-image]: https://img.shields.io/crates/v/decancer?style=flat-square
[crates-io-downloads-image]: https://img.shields.io/crates/d/decancer?style=flat-square
[crates-io-url]: https://crates.io/crates/decancer
[npm-image]: https://img.shields.io/npm/v/decancer.svg?style=flat-square
[npm-url]: https://npmjs.org/package/decancer
[jitpack-io-image]: https://jitpack.io/v/null8626/decancer.svg
[jitpack-io-url]: https://jitpack.io/#null8626/decancer
[npm-downloads-image]: https://img.shields.io/npm/dt/decancer.svg?style=flat-square
[npm-downloads-url]: https://npmjs.org/package/decancer

A tiny package that removes common unicode confusables/homoglyphs from strings.

- Its core is written in [Rust](https://www.rust-lang.org) and utilizes a form of **Binary Search** to ensure speed!
- By default, it's capable of filtering **215.356 (19.33%) different unicode codepoints** like:
  - All [whitespace characters](https://en.wikipedia.org/wiki/Whitespace_character)
  - All [diacritics](https://en.wikipedia.org/wiki/Diacritic), this also eliminates all forms of [Zalgo text](https://en.wikipedia.org/wiki/Zalgo_text)
  - Most [leetspeak characters](https://en.wikipedia.org/wiki/Leet)
  - Most [homoglyphs](https://en.wikipedia.org/wiki/Homoglyph)
  - Several emojis
- Unlike other packages, this package is **[unicode bidi-aware](https://en.wikipedia.org/wiki/Bidirectional_text)** in a way that it also interprets right-to-left characters in the same way as it were to be rendered by an application.
- Its behavior is also highly customizable to your liking!
- And it's available in the following languages:
  - [Rust](https://crates.io/crates/decancer)
  - JavaScript ([Node.js](https://www.npmjs.com/package/decancer)/Browser)
  - C/C++
  - Java
  - [Python](https://pypi.org/project/decancer-py) (unofficial)

## Installation

<details>
<summary><b>Rust (v1.64 or later)</b></summary>

In your `Cargo.toml`:

```toml
decancer = "3.0.2"
```

</details>
<details>
<summary><b>JavaScript (Node.js)</b></summary>

In your shell:

```console
$ npm install decancer
```

In your code (CommonJS):

```js
const decancer = require('decancer')
```

In your code (ESM):

```js
import decancer from 'decancer'
```

</details>
<details>
<summary><b>JavaScript (Browser)</b></summary>

In your code:

```html
<script type="module">
  import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v3.0.2/bindings/wasm/bin/decancer.min.js'

  const decancer = await init()
</script>
```

</details>
<details>
<summary><b>Java</b></summary>

### As a dependency

In your `build.gradle`:

```gradle
repositories {
  mavenCentral()
  maven { url 'https://jitpack.io' }
}

dependencies {
  implementation 'com.github.null8626:decancer:v3.0.2'
}
```

In your `pom.xml`:

```xml
<repositories>
  <repository>
    <id>central</id>
    <url>https://repo.maven.apache.org/maven2</url>
  </repository>
  <repository>
    <id>jitpack.io</id>
    <url>https://jitpack.io</url>
  </repository>
</repositories>

<dependencies>
  <dependency>
    <groupId>com.github.null8626</groupId>
    <artifactId>decancer</artifactId>
    <version>v3.0.2</version>
  </dependency>
</dependencies>
```

### Building from source

```console
$ git clone https://github.com/null8626/decancer.git --depth 1
$ cd ./decancer/bindings/java
$ unzip ./bin/bindings.zip -d ./bin
$ chmod +x ./gradlew
$ ./gradlew build --warning-mode all
```
</details>
<details>
<summary><b>C/C++</b></summary>

### Download

- [Download for ARM64 macOS (11.0+, Big Sur+)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-aarch64-apple-darwin.zip)
- [Download for ARM64 iOS](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-aarch64-apple-ios.zip)
- [Download for Apple iOS Simulator on ARM6](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-aarch64-apple-ios-sim.zip)
- [Download for ARM64 Android](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-aarch64-linux-android.zip)
- [Download for ARM64 Windows MSVC](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-aarch64-pc-windows-msvc.zip)
- [Download for ARM64 Linux (kernel 4.1, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-aarch64-unknown-linux-gnu.zip)
- [Download for ARM64 Linux with MUSL](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-aarch64-unknown-linux-musl.zip)
- [Download for ARMv6 Linux (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-arm-unknown-linux-gnueabi.zip)
- [Download for ARMv5TE Linux (kernel 4.4, glibc 2.23)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-armv5te-unknown-linux-gnueabi.zip)
- [Download for ARMv7-A Android](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-armv7-linux-androideabi.zip)
- [Download for ARMv7-A Linux (kernel 4.15, glibc 2.27)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-armv7-unknown-linux-gnueabi.zip)
- [Download for ARMv7-A Linux, hardfloat (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-armv7-unknown-linux-gnueabihf.zip)
- [Download for 32-bit Linux w/o SSE (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-i586-unknown-linux-gnu.zip)
- [Download for 32-bit MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-i686-pc-windows-msvc.zip)
- [Download for 32-bit FreeBSD](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-i686-unknown-freebsd.zip)
- [Download for 32-bit Linux (kernel 3.2+, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-i686-unknown-linux-gnu.zip)
- [Download for PPC64LE Linux (kernel 3.10, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-powerpc64le-unknown-linux-gnu.zip)
- [Download for RISC-V Linux (kernel 4.20, glibc 2.29)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-riscv64gc-unknown-linux-gnu.zip)
- [Download for S390x Linux (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-s390x-unknown-linux-gnu.zip)
- [Download for SPARC Solaris 11, illumos](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-sparcv9-sun-solaris.zip)
- [Download for Thumb2-mode ARMv7-A Linux with NEON (kernel 4.4, glibc 2.23)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-thumbv7neon-unknown-linux-gnueabihf.zip)
- [Download for 64-bit macOS (10.12+, Sierra+)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-x86_64-apple-darwin.zip)
- [Download for 64-bit iOS](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-x86_64-apple-ios.zip)
- [Download for 64-bit MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-x86_64-pc-windows-msvc.zip)
- [Download for 64-bit FreeBSD](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-x86_64-unknown-freebsd.zip)
- [Download for 64-bit illumos](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-x86_64-unknown-illumos.zip)
- [Download for 64-bit Linux (kernel 3.2+, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-x86_64-unknown-linux-gnu.zip)
- [Download for 64-bit Linux with MUSL](https://github.com/null8626/decancer/releases/download/v3.0.2/decancer-x86_64-unknown-linux-musl.zip)

### Building from source

Building from source requires [Rust v1.64 or later](https://rustup.rs/).

```console
$ git clone https://github.com/null8626/decancer.git --depth 1
$ cd decancer/bindings/native
$ cargo build --release
```

And the binary files should be generated in the `target/release` directory.

</details>

## Examples

<details>
<summary><b>Rust</b></summary>

For more information, please read the [documentation](https://docs.rs/decancer).

```rust
let cured = decancer::cure!("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣").unwrap();

assert_eq!(cured, "very funny text");
assert!(cured.contains("FuNny"));
assert_eq!(cured.into_str(), String::from("very funny text"));
```

</details>
<details>
<summary><b>JavaScript (Node.js)</b></summary>

```js
const assert = require('assert')
const cured = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')

assert(cured.equals('very funny text'))
assert(cured.contains('funny'))

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
      import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v3.0.2/bindings/wasm/bin/decancer.min.js'

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

[See this in action here.](https://null8626.github.io/decancer)

</details>
<details>
<summary><b>Java</b></summary>

```java
import com.github.null8626.decancer.CuredString;

public class Program {
  public static void main(String[] args) {
    final CuredString cured = new CuredString("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
    
    assert cured.equals("very funny text");
    assert cured.contains("funny");
    
    System.out.println(cured.toString());
    
    cured.destroy();
  }
}
```

</details>
<details>
<summary><b>C/C++</b></summary>

UTF-8 example:

```c
#include <decancer.h>

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

// global variable for assertion purposes only
decancer_cured_t cured;

static void assert(const bool expr, const char *message)
{
    if (!expr)
    {
        fprintf(stderr, "assertion failed (%s)\n", message);
        decancer_cured_free(cured);
        
        exit(1);
    }
}

static void print_error(decancer_error_t *error)
{
    char message[90];
    uint8_t message_size;
    
    memcpy(message, error->message, error->message_size);
   
    // rust strings are NOT null-terminated
    message[error->message_size] = '\0';
    
    fprintf(stderr, "error: %s", message);
}

int main(void) {
    decancer_error_t error;

    // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
    uint8_t string[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
                        0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
                        0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};

    cured = decancer_cure(string, sizeof(string), DECANCER_OPTION_DEFAULT, &error);

    if (cured == NULL)
    {
        print_error(&error);
        return 1;
    }

    assert(decancer_equals(cured, (uint8_t *)("very funny text"), 15), "equals");
    assert(decancer_contains(cured, (uint8_t *)("funny"), 5), "contains");

    // coerce output as a raw UTF-8 pointer and retrieve its size (in bytes)
    size_t output_size;
    const uint8_t *output_raw = decancer_raw(cured, &output_size);

    assert(output_size == 15, "raw output size");

    // UTF-8 bytes for "very funny text"
    const uint8_t expected_raw[] = {0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e,
                                    0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74};

    char assert_message[38];
    for (uint32_t i = 0; i < sizeof(expected_raw); i++)
    {
        sprintf(assert_message, "mismatched utf-8 contents at index %u", i);
        assert(output_raw[i] == expected_raw[i], assert_message);
    }

    decancer_cured_free(cured);    
    return 0;
}
```

UTF-16 example:

```c
#include <decancer.h>

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

// global variable for assertion purposes only
decancer_cured_t cured;
decancer_raw_wide_t wide = NULL;

static void assert(const bool expr, const char *message)
{
    if (!expr)
    {
        fprintf(stderr, "assertion failed (%s)\n", message);
        
        if (wide != NULL)
        {
            decancer_raw_wide_free(wide);
        }
        
        decancer_cured_free(cured);
        
        exit(1);
    }
}

static void print_error(decancer_error_t *error)
{
    char message[90];
    uint8_t message_size;
    
    memcpy(message, error->message, error->message_size);
   
    // rust strings are NOT null-terminated
    message[error->message_size] = '\0';
    
    fprintf(stderr, "error: %s", message);
}

int main(void) {
    decancer_error_t error;

    // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
    uint16_t string[] = {
        0x0076, 0xff25, 0x24e1,
        0xd835, 0xdd02, 0x0020,
        0xd835, 0xdd3d, 0xd835,
        0xdd4c, 0x0147, 0x2115,
        0xff59, 0x0020, 0x0163,
        0x4e47, 0xd835, 0xdd4f,
        0xd835, 0xdce3
    };

    cured = decancer_cure_wide(string, sizeof(string), DECANCER_OPTION_DEFAULT, &error);

    if (cured == NULL)
    {
        print_error(&error);
        return 1;
    }

    assert(decancer_equals(cured, (uint8_t *)("very funny text"), 15), "equals");
    assert(decancer_contains(cured, (uint8_t *)("funny"), 5), "contains");

    // coerce output as a raw UTF-16 pointer and retrieve its size (in bytes)
    uint16_t *output_ptr;
    size_t utf16_output_size;
    wide = decancer_raw_wide(cured, &output_ptr, &utf16_output_size);

    assert(utf16_output_size == (15 * sizeof(uint16_t)), "raw output size");

    // UTF-16 bytes for "very funny text"
    const uint16_t expected_raw[] = {0x76, 0x65, 0x72, 0x79, 0x20, 0x66, 0x75, 0x6e,
                                     0x6e, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74};

    char assert_message[39];
    for (uint32_t i = 0; i < sizeof(expected_raw) / sizeof(uint16_t); i++)
    {
        sprintf(assert_message, "mismatched utf-16 contents at index %u", i);
        assert(output_raw[i] == expected_raw[i], assert_message);
    }

    decancer_raw_wide_free(wide);
    decancer_cured_free(cured);    
    return 0;
}
```

</details>

## Compatibility


Decancer is supported in the following platforms:

Platform name | C/C++/Rust | Java | JavaScript
---- | ---- | ---- | ----
ARM64 macOS (11.0+, Big Sur+) | ✓ | ✓ | ✓ |
ARM64 iOS | ✓ | | |
Apple iOS Simulator on ARM6 | ✓ | | |
ARM64 Android | ✓ | | ✓ |
ARM64 Windows MSVC | ✓ | ✓ | ✓ |
ARM64 Linux (kernel 4.1, glibc 2.17+) | ✓ | ✓ | ✓ |
ARM64 Linux with MUSL | ✓ | ✓ | ✓ |
ARMv6 Linux (kernel 3.2, glibc 2.17) | ✓ | ✓ | |
ARMv5TE Linux (kernel 4.4, glibc 2.23) | ✓ | ✓ | |
ARMv7-A Android | ✓ | | ✓ |
ARMv7-A Linux (kernel 4.15, glibc 2.27) | ✓ | ✓ | |
ARMv7-A Linux, hardfloat (kernel 3.2, glibc 2.17) | ✓ | ✓ | ✓ |
32-bit Linux w/o SSE (kernel 3.2, glibc 2.17) | ✓ | | |
32-bit MSVC (Windows 7+) | ✓ | ✓ | ✓ |
32-bit FreeBSD | ✓ | ✓ | |
32-bit Linux (kernel 3.2+, glibc 2.17+) | ✓ | ✓ | |
PPC64LE Linux (kernel 3.10, glibc 2.17) | ✓ | | |
RISC-V Linux (kernel 4.20, glibc 2.29) | ✓ | ✓ | |
S390x Linux (kernel 3.2, glibc 2.17) | ✓ | | |
SPARC Solaris 11, illumos | ✓ | | |
Thumb2-mode ARMv7-A Linux with NEON (kernel 4.4, glibc 2.23) | ✓ | | |
64-bit macOS (10.12+, Sierra+) | ✓ | ✓ | ✓ |
64-bit iOS | ✓ | | |
64-bit MSVC (Windows 7+) | ✓ | ✓ | ✓ |
64-bit FreeBSD | ✓ | ✓ | |
64-bit illumos | ✓ | | |
64-bit Linux (kernel 3.2+, glibc 2.17+) | ✓ | ✓ | ✓ |
64-bit Linux with MUSL | ✓ | ✓ | ✓ |

## Contributing

Please read [`CONTRIBUTING.md`](https://github.com/null8626/decancer/blob/main/CONTRIBUTING.md) for newbie contributors who want to contribute!