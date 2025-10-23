# decancer [![npm][npm-image]][npm-url] [![crates.io][crates-io-image]][crates-io-url] [![npm downloads][npm-downloads-image]][npm-url] [![crates.io downloads][crates-io-downloads-image]][crates-io-url] [![codacy][codacy-image]][codacy-url] [![ko-fi][ko-fi-brief-image]][ko-fi-url]

[crates-io-url]: https://crates.io/crates/decancer
[crates-io-image]: https://img.shields.io/crates/v/decancer?style=flat-square
[crates-io-downloads-image]: https://img.shields.io/crates/d/decancer?style=flat-squar
[npm-url]: https://npmjs.org/package/decancer
[npm-image]: https://img.shields.io/npm/v/decancer.svg?style=flat-square
[npm-downloads-image]: https://img.shields.io/npm/dt/decancer.svg?style=flat-square
[codacy-url]: https://app.codacy.com/gh/null8626/decancer/dashboard
[codacy-image]: https://app.codacy.com/project/badge/Grade/d740b1aa867d42f2b37eb992ad73784a
[ko-fi-url]: https://ko-fi.com/null8626
[ko-fi-image]: https://ko-fi.com/img/githubbutton_sm.svg
[ko-fi-brief-image]: https://img.shields.io/badge/donations-ko--fi-red?color=ff5e5b&style=flat-square

A library that removes common unicode confusables/homoglyphs from strings.

