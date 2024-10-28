# decancer
A Node.js module that removes common confusables from strings written in pure [Rust](https://rust-lang.org).

Pros:

- Extremely fast, no use of regex whatsoever!
- No dependencies.
- Simple to use, just one single function.
- Supports UTF-8 for basic confusables.
- Supports UTF-16 for the majority of confusables, this includes zalgos, foreign languages, etc.
- Supports UTF-32 for emojis and 'fancy fonts'.
- While this project may not be perfect, it should cover the vast majority of confusables.

Con:

- Remember that this project is not perfect, false-positives may happen.

__**As of version 1.2.3, This library supports 2,578 different UTF-16 code-points.**__

# installation
```bash
$ npm install decancer
```

Supported platforms:
- Windows x64
- Windows arm64
- Windows i686
- macOS x64
- macOS arm64
- Linux x64 GNU
- Linux x64 MUSL
- Linux arm64 GNU
- Linux arm64 MUSL
- Linux arm gnueabihf
- Android arm64

# example
```js
const decancer = require('decancer');
const noCancer = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣');

console.log(noCancer); // 'very funny text'
```
> **NOTE:** output will ALWAYS be in lowercase.

If you want to check if the decancered string contains a certain keyword, i recommend using this instead of using `.includes`, since mistranslations can happen (e.g mistaking the number 0 with the letter O)

```js
const decancer = require('decancer');
const noCancer = decancer(someString);

if (decancer.contains(noCancer, 'no-no-word')) {
  console.log('LANGUAGE!!!');
}
```

# contributions
All contributions are welcome. If you want to, you can [make a fork here at GitHub.](https://github.com/vierofernando/decancer/fork) Thanks! &lt;3

# special thanks
These are the primary resources that made this project possible.

- [The Official Unicode Confusables List](https://util.unicode.org/UnicodeJsps/confusables.jsp)
- [The Official Unicode Characters List](https://unicode.org/Public/UNIDATA/UnicodeData.txt)
- [Wikipedia's list of Unicode Characters](https://en.wikipedia.org/wiki/List_of_Unicode_characters)
- [Fancy Text Generator](https://lingojam.com/FancyTextGenerator)
- [Unicode character inspector](https://apps.timwhitlock.info/unicode/inspect)
- [napi-rs for integrating Rust into the Node.js ecosystem](https://napi.rs/)