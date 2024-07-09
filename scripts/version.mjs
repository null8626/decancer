/* eslint-disable */

'use strict'

import { readdir, readFile, writeFile } from 'node:fs/promises'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const CORE_DIR = join(ROOT_DIR, 'core')
const NODE_DIR = join(ROOT_DIR, 'bindings', 'node')
const JRELEASER_VERSION = '1.12.0'

async function update(filename, callback) {
  await writeFile(filename, callback(await readFile(filename, 'utf-8')))
  console.log(`- updated file: ${filename}`)
}

function updateJsonFunc(str) {
  const json = JSON.parse(str)
  json.version = process.argv[2]

  return JSON.stringify(json, null, 2)
}

function updateGradleFunc(x) {
  return x
    .replace(JRELEASER_VERSION, '{JRELEASER_VERSION}')
    .replace(/'\d+\.\d+\.\d+'/g, `'${process.argv[2]}'`)
    .replace(/\/v\d+\.\d+\.\d+\//, `/v${process.argv[2]}/`)
    .replace('{JRELEASER_VERSION}', JRELEASER_VERSION)
}

const updateTomlFunc = x =>
  x.replace(/version = "\d+\.\d+\.\d+"/, `version = "${process.argv[2]}"`)
const directUpdateFunc = x => x.replace(/\d\.\d\.\d/g, process.argv[2])

function updateNativeHeaderFunc(x) {
  const versionHex = `0x${process.argv[2]
    .split('.')
    .map(x => x.padStart(2, '0'))
    .join('')}`

  return x
    .replace(
      /#define DECANCER_VERSION 0x[a-fA-F0-9]{6}/,
      `#define DECANCER_VERSION ${versionHex}`
    )
    .replace(/\d\.\d\.\d/, process.argv[2])
}

void (await Promise.all([
  update(join(CORE_DIR, 'Cargo.toml'), updateTomlFunc),
  update(join(NODE_DIR, 'Cargo.toml'), updateTomlFunc),
  update(join(ROOT_DIR, 'bindings', 'wasm', 'Cargo.toml'), updateTomlFunc),
  update(join(ROOT_DIR, 'bindings', 'native', 'Cargo.toml'), updateTomlFunc),
  update(join(NODE_DIR, 'package.json'), updateJsonFunc),
  update(
    join(ROOT_DIR, 'bindings', 'wasm', 'bin', 'decancer.min.js'),
    directUpdateFunc
  ),
  update(join(ROOT_DIR, 'bindings', 'wasm', 'example.html'), directUpdateFunc),
  update(join(ROOT_DIR, 'README.md'), directUpdateFunc),
  update(join(CORE_DIR, 'README.md'), directUpdateFunc),
  update(
    join(ROOT_DIR, 'bindings', 'native', 'decancer.h'),
    updateNativeHeaderFunc
  ),
  update(
    join(ROOT_DIR, 'bindings', 'native', 'docs', 'Doxyfile'),
    directUpdateFunc
  ),
  update(join(ROOT_DIR, 'bindings', 'node', 'README.md'), directUpdateFunc),
  update(join(CORE_DIR, 'README.md'), directUpdateFunc),
  update(join(CORE_DIR, 'src', 'lib.rs'), directUpdateFunc),
  update(join(ROOT_DIR, 'bindings', 'java', 'build.gradle'), updateGradleFunc),
  new Promise(resolve => {
    readdir(join(NODE_DIR, 'npm')).then(files => {
      Promise.all(
        files.map(file =>
          update(join(NODE_DIR, 'npm', file, 'package.json'), updateJsonFunc)
        )
      ).then(resolve)
    })
  })
]))
