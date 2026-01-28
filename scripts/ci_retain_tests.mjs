/* eslint-disable */

import {
  CACHE_FILE,
  CODEPOINT_MASK,
  CORE_DIR,
  MODIFIED_RETAIN_TESTS_WARNING,
  RETAIN_TESTS_SAMPLE_SIZE,
  RETAINABLE_SCRIPTS,
  TURKISH_CHARACTERS,
  ROOT_DIR,
  SPDX_LICENSE_COMMENTS
} from './constants.mjs'
import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { binarySearchExists } from './util.mjs'
import { execSync } from 'node:child_process'
import { deserialize } from 'node:v8'
import { join } from 'node:path'

if (!existsSync(CACHE_FILE)) {
  execSync(`node "${join(ROOT_DIR, 'scripts', 'update_unicode.mjs')}"`, {
    stdio: 'inherit'
  })
}

const binary = readFileSync(join(CORE_DIR, 'bin', 'codepoints.bin'))
const { blocks } = deserialize(readFileSync(CACHE_FILE))

let codepointsEnd = binary.readUint16LE()
const codepoints = []

for (let offset = 6; offset < codepointsEnd; offset += 6) {
  codepoints.push(binary.readUint32LE(offset) & CODEPOINT_MASK)
}

codepointsEnd = binary.readUint16LE(2)

for (let offset = binary.readUint16LE(); offset < codepointsEnd; offset += 6) {
  codepoints.push(binary.readUint32LE(offset) & CODEPOINT_MASK)
}

const retain = {}

for (const { start, end, name } of blocks) {
  for (let codepoint = start; codepoint <= end; codepoint++) {
    if (binarySearchExists(codepoints, codepoint)) {
      const retainScript = RETAINABLE_SCRIPTS.find(([n, data]) =>
        typeof data === 'number' ? name.includes(n) : data.check(name)
      )

      if (retainScript) {
        const key = retainScript[0]

        if (retain[key]) {
          retain[key].push(codepoint)
        } else {
          retain[key] = [codepoint]
        }
      }
    }
  }
}

let testCode = `${SPDX_LICENSE_COMMENTS}

${MODIFIED_RETAIN_TESTS_WARNING}

#[test]
#[cfg(feature = "options")]
#[allow(clippy::unicode_not_nfc)]
fn retains() {
  let test_retain = |options: Options, test_string| {
    assert_ne!(super::cure!(test_string).unwrap(), test_string);
    assert_eq!(super::cure(test_string, options.disable_bidi()).unwrap(), test_string);
  };

  test_retain(Options::default().retain_turkish(), "${TURKISH_CHARACTERS.join('')}");\n`

for (const [name, codepoints] of Object.entries(retain)) {
  const middleIndex = Math.round(codepoints.length / 2)
  const middleOffset = Math.round(RETAIN_TESTS_SAMPLE_SIZE / 2)

  const testString = String.fromCodePoint(
    ...new Set([
      ...codepoints.slice(0, RETAIN_TESTS_SAMPLE_SIZE),
      ...codepoints.slice(
        middleIndex - middleOffset,
        middleIndex + (RETAIN_TESTS_SAMPLE_SIZE - middleOffset)
      ),
      ...codepoints.slice(-RETAIN_TESTS_SAMPLE_SIZE)
    ])
  )

  testCode += `  test_retain(Options::default().retain_${name}(), "${testString}");\n`
}

testCode += '}'

writeFileSync(join(CORE_DIR, 'src', 'retain_tests.rs'), testCode)
