/* eslint-disable */

'use strict'

import { readdir, rename } from 'node:fs/promises'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const TARGET = process.argv[2]
const IS_JAVA = process.argv.some(argv => argv === '--java')

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const TARGET_DIR = join(
  ROOT_DIR,
  'bindings',
  IS_JAVA ? 'java' : 'native',
  TARGET,
  'release'
)

const artifacts = await readdir(TARGET_DIR)
const promises = []

for (const artifact of artifacts) {
  try {
    console.log(artifact)
    const ext = artifact.match(/\.\w+$/)[0].slice(1)
    console.log(ext)

    if (
      (!IS_JAVA && ext === '.lib') ||
      ext === 'dll' ||
      ext === 'so' ||
      ext === 'dylib'
    ) {
      if (IS_JAVA) {
        name = name.replace('decancer', `decancer-${TARGET}`)
      }

      promises.push(
        rename(join(TARGET_DIR, artifact), join(ROOT_DIR, 'artifacts', name))
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
