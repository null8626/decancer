<!-- WARNING: this markdown file is computer generated.
     please modify the README.md file in the root directory instead. -->

# decancer [![npm][npm-image]][npm-url] [![crates.io][crates-io-image]][crates-io-url] [![npm downloads][npm-downloads-image]][npm-url] [![crates.io downloads][crates-io-downloads-image]][crates-io-url] [![codacy][codacy-image]][codacy-url] [![ko-fi][ko-fi-brief-image]][ko-fi-url]

[crates-io-image]: https://img.shields.io/crates/v/decancer?style=flat-square
[crates-io-downloads-image]: https://img.shields.io/crates/d/decancer?style=flat-square
[crates-io-url]: https://crates.io/crates/decancer
[npm-image]: https://img.shields.io/npm/v/decancer.svg?style=flat-square
[npm-url]: https://npmjs.org/package/decancer
[npm-downloads-image]: https://img.shields.io/npm/dt/decancer.svg?style=flat-square
[codacy-image]: https://app.codacy.com/project/badge/Grade/d740b1aa867d42f2b37eb992ad73784a
[codacy-url]: https://app.codacy.com/gh/null8626/decancer/dashboard
[ko-fi-brief-image]: https://img.shields.io/badge/donations-ko--fi-red?color=ff5e5b&style=flat-square
[ko-fi-image]: https://ko-fi.com/img/githubbutton_sm.svg
[ko-fi-url]: https://ko-fi.com/null8626

A library that removes common unicode confusables/homoglyphs from strings.

- Its core is written in [Rust](https://www.rust-lang.org) and utilizes a form of [**Binary Search**](https://en.wikipedia.org/wiki/Binary_search_algorithm) to ensure speed!
- By default, it's capable of filtering **221,529 (19.88%) different unicode codepoints** like:
  - All [whitespace characters](https://en.wikipedia.org/wiki/Whitespace_character)
  - All [diacritics](https://en.wikipedia.org/wiki/Diacritic), this also eliminates all forms of [Zalgo text](https://en.wikipedia.org/wiki/Zalgo_text)
  - Most [leetspeak characters](https://en.wikipedia.org/wiki/Leet)
  - Most [homoglyphs](https://en.wikipedia.org/wiki/Homoglyph)
  - Several emojis
- Unlike other packages, this package is **[unicode bidi-aware](https://en.wikipedia.org/wiki/Bidirectional_text)** where it also interprets right-to-left characters in the same way as it were to be rendered by an application!
- Its behavior is also highly customizable to your liking!

## Installation
### Download

- [Header file](https://raw.githubusercontent.com/null8626/decancer/v3.2.2/bindings/native/decancer.h)
- [Download for ARM64 macOS (11.0+, Big Sur+)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-aarch64-apple-darwin.zip)
- [Download for ARM64 iOS](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-aarch64-apple-ios.zip)
- [Download for Apple iOS Simulator on ARM6](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-aarch64-apple-ios-sim.zip)
- [Download for ARM64 Android](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-aarch64-linux-android.zip)
- [Download for ARM64 Windows MSVC](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-aarch64-pc-windows-msvc.zip)
- [Download for ARM64 Linux (kernel 4.1, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-aarch64-unknown-linux-gnu.zip)
- [Download for ARM64 Linux with MUSL](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-aarch64-unknown-linux-musl.zip)
- [Download for ARMv6 Linux (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-arm-unknown-linux-gnueabi.zip)
- [Download for ARMv5TE Linux (kernel 4.4, glibc 2.23)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-armv5te-unknown-linux-gnueabi.zip)
- [Download for ARMv7-A Android](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-armv7-linux-androideabi.zip)
- [Download for ARMv7-A Linux (kernel 4.15, glibc 2.27)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-armv7-unknown-linux-gnueabi.zip)
- [Download for ARMv7-A Linux, hardfloat (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-armv7-unknown-linux-gnueabihf.zip)
- [Download for 32-bit Linux w/o SSE (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-i586-unknown-linux-gnu.zip)
- [Download for 32-bit MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-i686-pc-windows-msvc.zip)
- [Download for 32-bit FreeBSD](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-i686-unknown-freebsd.zip)
- [Download for 32-bit Linux (kernel 3.2+, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-i686-unknown-linux-gnu.zip)
- [Download for PPC64LE Linux (kernel 3.10, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-powerpc64le-unknown-linux-gnu.zip)
- [Download for RISC-V Linux (kernel 4.20, glibc 2.29)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-riscv64gc-unknown-linux-gnu.zip)
- [Download for S390x Linux (kernel 3.2, glibc 2.17)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-s390x-unknown-linux-gnu.zip)
- [Download for SPARC Solaris 11, illumos](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-sparcv9-sun-solaris.zip)
- [Download for Thumb2-mode ARMv7-A Linux with NEON (kernel 4.4, glibc 2.23)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-thumbv7neon-unknown-linux-gnueabihf.zip)
- [Download for 64-bit macOS (10.12+, Sierra+)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-x86_64-apple-darwin.zip)
- [Download for 64-bit iOS](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-x86_64-apple-ios.zip)
- [Download for 64-bit MSVC (Windows 7+)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-x86_64-pc-windows-msvc.zip)
- [Download for 64-bit FreeBSD](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-x86_64-unknown-freebsd.zip)
- [Download for 64-bit illumos](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-x86_64-unknown-illumos.zip)
- [Download for 64-bit Linux (kernel 3.2+, glibc 2.17+)](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-x86_64-unknown-linux-gnu.zip)
- [Download for 64-bit Linux with MUSL](https://github.com/null8626/decancer/releases/download/v3.2.2/decancer-x86_64-unknown-linux-musl.zip)

### Building from source

Building from source requires [Rust v1.65 or later](https://rustup.rs/).

```sh
git clone https://github.com/null8626/decancer.git --depth 1
cd decancer/bindings/native
cargo build --release
```

And the binary files should be generated in the `target/release` directory.
## Examples
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

int main(void) {
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

int main(void) {
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
## Donations

If you want to support my eyes for manually looking at thousands of unicode characters, consider donating! ❤

[![ko-fi][ko-fi-image]][ko-fi-url]
