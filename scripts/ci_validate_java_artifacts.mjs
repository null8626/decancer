/* eslint-disable */

'use strict'

import { EXPECTED_JAVA_TARGETS, ROOT_DIR } from './constants.mjs'
import { readdir } from 'node:fs/promises'
import { join } from 'node:path'

const artifacts = await readdir(join(ROOT_DIR, 'java-artifacts'))

for (const artifact of artifacts) {
  EXPECTED_JAVA_TARGETS.splice(
    EXPECTED_JAVA_TARGETS.indexOf(artifact.slice(5)),
    1
  )
}

if (EXPECTED_JAVA_TARGETS.length !== 0) {
  console.error('error: found missing targets')
  process.exit(1)
}
