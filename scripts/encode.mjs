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
    Number.isSafeInteger(conf.codepoint) && conf.codepoint < 0x110000,
    'codepoint must be a valid number'
  )
  assert(
    typeof conf.translation === 'string' &&
      [...conf.translation].every(
        c => c.codePointAt() <= 0x7f && !isCaseSensitive(c.codePointAt())
      ) &&
      conf.translation.length <= 15,
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
        `translation length for confusables with syncedTranslation must be one character in length, got '${conf.translation}'`
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
  )} confusables.\n- searching for collisions...`
)

function merge(a, b, recurse = true) {
  if (a.includes(b)) {
    return a
  } else if (b.includes(a)) {
    return b
  }

  const minimumLength = Math.min(a.length, b.length)
  let maxLimit

  for (let limit = 1; limit <= minimumLength; limit++) {
    if (a.slice(0, limit) === b.slice(-limit)) {
      maxLimit = limit
    }
  }

  if (maxLimit === undefined) {
    if (recurse) {
      return merge(b, a, false)
    }
  } else {
    return b.slice(0, -maxLimit) + a
  }
}

function mergeArray(arr, recurse = true) {
  const mergedSections = []

  while (true) {
    let index = 0

    for (; index < arr.length; index++) {
      if (arr[index] !== undefined) {
        break
      }
    }

    if (index === arr.length) {
      break
    }

    let section = arr[index]
    arr[index] = undefined

    for (index++; index < arr.length; index++) {
      if (arr[index] === undefined) {
        continue
      }

      const newSection = merge(section, arr[index])

      if (newSection) {
        section = newSection
        arr[index] = undefined
      }
    }

    mergedSections.push(section)
  }

  if (recurse) {
    return mergeArray(mergedSections, false)
  } else {
    return mergedSections.reduce((a, b) => a + b, '')
  }
}

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

  if (
    codepoint <= 127 ||
    (codepoint >= 0xa6a0 && codepoint <= 0xa6ff) ||
    (codepoint >= 0xd800 && codepoint <= 0xf8ff) ||
    (codepoint >= 0x10500 && codepoint <= 0x1052f) ||
    (codepoint >= 0x11700 && codepoint <= 0x1173f) ||
    (codepoint >= 0x118a0 && codepoint <= 0x118ff) ||
    (codepoint >= 0x16f00 && codepoint <= 0x16f9f) ||
    (codepoint >= 0x1e800 && codepoint <= 0x1e8df) ||
    (codepoint >= 0xe0100 && codepoint <= 0xe01ef) ||
    codepoint >= 0xf0000
  ) {
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

const notSyncedSequences = [],
  syncedSequences = [],
  rest = []

let curr
for (i = 0, curr = null; i < expanded.length; i++) {
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
      nextTranslation &&
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

console.log(
  `- condensed down from ${expanded.length.toLocaleString(
    'en-US'
  )} to ${grandTotal.length.toLocaleString('en-US')} (${Math.round(
    (grandTotal.length / expanded.length) * 100
  )}%).`
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
      .filter(({ translation }) => translation.length !== 1)
      .map(({ translation }) => translation)
  )
])
const confusablesBuffers = []
const caseSensitiveConfusablesBuffers = []

for (const {
  codepoint,
  translation,
  rangeUntil,
  syncedTranslation
} of grandTotal) {
  const buf = Buffer.alloc(5)
  let integer = BigInt(codepoint)
  let secondByte = 0

  if (translation.length > 1) {
    const offset = strings.indexOf(translation)

    integer |= 0x40000000n
    integer |= (BigInt(translation.length << 4) | BigInt(offset >> 8)) << 21n
    secondByte = offset & 0xff
  } else {
    if (rangeUntil !== null) {
      if (syncedTranslation) secondByte = 0x80

      integer |= 0x20000000n
      secondByte |= rangeUntil - codepoint
    }

    integer |= BigInt(translation.charCodeAt()) << 21n
  }

  buf.writeUint32LE(Number(integer))
  buf.writeUint8(secondByte, 4)

  if (isCaseSensitive(codepoint)) {
    caseSensitiveConfusablesBuffers.push(buf)
  } else {
    confusablesBuffers.push(buf)
  }
}

assert(
  strings.length <= 0xfff,
  `strings size must be equal or less than ${0xfff}. (got ${strings.length})`
)

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
    Buffer.from(strings)
  ])
)

console.log('- wrote to output.bin.')
