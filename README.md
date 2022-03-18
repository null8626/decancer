# decancer
A Node.js module that removes common confusables from strings written in pure [Rust](https://rust-lang.org).

- Extremely fast, no use of regex whatsoever!
- No dependencies.
- Simple to use, just one single function.
- Supports UTF-8 for basic confusables.
- Supports UTF-16 for the majority of confusables, this includes zalgos, foreign languages, etc.
- Supports UTF-32 for emojis and 'fancy fonts'.
- While this project may not be perfect, it should cover the vast majority of confusables.

__**As of version 1.1.6, This library supports 2,486 different UTF-16 code-points.**__

# installation
Install cross-platform release with npm: (Highly recommended)
```bash
$ npm install decancer
```

Or if you're targeting a specific platform, You can use these links to go to each platform-specific release.

- [Windows x64](https://www.npmjs.com/package/@vierofernando/decancer-win32-x64-msvc)
- [Windows arm64](https://www.npmjs.com/package/@vierofernando/decancer-win32-arm64-msvc)
- [Windows i686](https://www.npmjs.com/package/@vierofernando/decancer-win32-ia32-msvc)
- [macOS x64](https://www.npmjs.com/package/@vierofernando/decancer-darwin-x64)
- [macOS arm64](https://www.npmjs.com/package/@vierofernando/decancer-darwin-arm64)
- [Linux x64 GNU](https://www.npmjs.com/package/@vierofernando/decancer-linux-x64-gnu)
- [Linux x64 MUSL](https://www.npmjs.com/package/@vierofernando/decancer-linux-x64-musl)
- [Linux arm64 GNU](https://www.npmjs.com/package/@vierofernando/decancer-linux-arm64-gnu)
- [Linux arm64 MUSL](https://www.npmjs.com/package/@vierofernando/decancer-linux-arm64-musl)
- [Linux arm gnueabihf](https://www.npmjs.com/package/@vierofernando/decancer-linux-arm-gnueabihf)
- [Android arm64](https://www.npmjs.com/package/@vierofernando/decancer-android-arm64)

# example
```js
const decancer = require('decancer');

const noCancer = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣');
console.log(noCancer); // 'very funny text'
```
> **NOTE:** output will ALWAYS be in lowercase and invalid UTF-16 code-points will be replaced by a replacement character (\uFFFD or �).

# contributions
All contributions are welcome. If you want to, you can [make a fork here at GitHub.](https://github.com/vierofernando/decancer/fork) Thanks! &lt;3

# special thanks
These are the primary resources that made this project possible.

- [The Official Unicode Confusables List](https://util.unicode.org/UnicodeJsps/confusables.jsp)
- [Fancy Text Generator](https://lingojam.com/FancyTextGenerator)
- [Unicode character inspector](https://apps.timwhitlock.info/unicode/inspect)