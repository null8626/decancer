/* eslint-disable */

'use strict'

import {
  binarySearchExists,
  containsInclusive,
  isCaseSensitive,
  mergeArray,
  removeFromSet
} from './util.mjs'
import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { execSync } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { deserialize } from 'node:v8'
import { inspect } from 'node:util'
import assert from 'node:assert'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const CACHE_FILE = join(ROOT_DIR, '.cache.bin')
const STRING_TRANSLATION_MASK = 0x10000000n

console.log('- fetching unicode data...')

if (!existsSync(CACHE_FILE)) {
  execSync(`node "${join(ROOT_DIR, 'scripts', 'update_unicode.mjs')}"`, {
    stdio: 'inherit'
  })
}

const { blocks, diacritics, expected, emojis } = deserialize(
  readFileSync(CACHE_FILE)
)

if (typeof process.argv[2] !== 'string') {
  console.error('error: missing json file path')
  process.exit(1)
}

const RETAINABLE_SCRIPTS = Object.entries({
  greek: {
    shift: 3,
    check: name => name.includes('greek') && !name.includes('ancient')
  },
  cyrillic: 4,
  hebrew: 5,
  arabic: 6,
  devanagari: 7,
  bengali: 8,
  armenian: 9,
  gujarati: 10,
  tamil: 11,
  thai: 12,
  lao: 13,
  burmese: {
    shift: 14,
    check: name => name.includes('myanmar')
  },
  khmer: 15,
  mongolian: 16,
  chinese: {
    shift: 17,
    check: name => name.includes('cjk') || name.includes('kangxi')
  },
  japanese: {
    shift: 18,
    check: name => name.includes('katakana') || name.includes('hiragana')
  },
  korean: {
    shift: 19,
    check: name => name.includes('hangul')
  },
  braille: 20
})

const TURKISH_CHARACTERS = ['ç', 'ğ', 'ı', 'ö', 'ş', 'ü'] // İ is omitted from here because it's lowercase form is just a normal i

function getAttributes(codepoint) {
  const { name } = blocks.find(({ start, end }) =>
    containsInclusive(codepoint, start, end)
  )

  const retainableScript = RETAINABLE_SCRIPTS.find(([n, data]) => {
    if (typeof data === 'number') {
      return name.includes(n)
    } else {
      return data.check(name)
    }
  })

  let retainableScriptShift = 0

  if (retainableScript) {
    retainableScriptShift = retainableScript[1].shift ?? retainableScript[1]
  } else if (binarySearchExists(emojis, codepoint)) {
    retainableScriptShift = 21
  }

  return (
    (retainableScriptShift << 2) |
    (Number(
      TURKISH_CHARACTERS.includes(
        String.fromCodePoint(codepoint).toLowerCase()
      ) || codepoint === 0x130
    ) <<
      1) |
    Number(binarySearchExists(diacritics, codepoint))
  )
}

const { codepoints, similar } = JSON.parse(readFileSync(process.argv[2]))

assert(
  Array.isArray(codepoints) && codepoints.length > 0,
  'codepoints must be an array'
)
assert(
  Array.isArray(similar) &&
    similar.length > 0 &&
    similar.every(
      x =>
        Array.isArray(x) &&
        x.length >= 2 &&
        x.every(
          y =>
            typeof y === 'string' && y.length === 1 && y.codePointAt() <= 0x7f
        )
    ),
  'similar must be an array of an array of ASCII strings'
)

console.log(
  `- checking, expanding, and sorting ${codepoints.length.toLocaleString(
    'en-US'
  )} codepoints...`
)

let expanded = []

for (const conf of codepoints) {
  if (!Number.isSafeInteger(conf.codepoint)) {
    console.warn(
      `- [warn] this codepoint is not a valid value and therefore ignored: ${conf.codepoint}`
    )
    continue
  } else if (
    typeof conf.translation !== 'string' ||
    ![...conf.translation].every(
      c => c.codePointAt() <= 0x7f && !isCaseSensitive(c.codePointAt())
    ) ||
    conf.translation.length > 0x1f
  ) {
    console.warn(
      `- [warn] translation is not a valid value and therefore ignored: '${conf.translation}'`
    )
    continue
  }

  if (conf.translation.length === 0) {
    conf.translation = '\0'
  } else if (!binarySearchExists(expected, conf.codepoint)) {
    console.warn(
      `- [warn] this codepoint is not allowed and therefore ignored: ${conf.codepoint}`
    )
    continue
  } else if (
    expanded.find(([codepoint]) => codepoint === conf.codepoint)?.[1] ===
    conf.translation
  ) {
    console.warn(`- [warn] this duplicate is ignored: ${conf.codepoint}`)
    continue
  }

  expanded.push([conf.codepoint, conf.translation])
}

console.log(
  `- expanded to a grand total of ${expanded.length.toLocaleString(
    'en-US'
  )} codepoints.\n- searching for collisions...`
)

{
  const set = Array.from(new Set(expanded.map(([codepoint]) => codepoint)))

  assert(
    expanded.length === set.length,
    `discovered ${(expanded.length - set.length).toLocaleString(
      'en-US'
    )} collisions. at codepoints: ${inspect(
      removeFromSet(
        expanded.map(([codepoint]) => codepoint),
        set
      ),
      {
        maxArrayLength: Infinity
      }
    )}`
  )
}

