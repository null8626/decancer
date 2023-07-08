import { readFileSync, writeFileSync } from 'node:fs'

const CODEPOINT_MASK = 0xfffff
const RANGE_MASK = 0x8000000
const STRING_TRANSLATION_MASK = 0x10000000

if (typeof process.argv[2] !== 'string') {
  console.error('error: missing binary file path.')
  process.exit(1)
}

class Codepoints {
  #inner

  constructor() {
    this.#inner = []
  }

  push(input) {
    if (input.translation === '\0') {
      input.translation = ''
    }

    if (process.argv[3] === '--full') {
      if (input.rangeUntil === null) {
        this.#inner.push({
          codepoint: input.codepoint,
          translation: input.translation
        })
      } else {
        const ogTranslationCode = input.syncedTranslation
          ? input.translation.charCodeAt()
          : input.translation

        for (let c = input.codepoint; c <= input.rangeUntil; c++)
          this.#inner.push({
            codepoint: c,
            translation:
              typeof ogTranslationCode === 'number'
                ? String.fromCharCode(ogTranslationCode + (c - input.codepoint))
                : ogTranslationCode
          })
      }
    } else {
      this.#inner.push(input)
    }
  }

  get inner() {
    return this.#inner
  }
}

const binary = readFileSync(process.argv[2])

const similar = []
let currentSimilar = []

let offset = binary.readUint16LE(2)
const offsetEnd = binary.readUint16LE(4)

do {
  const current = binary.readUint8(offset)

  if (current >= 0x80) {
    similar.push([...currentSimilar, String.fromCharCode(current & 0x7f)])
    currentSimilar = []
  } else {
    currentSimilar.push(String.fromCharCode(current))
  }

  offset++
} while (offset < offsetEnd)

function getTranslation(integer, secondByte) {
  const offset =
    binary.readUint16LE(4) + ((((integer >> 20) & 0x07) << 8) | secondByte)

  return binary.subarray(offset, offset + ((integer >> 23) & 0x1f)).toString()
}

let codepointsEnd = binary.readUint16LE()
let codepoints = new Codepoints()

for (let offset = 6; offset < codepointsEnd; offset += 5) {
  const integer = binary.readUint32LE(offset)
  const secondByte = binary.readUint8(offset + 4)

  const codepoint = integer & CODEPOINT_MASK

  codepoints.push({
    codepoint,
    translation:
      (integer & STRING_TRANSLATION_MASK) !== 0
        ? getTranslation(integer, secondByte)
        : String.fromCharCode((integer >> 20) & 0x7f),
    rangeUntil:
      (integer & RANGE_MASK) !== 0 ? codepoint + (secondByte & 0x7f) : null,
    syncedTranslation: secondByte >= 0x80
  })
}

codepointsEnd = binary.readUint16LE(2)

for (let offset = binary.readUint16LE(); offset < codepointsEnd; offset += 5) {
  const integer = binary.readUint32LE(offset)
  const secondByte = binary.readUint8(offset + 4)

  const codepoint = integer & CODEPOINT_MASK

  codepoints.push({
    codepoint,
    translation:
      (integer & STRING_TRANSLATION_MASK) !== 0
        ? getTranslation(integer, secondByte)
        : String.fromCharCode((integer >> 20) & 0x7f),
    rangeUntil:
      (integer & RANGE_MASK) !== 0 ? codepoint + (secondByte & 0x7f) : null,
    syncedTranslation: secondByte >= 0x80
  })
}

writeFileSync(
  'output.json',
  JSON.stringify(
    {
      codepoints: codepoints.inner.sort((a, b) => a.codepoint - b.codepoint),
      similar
    },
    null,
    2
  )
)
