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

const expectedNativeTargets = [
  'aarch64-apple-darwin',
  'aarch64-apple-ios',
  'aarch64-apple-ios-sim',
  'aarch64-linux-android',
  'aarch64-pc-windows-msvc',
  'aarch64-unknown-linux-gnu',
  'aarch64-unknown-linux-musl',
  'arm-unknown-linux-gnueabi',
  'armv5te-unknown-linux-gnueabi',
  'armv7-linux-androideabi',
  'armv7-unknown-linux-gnueabi',
  'armv7-unknown-linux-gnueabihf',
  'i586-unknown-linux-gnu',
  'i686-pc-windows-msvc',
  'i686-unknown-freebsd',
  'i686-unknown-linux-gnu',
  'powerpc64le-unknown-linux-gnu',
  'riscv64gc-unknown-linux-gnu',
  's390x-unknown-linux-gnu',
  'sparcv9-sun-solaris',
  'thumbv7neon-unknown-linux-gnueabihf',
  'x86_64-apple-darwin',
  'x86_64-apple-ios',
  'x86_64-pc-windows-msvc',
  'x86_64-unknown-freebsd',
  'x86_64-unknown-illumos',
  'x86_64-unknown-linux-gnu',
  'x86_64-unknown-linux-musl'
]

const expectedNodeTargets = [
  'x86_64-apple-darwin',
  'x86_64-pc-windows-msvc',
  'i686-pc-windows-msvc',
  'x86_64-unknown-linux-gnu',
  'x86_64-unknown-linux-musl',
  'aarch64-apple-darwin',
  'aarch64-unknown-linux-gnu',
  'armv7-unknown-linux-gnueabihf',
  'aarch64-linux-android',
  'armv7-linux-androideabi',
  'aarch64-unknown-linux-musl',
  'aarch64-pc-windows-msvc' //, 'freebsd-x64'
]

let foundJavaJar = false

void (await Promise.all(
  artifacts.map(async artifact => {
    if (artifact.startsWith('native-')) {
      const target = artifact.slice(7)

      expectedNativeTargets.splice(expectedNativeTargets.indexOf(target), 1)

      await execute(`zip ../decancer-${target}.zip ./${artifact}/*`, {
        cwd: ARTIFACTS_DIR,
        stdio: 'inherit'
      })
    } else if (artifact.startsWith('node-')) {
      expectedNodeTargets.splice(
        expectedNodeTargets.indexOf(artifact.slice(5)),
        1
      )

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
    } else if (artifact === 'java-jar') {
      await rename(
        join(ARTIFACTS_DIR, artifact, 'decancer.jar'),
        join(ROOT_DIR, 'decancer.jar')
      )
      
      foundJavaJar = true
    }
  })
))

if (expectedNativeTargets.length !== 0 || expectedNodeTargets.length !== 0 || !foundJavaJar) {
  console.error('error: found missing targets. exiting.')
  process.exit(1)
}
