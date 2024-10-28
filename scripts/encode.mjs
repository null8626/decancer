import { readFileSync, writeFileSync } from 'node:fs'
import { inspect } from 'node:util'
import assert from 'node:assert'

if (typeof process.argv[2] !== 'string') {
  console.error('error: missing json file path.')
  process.exit(1)
}

const { confusables, similar } = JSON.parse(readFileSync(process.argv[2]))

assert(
  Array.isArray(confusables) && confusables.length > 0,
  'confusables must be an array'
)
assert(
  Array.isArray(similar) &&
    similar.length > 0 &&
    similar.length <= 0x7f &&
    similar.every(
      x =>
        Array.isArray(x) &&
        x.length > 0 &&
        x.length <= 0xff &&
        x.every(y => y.length === 1 && y.codePointAt() <= 0xff)
    ),
  'similar must be an array of an array of ASCII strings'
)

function isCaseSensitive(x) {
  return String.fromCodePoint(x).toLowerCase().codePointAt() !== x
}

console.log(
  `- checking, expanding, and sorting ${confusables.length.toLocaleString(
    'en-US'
  )} confusables...`
)

let expanded = []

for (const conf of confusables) {
  assert(
    Number.isSafeInteger(conf.codepoint) &&
      conf.codepoint >= 0x80 &&
      conf.codepoint < 0x110000,
    'codepoint must be a valid number'
  )
  assert(
    typeof conf.translation === 'string' && conf.translation.length >= 1,
    'translation must be a string'
  )

  if (typeof conf.rangeUntil === 'number') {
    assert(
      Number.isSafeInteger(conf.rangeUntil) &&
        conf.rangeUntil >= 0 &&
        conf.rangeUntil < 0x110000 &&
        conf.rangeUntil - conf.codepoint <= 0x7f,
      'rangeUntil must be a valid number'
    )
    assert(
      conf.rangeUntil > conf.codepoint,
      `rangeUntil must be greater than codepoint. (rangeUntil: ${conf.rangeUntil}, codepoint: ${conf.codepoint})`
    )

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
  )} confusables.\n- searching for collisions...`
)

function retrieveCollisions(array, set) {
  for (const part of set) array.splice(array.indexOf(part), 1)

  return array
}

{
  const set = Array.from(new Set(expanded.map(([codepoint]) => codepoint)))
  assert(
    expanded.length === set.length,
    `discovered ${(expanded.length - set.length).toLocaleString(
      'en-US'
    )} collisions. at codepoints: ${inspect(
      retrieveCollisions(
        expanded.map(([codepoint]) => codepoint),
        set
      )
    )}`
  )
}

const caseSensitiveCollisions = []

for (const [codepoint, translation] of expanded) {
  if (isCaseSensitive(codepoint)) {
    const lowercasedCodepoint = String.fromCodePoint(codepoint)
      .toLowerCase()
      .codePointAt()
    const already = expanded.find(
      ([codepoint2, translation2]) =>
        codepoint2 === lowercasedCodepoint && translation === translation2
    )

    if (already) {
      caseSensitiveCollisions.push(codepoint)
    }
  }
}

assert(
  caseSensitiveCollisions.length === 0,
  `discovered ${caseSensitiveCollisions.length.toLocaleString(
    'en-US'
  )} case-sensitive collisions. at codepoints: ${inspect(
    caseSensitiveCollisions
  )}`
)

const notSyncedSequences = [],
  syncedSequences = [],
  rest = []

for (let i = 0, curr = null; i < expanded.length; i++) {
  const [codepoint, translation] = expanded[i]

  if (translation.length === 1) {
    const [nextCodepoint, nextTranslation] = expanded[i + 1] ?? []
    const ordered =
      codepoint + 1 === nextCodepoint &&
      isCaseSensitive(codepoint) === isCaseSensitive(nextCodepoint)

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

      if (curr.syncedTranslation) syncedSequences.push(curr)
      else notSyncedSequences.push(curr)

      curr = null
      continue
    }

    const synced =
      translation.charCodeAt() + 1 === nextTranslation.charCodeAt() &&
      nextTranslation.length === 1

    if (ordered && (synced || nextTranslation === translation)) {
      curr = {
        codepoint,
        translation,
        rangeUntil: codepoint + 1,
        syncedTranslation: synced
      }

      continue
    }
  }

  rest.push({
    codepoint,
    translation,
    rangeUntil: null,
    syncedTranslation: false
  })
}

const sequenceReduceFunc = (a, b) => a + (b.rangeUntil - b.codepoint) + 1
console.log(
  `- discovered ${syncedSequences.length.toLocaleString('en-US')} (${Math.round(
    (syncedSequences.reduce(sequenceReduceFunc, 0) / expanded.length) * 100
  )}%) synced sequences and ${notSyncedSequences.length.toLocaleString(
    'en-US'
  )} (${Math.round(
    (notSyncedSequences.reduce(sequenceReduceFunc, 0) / expanded.length) * 100
  )}%) unsynced sequences.`
)

const grandTotal = [...syncedSequences, ...notSyncedSequences, ...rest].sort(
  (a, b) => a.codepoint - b.codepoint
)

writeFileSync(
  process.argv[2].replace(/\.json$/i, 'Optimized.json'),
  JSON.stringify({ confusables: grandTotal, similar }, null, 2)
)

console.log(
  `- condensed down from ${expanded.length.toLocaleString(
    'en-US'
  )} to ${grandTotal.length.toLocaleString('en-US')} (${Math.round(
    (grandTotal.length / expanded.length) * 100
  )}%). (wrote refactored JSON output to ${process.argv[2].replace(
    /\.json$/i,
    'Optimized.json'
  )})`
)

const similarBytes = Buffer.concat(
  similar.map(x => Buffer.from([x.length, ...x.map(y => y.charCodeAt())]))
)
const strings = []
const confusablesBuffers = []
const caseSensitiveConfusablesBuffers = []

for (const {
  codepoint,
  translation,
  rangeUntil,
  syncedTranslation
} of grandTotal) {
  const buf = Buffer.alloc(5)
  let integer = 0x100000000n | BigInt(codepoint)
  let secondByte = 0

  if (syncedTranslation) secondByte = 0x80

  if (rangeUntil !== null) {
    integer |= 0x10000000n
    secondByte |= rangeUntil - codepoint
  }

  if (translation.length > 1) {
    if (!strings.includes(translation)) strings.push(translation)

    integer |= 0x20000000n
    integer |= BigInt(strings.indexOf(translation)) << 21n
  } else {
    integer |= BigInt(translation.charCodeAt()) << 21n
  }

  buf.writeUint32LE(Number(integer & 0xffffffffn))
  buf.writeUint8(secondByte, 4)

  if (isCaseSensitive(codepoint)) {
    caseSensitiveConfusablesBuffers.push(buf)
  } else {
    confusablesBuffers.push(buf)
  }
}

const headers = Buffer.alloc(6)
headers.writeUint16LE(headers.byteLength + confusablesBuffers.length * 5)
headers.writeUint16LE(
  headers.readUint16LE() + caseSensitiveConfusablesBuffers.length * 5,
  2
)
headers.writeUint16LE(headers.readUint16LE(2) + similarBytes.length, 4)

writeFileSync(
  'output.bin',
  Buffer.concat([
    headers,
    Buffer.concat(confusablesBuffers),
    Buffer.concat(caseSensitiveConfusablesBuffers),
    similarBytes,
    Buffer.from(
      strings
        .map(x => [x.length, ...x.split('').map(y => y.charCodeAt())])
        .flat()
    )
  ])
)

console.log('- wrote to output.bin.')
