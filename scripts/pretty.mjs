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
const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const CORE_DIR = join(ROOT_DIR, 'core')
const BINDINGS_DIR = join(ROOT_DIR, 'bindings')
const SETUP_OUTPUTS = process.env.DECANCER_SETUP_OUTPUTS
  ? JSON.parse(process.env.DECANCER_SETUP_OUTPUTS)
  : null
const OPTIONS = options(process.argv.slice(2))

function isAffected(value) {
  return SETUP_OUTPUTS !== null
    ? SETUP_OUTPUTS.release !== 'null' ||
        SETUP_OUTPUTS[`${value}_affected`] === 'true'
    : true
}

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
  if (isAffected('core')) {
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

      const rangeSize = bin.readUint8(offset + 4) & 0x7f

      caseSensitiveCodepoints.push(
        ...Array.from({ length: rangeSize }, (_, i) => codepoint + 1 + i)
      )
      toAdd += rangeSize

      codepointsCount += toAdd
    }

    for (offset = 6; offset < codepointsEnd; offset += 6) {
      const integer = bin.readUint32LE(offset)

      const codepoint = integer & CODEPOINT_MASK
      let toAdd = (1 + bin.readUint8(offset + 4)) & 0x7f

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
}

async function prettier() {
  const extensions = ['css', 'js', 'ts', 'mjs', 'cjs', 'json']

  await execute('npm i -g prettier')

  if (isAffected('java')) {
    await execute('npm i prettier-plugin-java --save-dev')

    extensions.push('java')
  }

  await execute(`npx prettier **/*.{${extensions.join(',')}} --write`, {
    cwd: ROOT_DIR
  })

  await execute('git restore yarn.lock', { cwd: ROOT_DIR })

  console.log('- [prettier] completed prettifying files')
}

async function cargo(cwd, ty) {
  if (isAffected(ty)) {
    console.log(`- [cargo -> ${cwd}] running clippy and rustfmt...`)

    await execute('cargo fmt', { cwd })

    console.log(`- [cargo -> ${cwd}] completed`)
  }
}

async function clangFormat() {
  if (isAffected('native')) {
    const clangFormatExecutable = OPTIONS['clang-format'] ?? 'clang-format'

    console.log(`- [${clangFormatExecutable}] running...`)

    await execute(`${clangFormatExecutable} -i decancer.h`, {
      cwd: join(BINDINGS_DIR, 'native')
    })

    console.log(`- [${clangFormatExecutable}] completed`)
  }
}

void (await Promise.all([
  cargo(join(CORE_DIR), 'core'),
  cargo(join(BINDINGS_DIR, 'java'), 'java'),
  cargo(join(BINDINGS_DIR, 'node'), 'node'),
  cargo(join(BINDINGS_DIR, 'wasm'), 'wasm'),
  cargo(join(BINDINGS_DIR, 'native'), 'native'),
  clangFormat(),
  prettier(),
  updateReadme()
]))
