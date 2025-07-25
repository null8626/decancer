/* eslint-disable */

'use strict'

import { readdir } from 'node:fs/promises'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')

const expectedJavaTargets = [
  'aarch64-apple-darwin',
  'aarch64-pc-windows-msvc',
  'aarch64-unknown-linux-gnu',
  'aarch64-unknown-linux-musl',
  'arm-unknown-linux-gnueabi',
  'armv5te-unknown-linux-gnueabi',
  'armv7-unknown-linux-gnueabi',
  'armv7-unknown-linux-gnueabihf',
  'i686-pc-windows-msvc',
  'i686-unknown-freebsd',
  'i686-unknown-linux-gnu',
  'riscv64gc-unknown-linux-gnu',
  'x86_64-apple-darwin',
  'x86_64-pc-windows-msvc',
  'x86_64-unknown-freebsd',
  'x86_64-unknown-linux-gnu',
  'x86_64-unknown-linux-musl'
]

const artifacts = await readdir(join(ROOT_DIR, 'java-artifacts'))

for (const artifact of artifacts) {
  expectedJavaTargets.splice(expectedJavaTargets.indexOf(artifact.slice(5)), 1)
}

if (expectedJavaTargets.length !== 0) {
  console.error('error: found missing targets')
  process.exit(1)
}
