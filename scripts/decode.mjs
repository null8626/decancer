import { readFileSync, writeFileSync } from 'node:fs'

if (typeof process.argv[2] !== 'string') {
  console.error('error: missing binary file path.')
  process.exit(1)
}

class Confusables {
  #inner

  constructor() {
    this.#inner = []
  }

  push({ codepoint, translation, rangeUntil, syncedTranslation }) {
    if (process.argv[3] === '--full') {
      if (rangeUntil === null) {
        this.#inner.push({ codepoint, translation })
      } else {
        const ogTranslationCode = syncedTranslation
          ? translation.charCodeAt()
          : translation

        for (let c = codepoint; c <= rangeUntil; c++)
          this.#inner.push({
            codepoint: c,
            translation:
              typeof ogTranslationCode === 'number'
                ? String.fromCharCode(ogTranslationCode + (c - codepoint))
                : ogTranslationCode
          })
      }
    } else {
      this.#inner.push(arguments[0])
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
    binary.readUint16LE(4) + ((((integer >> 21) & 0x0f) << 8) | secondByte)

  return binary.subarray(offset, offset + ((integer >> 25) & 0x0f)).toString()
}

let confusablesEnd = binary.readUint16LE()
let confusables = new Confusables()

for (let offset = 6; offset < confusablesEnd; offset += 5) {
  const integer = binary.readUint32LE(offset)
  const secondByte = binary.readUint8(offset + 4)

  const codepoint = integer & 0x1fffff

  confusables.push({
    codepoint,
    translation:
      (integer & 0x40000000) !== 0
        ? getTranslation(integer, secondByte)
        : String.fromCharCode((integer >> 21) & 0xff),
    rangeUntil:
      (integer & 0x20000000) !== 0 ? codepoint + (secondByte & 0x7f) : null,
    syncedTranslation: secondByte >= 0x80
  })
}

confusablesEnd = binary.readUint16LE(2)

for (let offset = binary.readUint16LE(); offset < confusablesEnd; offset += 5) {
  const integer = binary.readUint32LE(offset)
  const secondByte = binary.readUint8(offset + 4)

  const codepoint = integer & 0x1fffff

  confusables.push({
    codepoint,
    translation:
      (integer & 0x40000000) !== 0
        ? getTranslation(integer, secondByte)
        : String.fromCharCode((integer >> 21) & 0xff),
    rangeUntil:
      (integer & 0x20000000) !== 0 ? codepoint + (secondByte & 0x7f) : null,
    syncedTranslation: secondByte >= 0x80
  })
}

writeFileSync(
  'output.json',
  JSON.stringify(
    {
      confusables: confusables.inner.sort((a, b) => a.codepoint - b.codepoint),
      similar
    },
    null,
    2
  )
)
