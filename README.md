# decancer
Node.js module that decancers a string. Cleans everything back to lowercased, clean, alphanumeric state. Covers a large range of unicode groups, including:

- Zalgos
- Fancy characters & unicode "fonts"
- Emojified characters
- Foreign punctuations
- Cyrillic characters
- Greek characters
- Japanese characters
- Arabic characters
- And so much more.

[For more info on supported characters, see here.](#characters)

# installation
Install with npm:
```bash
$ npm install decancer
```

# examples
```js
const decancer = require('decancer');

const noCancer = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣');
console.log(noCancer); // 'very funny text'
```
> **NOTE:** all output will be in lowercase.

# characters
This library as of the latest version can decode up to **2,206** characters. Here are the details:

| Unicode Group | Amount |
|-----|-----|
| High Surrogates | 895 |
| Low Surrogates | 292 |
| Halfwidth and Fullwidth Forms | 101 |
| Phonetic Extensions | 99 |
| Latin Extended-B | 95 |
| Cyrillic | 94 |
| IPA Extensions | 91 |
| Latin Extended-A | 64 |
| Enclosed Alphanumerics | 62 |
| Letterlike Symbols | 59 |
| Latin-1 Supplement | 52 |
| Greek and Coptic | 52 |
| Superscripts and Subscripts | 44 |
| CJK Unified Ideographs | 34 |
| Cherokee Supplement | 31 |
| Lisu | 29 |
| Small Form Variants | 26 |
| Unified Canadian Aboriginal Syllabics | 25 |
| Basic Latin | 22 |
| Coptic | 18 |
| Spacing Modifier Letters | 16 |
| Arabic | 16 |
| General Punctuation | 14 |
| Armenian | 12 |
| Currency Symbols | 12 |
| Dingbats | 12 |
| Mathematical Operators | 10 |
| Arabic Presentation Forms-A | 10 |
| Latin Extended-D | 9 |
| Latin Extended-E | 9 |
| Runic | 7 |
| Number Forms | 7 |
| Tifinagh | 7 |
| Hebrew | 6 |
| Phonetic Extensions Supplement | 6 |
| CJK Symbols and Punctuation | 6 |
| Arabic Presentation Forms-B | 6 |
| Latin Extended-C | 5 |
| Katakana | 5 |
| Bopomofo | 5 |
| Cyrillic Supplement | 4 |
| Syriac | 4 |
| Oriya | 4 |
| Malayalam | 4 |
| Miscellaneous Technical | 4 |
| CJK Compatibility Forms | 4 |
| NKo | 3 |
| Devanagari | 3 |
| Bengali | 3 |
| Gurmukhi | 3 |
| Miscellaneous Mathematical Symbols-B | 3 |
| Bamum | 3 |
| Gujarati | 2 |
| Telugu | 2 |
| Kannada | 2 |
| Thai | 2 |
| Myanmar | 2 |
| Georgian | 2 |
| Hangul Jamo | 2 |
| Ethiopic | 2 |
| Cherokee | 2 |
| Mongolian | 2 |
| Latin Extended Additional | 2 |
| Box Drawing | 2 |
| Miscellaneous Mathematical Symbols-A | 2 |
| Supplemental Arrows-B | 2 |
| Kangxi Radicals | 2 |
| Hiragana | 2 |
| CJK Strokes | 2 |
| Cyrillic Extended-B | 2 |
| Tamil | 1 |
| Sinhala | 1 |
| Lao | 1 |
| Hanunoo | 1 |
| Combining Diacritical Marks for Symbols | 1 |
| Supplemental Mathematical Operators | 1 |
| Supplemental Punctuation | 1 |
| Vai | 1 |
| Variation Selectors | 1 |

# contributions
All contributions are welcome. If you want to, you can [make a fork here at GitHub.](https://github.com/vierofernando/decancer/fork) Thanks! &lt;3
