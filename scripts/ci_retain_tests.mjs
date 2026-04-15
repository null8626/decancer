/* eslint-disable */

import {
  BINDINGS_DIR,
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
import { binarySearchExists, snakeToCamel, snakeToPascal } from './util.mjs'
import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { execSync } from 'node:child_process'
import { deserialize } from 'node:v8'
import { join } from 'node:path'

if (!existsSync(CACHE_FILE)) {
  execSync(`node "${join(ROOT_DIR, 'scripts', 'update_unicode.mjs')}"`, {
    stdio: 'inherit'
  })
}

const TURKISH_TEST_STRING = TURKISH_CHARACTERS.join('')

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

let coreTestCode = `${SPDX_LICENSE_COMMENTS}

${MODIFIED_RETAIN_TESTS_WARNING}

fn do_retain_test(options: Options, test_string: &str) {
  assert_ne!(super::cure!(test_string).unwrap(), test_string);
  assert_eq!(super::cure(test_string, options.disable_bidi()).unwrap(), test_string);
}

#[test]
#[cfg(feature = "options")]
#[allow(clippy::unicode_not_nfc)]
fn retains() {
  do_retain_test(Options::default().retain_turkish(), "${TURKISH_TEST_STRING}");`

let goTestCode = `${SPDX_LICENSE_COMMENTS}

${MODIFIED_RETAIN_TESTS_WARNING}

package decancer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func DoRetainTest(t *testing.T, options Option, input string) {
	defaultCured, err := Cure(input, Default)

	assert.Nil(t, err, "curing should not fail")

	defer defaultCured.Close()

	assert.False(t, defaultCured.Equals(input), "Default should make decancer cure the designated characters")

	retainCured, err := Cure(input, options | DisableBidi)

	assert.Nil(t, err, "curing should not fail")

	defer retainCured.Close()

	assert.True(t, retainCured.Equals(input), "Retain should prevent decancer from curing the designated characters")
}

func TestRetains(t *testing.T) {
	DoRetainTest(t, RetainTurkish, "${TURKISH_TEST_STRING}")`

const jsonData = {
  retainTurkish: TURKISH_TEST_STRING
}

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

  coreTestCode += `\n  do_retain_test(Options::default().retain_${name}(), "${testString}");`
  goTestCode += `\n\tDoRetainTest(t, ${snakeToPascal(`retain_${name}`)}, "${testString}")`

  jsonData[snakeToCamel(`retain_${name}`)] = testString
}

coreTestCode += '\n}'
goTestCode += '\n}'

writeFileSync(join(CORE_DIR, 'src', 'retain_tests.rs'), coreTestCode)
writeFileSync(join(BINDINGS_DIR, 'go', 'decancer_retain_test.go'), goTestCode)
writeFileSync(join(BINDINGS_DIR, 'node', 'retain_data.json'), JSON.stringify(jsonData, null, 2))
