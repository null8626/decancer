/* eslint-disable */

'use strict'

import { readFile, writeFile } from 'node:fs/promises'
import { exec, execSync } from 'node:child_process'
import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'
import { deserialize } from 'node:v8'
import { options } from './util.mjs'

const CODEPOINT_MASK = 0xfffff
// 0..=9 | 14..=31 | 127 | 0xd800..=0xf8ff | 0xe01f0..=0x10ffff
const NONE_CODEPOINTS_COUNT = 10 + 18 + 1 + 8448 + 196112
const RANGE_MASK = 0x8000000
const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const CORE_DIR = join(ROOT_DIR, 'core')
const BINDINGS_DIR = join(ROOT_DIR, 'bindings')
const OPTIONS = options(process.argv.slice(2))

if (!existsSync(join(ROOT_DIR, '.cache.bin'))) {
  execSync(`node ${join(ROOT_DIR, 'scripts', 'update_unicode.mjs')}`, {
    stdio: 'inherit'
  })
}

const { alreadyHandledCount } = deserialize(
  readFileSync(join(ROOT_DIR, '.cache.bin'))
)

const execute = promisify(exec)

async function updateReadme() {
  console.log('- [readme] reading codepoints.bin...')

  const bin = await readFile(join(CORE_DIR, 'bin', 'codepoints.bin'))

  console.log('- [readme] parsing codepoints.bin...')

  let codepointsCount = NONE_CODEPOINTS_COUNT + alreadyHandledCount

  const codepointsEnd = bin.readUint16LE()
  const caseSensitiveCodepointsEnd = bin.readUint16LE(2)
  const caseSensitiveCodepoints = []
  let offset = codepointsEnd

  for (; offset < caseSensitiveCodepointsEnd; offset += 6) {
    const integer = bin.readUint32LE(offset)

    const codepoint = integer & CODEPOINT_MASK
    let toAdd = 1

    caseSensitiveCodepoints.push(codepoint)

    if ((integer & RANGE_MASK) !== 0) {
      const rangeUntil = bin.readUint8(offset + 4) & 0x7f

      caseSensitiveCodepoints.push(
        ...Array.from({ length: rangeUntil }, (_, i) => codepoint + 1 + i)
      )
      toAdd += rangeUntil
    }

    codepointsCount += toAdd
  }

  for (offset = 6; offset < codepointsEnd; offset += 6) {
    const integer = bin.readUint32LE(offset)

    const codepoint = integer & CODEPOINT_MASK
    let toAdd = 1

    if ((integer & RANGE_MASK) !== 0) {
      toAdd += bin.readUint8(offset + 4) & 0x7f
    }

    const uppercasedCodepoint = String.fromCodePoint(codepoint)
      .toUpperCase()
      .codePointAt()

    if (
      uppercasedCodepoint !== codepoint &&
      !caseSensitiveCodepoints.includes(uppercasedCodepoint)
    ) {
      toAdd *= 2
    }

    codepointsCount += toAdd
  }

  console.log('- [readme] reading README.md...')

  const readme = await readFile(join(CORE_DIR, 'README.md'))

  await writeFile(
    join(CORE_DIR, 'README.md'),
    readme
      .toString()
      .trim()
      .replace(
        /\*\*[\d,\.]+ \(\d+[\.\,]\d{2}%\) different unicode codepoints\*\*/,
        `**${codepointsCount.toLocaleString('en-US')} (${(
          (codepointsCount / 0x10ffff) *
          100
        ).toFixed(2)}%) different unicode codepoints**`
      )
  )

  console.log('- [readme] updated')
}

async function prettier() {
  await execute('npm i -g prettier')
  await execute('npm i prettier-plugin-java --save-dev')

  await execute('npx prettier **/*.{js,ts,mjs,cjs,json,java} --write', {
    cwd: ROOT_DIR
  })

  await execute('git restore yarn.lock', { cwd: ROOT_DIR })

  console.log('- [prettier] completed prettifying files')
}

async function cargo(cwd) {
  console.log(`- [cargo -> ${cwd}] running clippy and rustfmt...`)

  await execute('cargo fmt', { cwd })

  console.log(`- [cargo -> ${cwd}] completed`)
}

async function clangFormat() {
  const clangFormatExecutable = OPTIONS['clang-format'] ?? 'clang-format'

  console.log(`- [${clangFormatExecutable}] running...`)

  await execute(`${clangFormatExecutable} -i decancer.h`, {
    cwd: join(BINDINGS_DIR, 'native')
  })

  console.log(`- [${clangFormatExecutable}] completed`)
}

void (await Promise.all([
  cargo(join(CORE_DIR)),
  cargo(join(BINDINGS_DIR, 'java')),
  cargo(join(BINDINGS_DIR, 'node')),
  cargo(join(BINDINGS_DIR, 'wasm')),
  cargo(join(BINDINGS_DIR, 'native')),
  clangFormat(),
  prettier(),
  updateReadme()
]))
