# Contributing guide

If you want to contribute to the source code, feel free to do so! ❤️<br>
Don't forget to state on what should be changed and their reason.

If you want to see/modify on the confusables supported and/or their respective translation(s), feel free to do so! Here are the ways on how to do them;

- **To convert the binary *into* a readable and modifiable JSON, do the following:**

```shell
node scripts/decode.mjs core/bin/confusables.bin
```

And a file called `output.json` should be generated in the same directory.

- **And, to validate and retrieve the binary back *from* the JSON, do the following:**

```shell
node scripts/encode.mjs path/to/file.json
```

And a file called `output.bin` should be generated in the same directory, and a condensed, refactored JSON file should appear at `path/to/fileRefactored.json`.

- **The structure of the JSON is as follows:**

The optional fields here are only optional when writing and encoding them back into a binary.

```ts
interface Confusable {
  codepoint: number;
  translation: string;
  caseSensitive?: boolean;
  rangeUntil?: number | null;
  syncedTranslation?: boolean;
}

interface JsonContents {
  confusables: Confusable[];
  similar: string[][];
}
```

Information regarding the `Confusable` structure:

- `Confusable#codepoint` is the Unicode codepoint of the desired confusable. It must be in the unicode range (`0` to `0x10FFFF`)
- `Confusable#translation` is the translation string, it must NOT be empty.
- `Confusable#caseSensitive` is a flag whether this `Confusable#codepoint` is case-sensitive or not. In other words, whether the `codepoint` changes when it is called with `.toLowerCase()`.
- `Confusable#rangeUntil` is an optional number that indicates where the range of this confusable should end. If it's `null`, then the confusable is not a range. For example: Say a confusable with the codepoints of `\u00E0` to `\u00E5` all translates to `a`, then the `codepoint` field would be `0x00EO` and the `rangeUntil` field would be `0x00E5`. **Please note that the range size MUST be around `1` to `127`.**
- `Confusable#syncedTranslation` is a flag whether this range's translation would change accordingly with the codepoint index. For example: `\u00E0` translates to `a`, `\u00E1` translates to `b`, `\u00E2` translates to `c`, and so on.

Information regarding the `JsonContents#similar` field:

- The `string[]` array length **must NOT exceed `127`.**
- Each `string` in the `string[][]` array **must ONLY be only one character in length.**
- Each `string` in the `string[][]` array **must ONLY be in the UTF-1 character range. This means only `\x00` to `\xFF`.**