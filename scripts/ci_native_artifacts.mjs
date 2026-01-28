/* eslint-disable */

import { BINDINGS_DIR, ROOT_DIR } from './constants.mjs'
import { readdir, rename } from 'node:fs/promises'
import process from 'node:process'
import { join } from 'node:path'

const TARGET = process.argv[2]
const IS_JAVA = process.argv.slice(3).some(argv => argv === '--java')

const TARGET_DIR = join(
  BINDINGS_DIR,
  IS_JAVA ? 'java' : 'native',
  TARGET,
  'release'
)

const artifacts = await readdir(TARGET_DIR)
const promises = []

for (const artifact of artifacts) {
  try {
    const ext = artifact.match(/\.\w+$/)[0].slice(1)

    if (
      (!IS_JAVA && ext === 'lib') ||
      ext === 'dll' ||
      ext === 'so' ||
      ext === 'dylib'
    ) {
      const outputArtifact = IS_JAVA
        ? artifact.replace('decancer', `decancer-${TARGET}`)
        : artifact

      promises.push(
        rename(
          join(TARGET_DIR, artifact),
          join(ROOT_DIR, 'artifacts', outputArtifact)
        )
      )
    }
  } catch {
    continue
  }
}

if (promises.length === 0) {
  console.error('error: target directory is empty')
  process.exit(1)
}

void (await Promise.all(promises))
