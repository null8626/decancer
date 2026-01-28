/* eslint-disable */

import {
  BINDINGS_DIR,
  EXPECTED_NATIVE_TARGETS,
  EXPECTED_NODE_TARGETS,
  ROOT_DIR
} from './constants.mjs'
import { readdir, rename, mkdir } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { promisify } from 'node:util'
import process from 'node:process'
import { join } from 'node:path'

const ARTIFACTS_DIR = join(ROOT_DIR, 'artifacts')
const NODE_ARTIFACTS_DIR = join(BINDINGS_DIR, 'node', 'artifacts')

const execute = promisify(exec)

const [artifacts] = await Promise.all([
  readdir(ARTIFACTS_DIR),
  mkdir(NODE_ARTIFACTS_DIR)
])

let foundJavaJar = false

void (await Promise.all(
  artifacts.map(async artifact => {
    if (artifact.startsWith('native-')) {
      const target = artifact.slice(7)

      EXPECTED_NATIVE_TARGETS.splice(EXPECTED_NATIVE_TARGETS.indexOf(target), 1)

      await execute(`zip ../decancer-${target}.zip ./${artifact}/*`, {
        cwd: ARTIFACTS_DIR,
        stdio: 'inherit'
      })
    } else if (artifact.startsWith('node-')) {
      EXPECTED_NODE_TARGETS.splice(
        EXPECTED_NODE_TARGETS.indexOf(artifact.slice(5)),
        1
      )

      const artifactsDir = join(
        NODE_ARTIFACTS_DIR,
        artifact.replace(/^node-/, 'bindings-')
      )
      const originDir = join(ARTIFACTS_DIR, artifact)

      const [[nodeBinary]] = await Promise.all([
        readdir(originDir),
        mkdir(artifactsDir)
      ])

      await rename(join(originDir, nodeBinary), join(artifactsDir, nodeBinary))
    } else if (artifact === 'java-jar') {
      await rename(
        join(ARTIFACTS_DIR, artifact, 'decancer.jar'),
        join(ROOT_DIR, 'decancer.jar')
      )

      foundJavaJar = true
    }
  })
))

if (EXPECTED_NODE_TARGETS.length !== 0 || !foundJavaJar) {
  console.error('error: found missing targets')
  process.exit(1)
}
