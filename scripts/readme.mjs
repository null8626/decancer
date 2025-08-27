/* eslint-disable */

'use strict'

import { readFile, writeFile } from 'node:fs/promises'
import { existsSync, readFileSync } from 'node:fs'
import { execSync } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { deserialize } from 'node:v8'

const CODEPOINT_MASK = 0xfffff
const STRING_TRANSLATION_MASK = 0x10000000

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const CORE_DIR = join(ROOT_DIR, 'core')
const BINDINGS_DIR = join(ROOT_DIR, 'bindings')
const CACHE_FILE = join(ROOT_DIR, '.cache.bin')

const MODIFIED_WARNING =
  '<!-- WARNING: this markdown file is computer generated.\n     please modify the README.md file in the root directory instead. -->\n\n'

// 0..=9 | 14..=31 | 127 | 0xd800..=0xf8ff | 0xe01f0..=0x10ffff
const NONE_CODEPOINTS_COUNT = 10 + 18 + 1 + 8448 + 196112

function addCodepoint(set, binary, offset) {
  const integer = binary.readUint32LE(offset)
  const secondByte = binary.readUint8(offset + 4)

  const codepoint = integer & CODEPOINT_MASK
  const isStringTranslation = integer >= STRING_TRANSLATION_MASK
  const rangeSize = isStringTranslation ? 0 : secondByte & 0x7f

  for (let i = 0; i <= rangeSize; i++) {
    set.add(codepoint + i)
  }
}

async function preprocess(readmePath, inputDefinitions) {
  const preprocessedLines = []
  let currentDefinition = null

  for (const line of rootReadmeLines) {
    if (line.startsWith('<!---[') && line.endsWith(']--->')) {
      for (let instruction of line
        .slice(6, -5)
        .trim()
        .split(/\s*\,\s*/)) {
        instruction = instruction.trim()

        if (instruction === 'end') {
          currentDefinition = null
        } else if (instruction.startsWith('begin')) {
          currentDefinition = instruction.replace(/^begin\s*/, '')
        }
      }
    } else if (
      currentDefinition === null ||
      inputDefinitions.includes(currentDefinition)
    ) {
      preprocessedLines.push(line)
    }
  }

  await writeFile(
    readmePath,
    MODIFIED_WARNING + preprocessedLines.join('\n').replaceAll(/\n{3,}/g, '\n')
  )
}

const rootReadmeLines = readFileSync(join(ROOT_DIR, 'README.md'))
  .toString()
  .trim()
  .split(/\r?\n/g)

if (!existsSync(CACHE_FILE)) {
  execSync(`node ${join(ROOT_DIR, 'scripts', 'update_unicode.mjs')}`, {
    stdio: 'inherit'
  })
}

const { alreadyHandledCount } = deserialize(readFileSync(CACHE_FILE))

const binary = await readFile(join(CORE_DIR, 'bin', 'codepoints.bin'))

let codepointsEnd = binary.readUint16LE()
const codepoints = new Set()

for (let offset = 6; offset < codepointsEnd; offset += 6) {
  addCodepoint(codepoints, binary, offset)
}

codepointsEnd = binary.readUint16LE(2)

for (let offset = binary.readUint16LE(); offset < codepointsEnd; offset += 6) {
  addCodepoint(codepoints, binary, offset)
}

for (const codepoint of [...codepoints]) {
  codepoints.add(String.fromCodePoint(codepoint).toUpperCase().codePointAt())
}

const codepointsCount =
  codepoints.size + alreadyHandledCount + NONE_CODEPOINTS_COUNT

const readmeContents = await readFile(join(ROOT_DIR, 'README.md'))

await writeFile(
  join(ROOT_DIR, 'README.md'),
  readmeContents
    .toString()
    .trim()
    .replace(
      /\*\*[\d,\.]+ \(\d+[\.\,]\d{2}%\) different unicode codepoints\*\*/,
      `**${codepointsCount.toLocaleString('en-US')} (${(
        (codepointsCount / 0x10ffff) *
        100
      ).toFixed(2)}%) different unicode codepoints**`
    )
)

await Promise.all([
  preprocess(join(CORE_DIR, 'README.md'), 'DECANCER_RUST'),
  preprocess(join(BINDINGS_DIR, 'node', 'README.md'), 'DECANCER_JS'),
  preprocess(join(BINDINGS_DIR, 'native', 'README.md'), 'DECANCER_NATIVE'),
  preprocess(join(BINDINGS_DIR, 'go', 'README.md'), 'DECANCER_GO')
])
