# Contributing guide

If you want to contribute to the source code, feel free to do so! ❤️<br>
Don't forget to state on what should be changed and their reason.

If you want to see/modify on the confusables supported and/or their respective translation(s), feel free to do so! Here are the ways on how to do them;

- **To convert the binary into a readable and modifiable JSON, do the following:**

```console
node scripts/decode.mjs core/bin/confusables.bin
```

- **Or if you want a simplified and unoptimized version without ranges and all of that nonsenese, run:**

```console
node scripts/decode.mjs core/bin/confusables.bin --full
```

And a file called `output.json` should be generated in the same directory.

- **And to validate, optimize, and convert the JSON back into a binary, do the following:**

```console
node scripts/encode.mjs path/to/file.json
```

And a file called `output.bin` should be generated in the same directory, and an optimized version of the JSON file should appear at `path/to/fileOptimized.json`.

> P.S: for further optimizations purposes, it's recommended to do a second run of the encoding process, e.g `node scripts/encode.mjs path/to/fileOptimized.json`

- **The structure of the JSON is as follows:**

The optional fields here are only optional when writing and encoding them back into a binary.

```ts
interface Confusable {
  codepoint: number
  translation: string
  rangeUntil?: number | null
  syncedTranslation?: boolean
}

interface JsonContents {
  confusables: Confusable[]
  similar: string[][]
}
```

- **Information regarding the `Confusable` structure:**

  - The `JsonContents#confusables` array **must NOT be empty.** _(duh)_
  - `Confusable#codepoint` is the Unicode codepoint of the desired confusable. It must be in the unicode range, and must NOT be an [ASCII character](https://en.wikipedia.org/wiki/ASCII), [control character](https://en.wikipedia.org/wiki/Control_character), [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates), [combining character](https://en.wikipedia.org/wiki/Script_(Unicode)#Special_script_property_values), or [private use character](https://en.wikipedia.org/wiki/Private_Use_Areas), [byte order character](https://en.wikipedia.org/wiki/Byte_order_mark).
  - `Confusable#translation` is the translation string, it's length must not exceed `15`, it must be in lowercase, and it must be in ASCII.
  - `Confusable#rangeUntil` is an optional number that indicates where the range of this confusable should end. If it's `null`, then the confusable is not a range. For example: Say a confusable with the codepoints of `\xE0` to `\xE5` all translates to `a`, then the `codepoint` field would be `0x00EO` and the `rangeUntil` field would be `0x00E5`. **Please note that the range size MUST be around `1` to `127`.**
  - `Confusable#syncedTranslation` is a flag whether this range's translation would change accordingly with the codepoint index. For example: `\xE0` translates to `a`, `\xE1` translates to `b`, `\xE2` translates to `c`, and so on (the translation property's length must be `1`).

- **Information regarding the `JsonContents#similar` field:**

  - The `string[][]` two-dimensional array **must NOT be empty** and it's length **must NOT exceed `127`.**
  - The `string[]` arrays **must NOT be empty** and their lengths **must NOT exceed `255`.**
  - Each `string` **must ONLY be only one character in length.**
  - Each `string` **must ONLY be in the ASCII range.**

## Other scripts

Other useful scripts have been added as a utility for the maintenance of `decancer`.

### Pretty source code files

Prerequisites:

- [Clang](https://clang.llvm.org)
- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org)

```console
$ node scripts/pretty.mjs
```

### Bump version number

```console
$ node scripts/version.mjs 1.2.3
```