import { readFileSync, writeFileSync } from 'node:fs'

if (typeof process.argv[2] !== 'string') {
  console.error('error: missing binary file path.')
  process.exit(1)
}

const binary = readFileSync(process.argv[2])

let bytes = binary.subarray(binary.readUint16LE(2), binary.readUint16LE(4))
const similar = []

while (bytes.length !== 0) {
  const length = bytes.readUint8()
  similar.push(
    Array.from(bytes.subarray(1, 1 + length)).map(x => String.fromCharCode(x))
  )

  bytes = bytes.subarray(1 + length)
}

bytes = binary.subarray(binary.readUint16LE(4))
const strings = []

while (bytes.length !== 0) {
  const length = bytes.readUint8()
  strings.push(
    Array.from(bytes.subarray(1, 1 + length)).reduce(
      (a, b) => a + String.fromCharCode(b),
      ''
    )
  )

  bytes = bytes.subarray(1 + length)
}

let confusablesEnd = binary.readUint16LE()
let confusables = []

for (let offset = 6; offset < confusablesEnd; offset += 5) {
  const integer = binary.readUint32LE(offset)
  const secondByte = binary.readUint8(offset + 4)

  const codepoint = integer & 0x1fffff
  const translationCode = (integer >> 21) & 0xff

  confusables.push({
    codepoint,
    translation:
      (integer & 0x40000000) !== 0
        ? strings[translationCode]
        : String.fromCharCode(translationCode),
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
  const translationCode = (integer >> 21) & 0xff

  confusables.push({
    codepoint,
    translation:
      (integer & 0x40000000) !== 0
        ? strings[translationCode]
        : String.fromCharCode(translationCode),
    rangeUntil:
      (integer & 0x20000000) !== 0 ? codepoint + (secondByte & 0x7f) : null,
    syncedTranslation: secondByte >= 0x80
  })
}

writeFileSync(
  'output.json',
  JSON.stringify(
    {
      confusables: confusables.sort((a, b) => a.codepoint - b.codepoint),
      similar
    },
    null,
    2
  )
)
