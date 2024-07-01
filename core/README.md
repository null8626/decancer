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
In your `Cargo.toml`:

```toml
decancer = "3.2.3"
```
## Examples
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
## Donations

If you want to support my eyes for manually looking at thousands of unicode characters, consider donating! ❤

[![ko-fi][ko-fi-image]][ko-fi-url]
