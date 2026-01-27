/* eslint-disable */

'use strict'

import {
  BINDINGS_DIR,
  CORE_DIR,
  JRELEASER_VERSION,
  ROOT_DIR
} from './constants.mjs'
import { readdir, readFile, writeFile } from 'node:fs/promises'
import { join } from 'node:path'

const NODE_DIR = join(BINDINGS_DIR, 'node')
const JAVA_SOURCES_DIR = join(
  BINDINGS_DIR,
  'java',
  'src',
  'main',
  'java',
  'io',
  'github',
  'null8626',
  'decancer'
)

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

const updateDocStringVersionFunc = x =>
  x.replace(/@version \d\.\d\.\d/, `@version ${process.argv[2]}`)

function updateNativeHeaderFunc(x) {
  const versionHex = `0x${process.argv[2]
    .split('.')
    .map(x => x.padStart(2, '0'))
    .join('')}`

  return updateDocStringVersionFunc(
    x
      .replace(
        /#define DECANCER_VERSION 0x[a-fA-F0-9]{6}/,
        `#define DECANCER_VERSION ${versionHex}`
      )
      .replace(
        /@date \d{4}\-\d{2}\-\d{2}/,
        `@date ${new Date().toISOString().replace(/T[\d\:\.]+Z$/, '')}`
      )
  )
}

void (await Promise.all([
  update(join(CORE_DIR, 'Cargo.toml'), updateTomlFunc),
  update(join(NODE_DIR, 'Cargo.toml'), updateTomlFunc),
  update(join(BINDINGS_DIR, 'wasm', 'Cargo.toml'), updateTomlFunc),
  update(join(BINDINGS_DIR, 'native', 'Cargo.toml'), updateTomlFunc),
  update(join(NODE_DIR, 'package.json'), updateJsonFunc),
  update(
    join(BINDINGS_DIR, 'wasm', 'bin', 'decancer.min.js'),
    directUpdateFunc
  ),
  update(join(BINDINGS_DIR, 'wasm', 'example.html'), directUpdateFunc),
  update(join(ROOT_DIR, 'README.md'), directUpdateFunc),
  update(join(CORE_DIR, 'README.md'), directUpdateFunc),
  update(join(BINDINGS_DIR, 'native', 'decancer.h'), updateNativeHeaderFunc),
  update(join(BINDINGS_DIR, 'native', 'docs', 'Doxyfile'), directUpdateFunc),
  update(join(BINDINGS_DIR, 'node', 'README.md'), directUpdateFunc),
  update(join(CORE_DIR, 'README.md'), directUpdateFunc),
  update(join(CORE_DIR, 'src', 'lib.rs'), directUpdateFunc),
  update(join(BINDINGS_DIR, 'java', 'build.gradle'), updateGradleFunc),
  new Promise(resolve => {
    readdir(join(NODE_DIR, 'npm')).then(files => {
      Promise.all(
        files.map(file =>
          update(join(NODE_DIR, 'npm', file, 'package.json'), updateJsonFunc)
        )
      ).then(resolve)
    })
  }),
  new Promise(resolve => {
    readdir(JAVA_SOURCES_DIR).then(files => {
      Promise.all(
        files.map(file =>
          update(join(JAVA_SOURCES_DIR, file), updateDocStringVersionFunc)
        )
      ).then(resolve)
    })
  })
]))
