# Contributing guide

If you want to contribute to the source code, feel free to do so! ❤️

Don't forget to state what should be changed and their respective reasons.

If you want to see/modify on the codepoints supported and/or their respective translation(s), feel free to do so! Here are the ways on how to do them;

- **To convert the binary into a readable text file, do the following:**

```console
node scripts/read.mjs path/to/output.txt
```

- **To convert the binary into a readable and modifiable JSON, do the following:**

```console
node scripts/read.mjs [path/to/output.json]
```

If the output file name is not supplied, it will default to `output.json`.

- **And to validate, optimize, and convert the JSON back into a binary, do the following:**

```console
node scripts/write.mjs path/to/input.json
```

- **The structure of the JSON is as follows:**

The optional fields here are only optional when writing and encoding them back into a binary.

```ts
interface Codepoint {
  codepoint: number
  translation: string
}

interface JsonContents {
  codepoints: Codepoint[]
  similar: string[][]
}
```

- **Information regarding the `Codepoint` structure:**

  - The `JsonContents#codepoints` array **must NOT be empty.**
  - `Codepoint#codepoint` is the unicode codepoint. It must be around `\u80` to `\ue00ff` and must NOT be a [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates).
  - `Codepoint#translation` is the translation string: its length must not exceed `15`, it must be in lowercase, and it must be in ASCII.

- **Information regarding the `JsonContents#similar` field:**

  - The `string[][]` two-dimensional array **must NOT be empty** and its length **must NOT exceed `127`.**
  - The `string[]` arrays **must NOT be empty** and their lengths **must NOT exceed `255`.**
  - Each `string` **must ONLY be one character long.**
  - Each `string` **must be ASCII.**

## Other scripts

Other useful scripts have been added as a utility for maintaining `decancer`.

### Update README files for bindings

Run this every time you modify the root README file.

```console
node scripts/readme.mjs
```

### Pretty source code files

Prerequisites:

- [clang-format v18 or later](https://clang.llvm.org)
- [Rust v1.65 or later](https://www.rust-lang.org)

```console
node scripts/pretty.mjs
```

### Bump version number

```console
node scripts/version.mjs 1.2.3
```

### Update cache if a new unicode version is released

```console
node scripts/update_unicode.mjs
```