- Its core is written in [Rust](https://www.rust-lang.org) and utilizes a form of [**Binary Search**](https://en.wikipedia.org/wiki/Binary_search_algorithm) to ensure speed!
- By default, it's capable of filtering **222,557 (19.98%) different unicode codepoints** like:
  - All [whitespace characters](https://en.wikipedia.org/wiki/Whitespace_character)
  - All [diacritics](https://en.wikipedia.org/wiki/Diacritic), this also eliminates all forms of [Zalgo text](https://en.wikipedia.org/wiki/Zalgo_text)
  - Most [leetspeak characters](https://en.wikipedia.org/wiki/Leet)
  - Most [homoglyphs](https://en.wikipedia.org/wiki/Homoglyph)
  - Several emojis
- Unlike other packages, this package is **[unicode bidi-aware](https://en.wikipedia.org/wiki/Bidirectional_text)** where it also interprets right-to-left characters in the same way as it were to be rendered by an application!
- Its behavior is also highly customizable to your liking!
<!---[ begin DECANCER_GLOBAL ]--->
- And it's available in the following languages:
  - [Rust](https://crates.io/crates/decancer)
  - JavaScript ([Node.js](https://www.npmjs.com/package/decancer)/Browser)
  - [Java](https://central.sonatype.com/artifact/io.github.null8626/decancer/overview)
  - C/C++
  - Go
  - [Python](https://pypi.org/project/decancer-py) (unofficial)
<!---[ end ]--->

## Installation

<!---[ begin DECANCER_GLOBAL ]--->
<details>
<summary><b>Rust (v1.65 or later)</b></summary>
<!---[ end, begin DECANCER_RUST ]--->

In your `Cargo.toml`:

```toml
decancer = "3.3.3"
```

<!---[ end, begin DECANCER_GLOBAL ]--->
</details>
<details>
<summary><b>JavaScript (Node.js)</b></summary>
<!---[ end, begin DECANCER_JS ]--->

In your shell:

```console
npm install decancer
```

In your code (CommonJS):

```js
const decancer = require('decancer')
```

In your code (ESM):

```js
import decancer from 'decancer'
```

<!---[ end, begin DECANCER_GLOBAL ]--->
</details>
<details>
<summary><b>JavaScript (Browser)</b></summary>

In your code:

```html
<script type="module">
  import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v3.3.3/bindings/wasm/bin/decancer.min.js'

  const decancer = await init()
</script>
```

</details>
<details>
<summary><b>Java</b></summary>

### As a JAR file

[You can download the latest JAR file here.](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer.jar)

### As a dependency

In your `build.gradle`:

```gradle
repositories {
  mavenCentral()
  maven { url 'https://jitpack.io' }
}

dependencies {
  implementation 'io.github.null8626:decancer:3.3.3'
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
    <groupId>io.github.null8626</groupId>
    <artifactId>decancer</artifactId>
    <version>3.3.3</version>
  </dependency>
</dependencies>
```

### Building from source

Windows:

```bat
> git clone https://github.com/null8626/decancer.git --branch v3.3.3 --depth 1
> cd decancer/bindings/java
> powershell -NoLogo -NoProfile -NonInteractive -Command "Expand-Archive -Path .\bin\bindings.zip -DestinationPath .\bin -Force"
> gradle build -x test
```

macOS/Linux:

```console
git clone https://github.com/null8626/decancer.git --branch v3.3.3 --depth 1
cd decancer/bindings/java
unzip ./bin/bindings.zip -d ./bin
chmod +x ./gradlew
./gradlew build -x test
```

Tip: You can shrink the size of the resulting JAR file by removing binaries in the `bin` directory for the platforms you don't want to support.

</details>
<details>
<summary><b>C/C++</b></summary>
<!---[ end, begin DECANCER_NATIVE ]--->

### Download

- [Header file](https://raw.githubusercontent.com/null8626/decancer/v3.3.3/bindings/native/decancer.h)
- [Download for ARM64 macOS (11.0+, Big Sur+)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-aarch64-apple-darwin.zip)
- [Download for ARM64 iOS](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-aarch64-apple-ios.zip)
- [Download for Apple iOS Simulator on ARM6](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-aarch64-apple-ios-sim.zip)
- [Download for ARM64 Android](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-aarch64-linux-android.zip)
- [Download for ARM64 Windows MSVC](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-aarch64-pc-windows-msvc.zip)
- [Download for ARM64 Linux (kernel 4.1, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-aarch64-unknown-linux-gnu.zip)
- [Download for ARM64 Linux with MUSL](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-aarch64-unknown-linux-musl.zip)
- [Download for ARMv6 Linux (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-arm-unknown-linux-gnueabi.zip)
- [Download for ARMv5TE Linux (kernel 4.4, glibc 2.23)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-armv5te-unknown-linux-gnueabi.zip)
- [Download for ARMv7-A Android](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-armv7-linux-androideabi.zip)
- [Download for ARMv7-A Linux (kernel 4.15, glibc 2.27)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-armv7-unknown-linux-gnueabi.zip)
- [Download for ARMv7-A Linux, hardfloat (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-armv7-unknown-linux-gnueabihf.zip)
- [Download for 32-bit Linux w/o SSE (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-i586-unknown-linux-gnu.zip)
- [Download for 32-bit MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-i686-pc-windows-msvc.zip)
- [Download for 32-bit FreeBSD](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-i686-unknown-freebsd.zip)
- [Download for 32-bit Linux (kernel 3.2+, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-i686-unknown-linux-gnu.zip)
- [Download for PPC64LE Linux (kernel 3.10, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-powerpc64le-unknown-linux-gnu.zip)
- [Download for RISC-V Linux (kernel 4.20, glibc 2.29)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-riscv64gc-unknown-linux-gnu.zip)
- [Download for S390x Linux (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-s390x-unknown-linux-gnu.zip)
- [Download for SPARC Solaris 11, illumos](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-sparcv9-sun-solaris.zip)
- [Download for Thumb2-mode ARMv7-A Linux with NEON (kernel 4.4, glibc 2.23)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-thumbv7neon-unknown-linux-gnueabihf.zip)
- [Download for 64-bit macOS (10.12+, Sierra+)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-x86_64-apple-darwin.zip)
- [Download for 64-bit iOS](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-x86_64-apple-ios.zip)
- [Download for 64-bit MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-x86_64-pc-windows-msvc.zip)
- [Download for 64-bit FreeBSD](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-x86_64-unknown-freebsd.zip)
- [Download for 64-bit illumos](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-x86_64-unknown-illumos.zip)
- [Download for 64-bit Linux (kernel 3.2+, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-x86_64-unknown-linux-gnu.zip)
- [Download for 64-bit Linux with MUSL](https://github.com/null8626/decancer/releases/download/v3.3.3/decancer-x86_64-unknown-linux-musl.zip)

### Building from source

Building from source requires [Rust v1.65 or later](https://rustup.rs/).

```console
git clone https://github.com/null8626/decancer.git --branch v3.3.3 --depth 1
cd decancer/bindings/native
cargo build --release
```

And the binary files should be generated in the `target/release` directory.

<!---[ end, begin DECANCER_GLOBAL ]--->
</details>
<details>
<summary><b>Go (v1.17 or later)</b></summary>
<!---[ end, begin DECANCER_GO ]--->

Building requires Rust v1.65 or later. Windows systems also require a MinGW compiler to be readily available.

In your shell:

```console
git clone https://github.com/null8626/decancer.git --branch v3.3.3 --depth 1
cd decancer/bindings/go
sudo -E "PATH=$PATH" go generate
go install
```

For most platforms, `go generate` will require elevated administrator permissions as decancer's native binding will be added to your system's libraries for convenience.

<!---[ end, begin DECANCER_GLOBAL ]--->
</details>
<!---[ end ]--->

## Examples

<!---[ begin DECANCER_GLOBAL ]--->
<details>
<summary><b>Rust</b></summary>
<!---[ end, begin DECANCER_RUST ]--->

For more information, please read the [documentation](https://docs.rs/decancer).

```rust
let mut cured = decancer::cure!(r"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣 wWiIiIIttHh l133t5p3/-\|<").unwrap();

assert_eq!(cured, "very funny text with leetspeak");

// WARNING: it's NOT recommended to coerce this output to a Rust string
//          and process it manually from there, as decancer has its own
//          custom comparison measures, including leetspeak matching!
assert_ne!(cured.as_str(), "very funny text with leetspeak");

assert!(cured.contains("funny"));

cured.censor("funny", '*');
assert_eq!(cured, "very ***** text with leetspeak");

cured.censor_multiple(["very", "text"], '-');
assert_eq!(cured, "---- ***** ---- with leetspeak");
```

<!---[ end, begin DECANCER_GLOBAL ]--->
</details>
<details>
<summary><b>JavaScript (Node.js)</b></summary>
<!---[ end, begin DECANCER_JS ]--->

```js
const assert = require('assert')
const cured = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣 wWiIiIIttHh l133t5p3/-\\|<')

assert(cured.equals('very funny text with leetspeak'))

// WARNING: it's NOT recommended to coerce this output to a JavaScript string
//          and process it manually from there, as decancer has its own
//          custom comparison measures, including leetspeak matching!
assert(cured.toString() !== 'very funny text with leetspeak')
console.log(cured.toString())
// => very funny text wwiiiiitthh l133t5p3/-\|<

assert(cured.contains('funny'))

cured.censor('funny', '*')
console.log(cured.toString())
// => very ***** text wwiiiiitthh l133t5p3/-\|<

cured.censorMultiple(['very', 'text'], '-')
console.log(cured.toString())
// => ---- ***** ---- wwiiiiitthh l133t5p3/-\|<
```

<!---[ end, begin DECANCER_GLOBAL ]--->
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
      import init from 'https://cdn.jsdelivr.net/gh/null8626/decancer@v3.3.3/bindings/wasm/bin/decancer.min.js'
    
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

[See this in action here.](https://null8626.github.io/decancer/wasm_example)

</details>
<details>
<summary><b>Java</b></summary>

For more information, please read the [documentation](https://javadoc.io/doc/io.github.null8626/decancer).

```java
import io.github.null8626.decancer.CuredString;

public class Program {
  public static void main(String[] args) {
    try (final CuredString cured = new CuredString("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣 wWiIiIIttHh l133t5p3/-\\|<")) {
      assert cured.equals("very funny text with leetspeak");
      
      // WARNING: it's NOT recommended to coerce this output to a Java String
      //          and process it manually from there, as decancer has its own
      //          custom comparison measures, including leetspeak matching!
      assert !cured.toString().equals("very funny text with leetspeak");
      System.out.println(cured.toString());
      // => very funny text wwiiiiitthh l133t5p3/-\|<
      
      assert cured.contains("funny");
      
      cured.censor("funny", '*');
      System.out.println(cured.toString());
      // => very ***** text wwiiiiitthh l133t5p3/-\|<
      
      String[] keywords = { "very", "text" };
      cured.censorMultiple(keywords, '-');
      System.out.println(cured.toString());
      // => ---- ***** ---- wwiiiiitthh l133t5p3/-\|<
    }
  }
}
```

</details>
<details>
<summary><b>C/C++</b></summary>
<!---[ end, begin DECANCER_NATIVE ]--->

For more information, please read the [documentation](https://null8626.github.io/decancer/native_docs).

UTF-8 example:

```c
#include <decancer.h>

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#define decancer_assert(expr, notes)                           \
  if (!(expr)) {                                               \
    fprintf(stderr, "assertion failure at " notes "\n");       \
    ret = 1;                                                   \
    goto END;                                                  \
  }

int main() {
  int ret = 0;

  // UTF-8 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
  uint8_t input[] = {0x76, 0xef, 0xbc, 0xa5, 0xe2, 0x93, 0xa1, 0xf0, 0x9d, 0x94, 0x82, 0x20, 0xf0, 0x9d,
                     0x94, 0xbd, 0xf0, 0x9d, 0x95, 0x8c, 0xc5, 0x87, 0xe2, 0x84, 0x95, 0xef, 0xbd, 0x99,
                     0x20, 0xc5, 0xa3, 0xe4, 0xb9, 0x87, 0xf0, 0x9d, 0x95, 0x8f, 0xf0, 0x9d, 0x93, 0xa3};

  decancer_error_t error;
  decancer_cured_t cured = decancer_cure(input, sizeof(input), DECANCER_OPTION_DEFAULT, &error);

  if (cured == NULL) {
    fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
    return 1;
  }

  decancer_assert(decancer_contains(cured, "funny", 5), "decancer_contains");

END:
  decancer_cured_free(cured);
  return ret;
}
```

UTF-16 example:

```c
#include <decancer.h>

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#define decancer_assert(expr, notes)                           \
  if (!(expr)) {                                               \
    fprintf(stderr, "assertion failure at " notes "\n");       \
    ret = 1;                                                   \
    goto END;                                                  \
  }

int main() {
  int ret = 0;

  // UTF-16 bytes for "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"
  uint16_t input[] = {
    0x0076, 0xff25, 0x24e1,
    0xd835, 0xdd02, 0x0020,
    0xd835, 0xdd3d, 0xd835,
    0xdd4c, 0x0147, 0x2115,
    0xff59, 0x0020, 0x0163,
    0x4e47, 0xd835, 0xdd4f,
    0xd835, 0xdce3
  };

  // UTF-16 bytes for "funny"
  uint16_t funny[] = { 0x66, 0x75, 0x6e, 0x6e, 0x79 };

  decancer_error_t error;
  decancer_cured_t cured = decancer_cure_utf16(input, sizeof(input) / sizeof(uint16_t), DECANCER_OPTION_DEFAULT, &error);

  if (cured == NULL) {
    fprintf(stderr, "curing error: %.*s\n", (int)error.message_length, error.message);
    return 1;
  }

  decancer_assert(decancer_contains_utf16(cured, funny, sizeof(funny) / sizeof(uint16_t)), "decancer_contains_utf16");

END:
  decancer_cured_free(cured);
  return ret;
}
```

<!---[ end, begin DECANCER_GLOBAL ]--->
</details>
<details>
<summary><b>Go</b></summary>
<!---[ end, begin DECANCER_GO ]--->

```go
package main

import (
  "os"
  "fmt"
  "strconv"
  "github.com/null8626/decancer/bindings/go"
)

func main() {
  cured, err := decancer.Cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣", decancer.Default)

  if err != nil {
    fmt.Fprintln(os.Stderr, "error:", err)
    os.Exit(1)
  }

  defer cured.Close()

  fmt.Println(cured.String())

  if cured.Equals("very funny text") {
    fmt.Println("it is indeed a very funny text")
  }

  if cured.StartsWith("very") {
    fmt.Println("it starts with 'very'")
  }
  
  if cured.EndsWith("text") {
    fmt.Println("it ends with 'text'")
  }

  if cured.Contains("funny") {
    fmt.Println("it has the funny")
  }

  funnyMatches := cured.Find("funny")

  fmt.Println("funny counter:")

  for i, match := range funnyMatches {
    fmt.Println("Match " + strconv.Itoa(i) + ":")
    fmt.Println("  - start: " + strconv.Itoa(match.Start))
    fmt.Println("  - end: " + strconv.Itoa(match.End))
  }

  keywords := []string{"very", "funny"}
  veryFunnyMatches, err := cured.FindMultiple(keywords)

  if err != nil {
    fmt.Fprintln(os.Stderr, "error:", err)
    os.Exit(1)
  }

  fmt.Println("very funny counter:")

  for i, match := range veryFunnyMatches {
    fmt.Println("Match " + strconv.Itoa(i) + ":")
    fmt.Println("  - start: " + strconv.Itoa(match.Start))
    fmt.Println("  - end: " + strconv.Itoa(match.End))
  }
}
```

<!---[ end, begin DECANCER_GLOBAL ]--->
</details>
<!---[ end ]--->

## Donations

If you want to support my eyes for manually looking at thousands of unicode characters, consider donating! ❤

[![ko-fi][ko-fi-image]][ko-fi-url]

<!---[ begin DECANCER_GLOBAL ]--->

## Contributing

Please read [`CONTRIBUTING.md`](https://github.com/null8626/decancer/blob/main/CONTRIBUTING.md) for newbie contributors who want to contribute!