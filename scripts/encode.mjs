import {
  binarySearchExists,
  isCaseSensitive,
  mergeArray,
  removeFromSet,
  SortedSet
} from './util.mjs'
import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { execSync } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { inspect } from 'node:util'
import assert from 'node:assert'

const RANGE_MASK = 0x8000000n
const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const STRING_TRANSLATION_MASK = 0x10000000n

console.log('- fetching unicode data...')

if (!existsSync(join(ROOT_DIR, '.expected.json'))) {
  execSync(`node ${join(ROOT_DIR, 'scripts', 'update_unicode.mjs')}`, {
    stdio: 'inherit'
  })
}

const EXPECTED = JSON.parse(readFileSync(join(ROOT_DIR, '.expected.json')))

if (typeof process.argv[2] !== 'string') {
  console.error('error: missing json file path.')
  process.exit(1)
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
  assert(
    Number.isSafeInteger(conf.codepoint),
    'codepoint must be a valid number'
  )
  assert(
    typeof conf.translation === 'string' &&
      [...conf.translation].every(
        c => c.codePointAt() <= 0x7f && !isCaseSensitive(c.codePointAt())
      ) &&
      conf.translation.length <= 0x1f,
    `translation must be a valid string: '${conf.translation}'`
  )

  if (conf.translation.length === 0) {
    assert(
      !conf.syncedTranslation,
      'syncedTranslation is not allowed in empty translations'
    )

    conf.translation = '\0'
  }

  if (typeof conf.rangeUntil === 'number') {
    assert(
      Number.isSafeInteger(conf.rangeUntil) &&
        conf.rangeUntil > conf.codepoint &&
        conf.rangeUntil < 0x110000 &&
        conf.rangeUntil - conf.codepoint <= 0x7f,
      'rangeUntil must be a valid number'
    )
    assert(
      conf.rangeUntil > conf.codepoint,
      `rangeUntil must be greater than codepoint. (rangeUntil: ${conf.rangeUntil}, codepoint: ${conf.codepoint})`
    )

    if (conf.syncedTranslation) {
      assert(
        conf.translation.length === 1,
        `translation length for codepoints with syncedTranslation must be one character in length, got '${conf.translation}'`
      )
    }

    const ogTranslationCode = conf.syncedTranslation
      ? conf.translation.charCodeAt()
      : conf.translation

    for (let c = conf.codepoint; c <= conf.rangeUntil; c++)
      expanded.push([
        c,
        typeof ogTranslationCode === 'number'
          ? String.fromCharCode(ogTranslationCode + (c - conf.codepoint))
          : ogTranslationCode
      ])
  } else expanded.push([conf.codepoint, conf.translation])
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

  if (!binarySearchExists(EXPECTED, codepoint)) {
    console.warn(
      `- [warn] this codepoint is not allowed: ${codepoint} (ignored)`
    )
    expanded.splice(i, 1)
    continue
  }

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

const grandTotal = new SortedSet(x => x.codepoint)

let curr
for (i = 0, curr = null; i < expanded.length; i++) {
  const [codepoint, translation] = expanded[i]
  const caseSensitive = isCaseSensitive(codepoint)

  if (translation.length === 1) {
    const [nextCodepoint, nextTranslation] = expanded[i + 1] ?? []
    const ordered =
      codepoint + 1 === nextCodepoint &&
      caseSensitive === isCaseSensitive(nextCodepoint)

    if (curr !== null) {
      if (
        ordered &&
        nextTranslation.length === 1 &&
        (curr.syncedTranslation
          ? translation.charCodeAt() + 1 === nextTranslation.charCodeAt()
          : nextTranslation === translation)
      ) {
        curr.rangeUntil++
        continue
      }

      grandTotal.push(curr)

      curr = null
      continue
    }

    const synced =
      nextTranslation &&
      translation.charCodeAt() + 1 === nextTranslation.charCodeAt() &&
      nextTranslation.length === 1

    if (ordered && (synced || nextTranslation === translation)) {
      curr = {
        caseSensitive,
        codepoint,
        translation,
        rangeUntil: codepoint + 1,
        syncedTranslation: synced
      }

      continue
    }
  }

  grandTotal.push({
    caseSensitive,
    codepoint,
    translation,
    rangeUntil: null,
    syncedTranslation: false
  })
}

console.log(
  `- condensed down from ${expanded.length.toLocaleString()} to ${grandTotal.array.length.toLocaleString()} (${(
    (grandTotal.array.length / expanded.length) *
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
    grandTotal.array
      .filter(({ translation }) => translation.length !== 1)
      .map(({ translation }) => translation)
  )
])

const codepointsBuffers = []
const caseSensitiveCodepointsBuffers = []

for (const {
  caseSensitive,
  codepoint,
  translation,
  rangeUntil,
  syncedTranslation
} of grandTotal.array) {
  const buf = Buffer.alloc(5)
  let integer = BigInt(codepoint)
  let secondByte = 0

  if (translation.length > 1) {
    const offset = strings.indexOf(translation)

    integer |=
      STRING_TRANSLATION_MASK |
      BigInt(((translation.length << 3) | (offset >> 8)) << 20)
    secondByte = offset & 0xff
  } else {
    if (rangeUntil !== null) {
      if (syncedTranslation) secondByte = 0x80

      integer |= RANGE_MASK
      secondByte |= rangeUntil - codepoint
    }

    integer |= BigInt(translation.charCodeAt() << 20)
  }

  buf.writeUint32LE(Number(integer))
  buf.writeUint8(secondByte, 4)

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
headers.writeUint16LE(headers.byteLength + codepointsBuffers.length * 5)
headers.writeUint16LE(
  headers.readUint16LE() + caseSensitiveCodepointsBuffers.length * 5,
  2
)
headers.writeUint16LE(headers.readUint16LE(2) + similarBytes.length, 4)

writeFileSync(
  'output.bin',
  Buffer.concat([
    headers,
    Buffer.concat(codepointsBuffers),
    Buffer.concat(caseSensitiveCodepointsBuffers),
    similarBytes,
    Buffer.from(strings)
  ])
)

console.log('- wrote to output.bin.')