const caseSensitiveCollisions = []
let i = 0

while (i < expanded.length) {
  const [codepoint, translation] = expanded[i]

  if (isCaseSensitive(codepoint)) {
    const lowercasedCodepoint = String.fromCodePoint(codepoint)
      .toLowerCase()
      .codePointAt()

    if (
      expanded.find(
        ([codepoint2, translation2]) =>
          codepoint2 === lowercasedCodepoint && translation === translation2
      )
    ) {
      caseSensitiveCollisions.push(codepoint)
    }
  }

  i++
}

assert(
  caseSensitiveCollisions.length === 0,
  `discovered ${caseSensitiveCollisions.length.toLocaleString(
    'en-US'
  )} case-sensitive collisions. at codepoints: ${inspect(
    caseSensitiveCollisions,
    {
      maxArrayLength: Infinity
    }
  )}`
)

expanded = expanded.sort((a, b) => a.codepoint - b.codepoint)

let grandTotal = []

for (i = 0; i < expanded.length; i++) {
  const [codepoint, translation] = expanded[i]
  const caseSensitive = isCaseSensitive(codepoint)
  const attributes = getAttributes(codepoint)

  if (translation.length === 1 && grandTotal.length > 0) {
    const previous = grandTotal[grandTotal.length - 1]

    if (
      previous.rangeSize < 0x7f &&
      previous.translation.length === 1 &&
      previous.attributes === attributes &&
      previous.caseSensitive === caseSensitive &&
      previous.codepoint + previous.rangeSize + 1 === codepoint
    ) {
      const previousTranslationCharCode = previous.translation.charCodeAt()
      const currentTranslationCharCode = translation.charCodeAt()

      if (
        previous.rangeSize === 0 &&
        previousTranslationCharCode + 1 === currentTranslationCharCode
      ) {
        previous.syncedTranslation = true
        previous.rangeSize++
        continue
      } else if (
        (previous.syncedTranslation &&
          previousTranslationCharCode + previous.rangeSize + 1 ===
            currentTranslationCharCode) ||
        (!previous.syncedTranslation &&
          previousTranslationCharCode === currentTranslationCharCode)
      ) {
        previous.rangeSize++
        continue
      }
    }
  }

  grandTotal.push({
    caseSensitive,
    codepoint,
    translation,
    rangeSize: 0,
    syncedTranslation: false,
    attributes
  })
}

grandTotal = grandTotal.sort((a, b) => a.codepoint - b.codepoint)

console.log(
  `- condensed down from ${expanded.length.toLocaleString('en-US')} to ${grandTotal.length.toLocaleString('en-US')} (${(
    (grandTotal.length / expanded.length) *
    100
  ).toFixed(2)}%).`
)

const similarBytes = Buffer.from(
  similar.reduce(
    (a, b) => [
      ...a,
      ...b.slice(0, -1).map(x => x.charCodeAt()),
      b.at(-1).charCodeAt() | 0x80
    ],
    []
  )
)

const strings = mergeArray([
  ...new Set(
    grandTotal
      .map(({ translation }) => translation)
      .filter(translation => translation.length !== 1)
  )
])

const codepointsBuffers = []
const caseSensitiveCodepointsBuffers = []

for (const {
  caseSensitive,
  codepoint,
  translation,
  rangeSize,
  syncedTranslation,
  attributes
} of grandTotal) {
  const buf = Buffer.alloc(6)
  let firstBytes = BigInt(codepoint)
  let middleByte = 0

  if (translation.length > 1) {
    const offset = strings.indexOf(translation)

    firstBytes |=
      STRING_TRANSLATION_MASK |
      BigInt(((translation.length << 3) | (offset >> 8)) << 20)
    middleByte = offset & 0xff
  } else {
    if (syncedTranslation) middleByte = 0x80

    middleByte |= rangeSize
    firstBytes |= BigInt(translation.charCodeAt() << 20)
  }

  buf.writeUint32LE(Number(firstBytes))
  buf.writeUint8(middleByte, 4)
  buf.writeUint8(attributes, 5)

  if (caseSensitive) {
    caseSensitiveCodepointsBuffers.push(buf)
  } else {
    codepointsBuffers.push(buf)
  }
}

assert(
  strings.length <= 0x7ff,
  `strings size must not exceed ${0x7ff}. (got ${strings.length})`
)

const headers = Buffer.alloc(6)
headers.writeUint16LE(headers.byteLength + codepointsBuffers.length * 6)
headers.writeUint16LE(
  headers.readUint16LE() + caseSensitiveCodepointsBuffers.length * 6,
  2
)
headers.writeUint16LE(headers.readUint16LE(2) + similarBytes.length, 4)

writeFileSync(
  join(ROOT_DIR, 'core', 'bin', 'codepoints.bin'),
  Buffer.concat([
    headers,
    Buffer.concat(codepointsBuffers),
    Buffer.concat(caseSensitiveCodepointsBuffers),
    similarBytes,
    Buffer.from(strings)
  ])
)

console.log('- wrote to codepoints.bin.')
