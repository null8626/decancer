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
Building in Windows requires MinGW to be installed.

In your shell:

```console
$ git clone https://github.com/null8626/decancer.git
$ cd decancer
$ git checkout v3.3.3
$ cd bindings/go
$ sudo -E "PATH=$PATH" go generate
$ go install
```

For most platforms, `go generate` will require elevated administrator permissions as decancer's native binding will be added to system libraries.
## Examples
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
## Donations

If you want to support my eyes for manually looking at thousands of unicode characters, consider donating! ❤

[![ko-fi][ko-fi-image]][ko-fi-url]
