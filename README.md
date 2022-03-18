# decancer
A Node.js module that removes common confusables from strings written in pure [Rust](https://rust-lang.org) without the use of Regexes.

__**As of version 1.1.5, This library supports 2,487 different code-points.**__

# installation
Install with npm:
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
> **NOTE:** output will ALWAYS be in lowercase and invalid UTF-16 code-points will be replaced by a replacement character (\uFFFD or �).

# contributions
All contributions are welcome. If you want to, you can [make a fork here at GitHub.](https://github.com/vierofernando/decancer/fork) Thanks! &lt;3