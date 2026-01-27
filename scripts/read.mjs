/* eslint-disable */

'use strict'

import {
  CODEPOINT_MASK,
  CORE_DIR,
  STRING_TRANSLATION_MASK
} from './constants.mjs'
import { readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'

class Codepoints {
  #inner

  constructor() {
    this.#inner = []
  }

  push(input) {
    if (input.translation === '\0') {
      input.translation = ''
    }

    const ogTranslationCode = input.syncedTranslation
      ? input.translation.charCodeAt()
      : input.translation

    for (let i = 0; i <= input.rangeSize; i++) {
      this.#inner.push({
        codepoint: input.codepoint + i,
        translation:
          typeof ogTranslationCode === 'number'
            ? String.fromCharCode(ogTranslationCode + i)
            : ogTranslationCode
      })
    }
  }

  get inner() {
    return this.#inner
  }
}

const binary = readFileSync(join(CORE_DIR, 'bin', 'codepoints.bin'))

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

for (let offset = 6; offset < codepointsEnd; offset += 6) {
  const integer = binary.readUint32LE(offset)
  const secondByte = binary.readUint8(offset + 4)

  const codepoint = integer & CODEPOINT_MASK
  const isStringTranslation = integer >= STRING_TRANSLATION_MASK

  codepoints.push({
    codepoint,
    translation: isStringTranslation
      ? getTranslation(integer, secondByte)
      : String.fromCharCode((integer >> 20) & 0x7f),
    rangeSize: isStringTranslation ? 0 : secondByte & 0x7f,
    syncedTranslation: !isStringTranslation && secondByte >= 0x80
  })
}

codepointsEnd = binary.readUint16LE(2)

for (let offset = binary.readUint16LE(); offset < codepointsEnd; offset += 6) {
  const integer = binary.readUint32LE(offset)
  const secondByte = binary.readUint8(offset + 4)

  const codepoint = integer & CODEPOINT_MASK
  const isStringTranslation = integer >= STRING_TRANSLATION_MASK

  codepoints.push({
    codepoint,
    translation: isStringTranslation
      ? getTranslation(integer, secondByte)
      : String.fromCharCode((integer >> 20) & 0x7f),
    rangeSize: isStringTranslation ? 0 : secondByte & 0x7f,
    syncedTranslation: !isStringTranslation && secondByte >= 0x80
  })
}

if (process.argv[2]?.endsWith('.txt')) {
  const translationMap = {}

  for (const { codepoint, translation } of codepoints.inner) {
    if (translationMap[translation]) {
      translationMap[translation].push(codepoint)
    } else {
      translationMap[translation] = [codepoint]
    }
  }

  writeFileSync(
    process.argv[2],
    Object.entries(translationMap)
      .map(
        ([translation, codepoints]) =>
          `${translation}:\n${codepoints.map(c => String.fromCodePoint(c)).join('')}`
      )
      .join('\n\n')
  )
} else {
  writeFileSync(
    process.argv[2]?.endsWith('.json') ? process.argv[2] : 'output.json',
    JSON.stringify(
      {
        codepoints: codepoints.inner.sort((a, b) => a.codepoint - b.codepoint),
        similar
      },
      null,
      2
    )
  )
}
