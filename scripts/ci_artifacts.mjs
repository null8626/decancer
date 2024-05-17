/* eslint-disable */

'use strict'

import { readdir, rename, mkdir } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const ARTIFACTS_DIR = join(ROOT_DIR, 'artifacts')
const NODE_DIR = join(ROOT_DIR, 'bindings', 'node')

const execute = promisify(exec)

const [artifacts] = await Promise.all([
  readdir(join(ROOT_DIR, 'artifacts')),
  mkdir(join(NODE_DIR, 'artifacts'))
])

void (await Promise.all(
  artifacts.map(async artifact => {
    if (artifact.startsWith('native-')) {
      await execute(
        `zip ../decancer-${artifact.slice(7)}.zip ./${artifact}/*`,
        {
          cwd: ARTIFACTS_DIR,
          stdio: 'inherit'
        }
      )
    } else if (artifact.startsWith('node-')) {
      const artifactsDir = join(
        join(NODE_DIR, 'artifacts'),
        artifact.replace(/^node-/, 'bindings-')
      )
      const originDir = join(ARTIFACTS_DIR, artifact)

      const [[nodeBinary]] = await Promise.all([
        readdir(originDir),
        mkdir(artifactsDir)
      ])

      await rename(join(originDir, nodeBinary), join(artifactsDir, nodeBinary))
    }
  })
))
