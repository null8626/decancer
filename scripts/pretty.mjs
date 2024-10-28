import { readFile, writeFile } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'

const CODEPOINT_MASK = 0xfffff
const NOT_INCLUDED_COUNT = 0x22210
const RANGE_MASK = 0x8000000
const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const STRING_TRANSLATION_MASK = 0x10000000

const execute = promisify(exec)

async function updateReadme() {
  console.log('- [readme] reading codepoints.bin...')

  const bin = await readFile(join(ROOT_DIR, 'core', 'bin', 'codepoints.bin'))

  console.log('- [readme] parsing codepoints.bin...')

  let codepointsCount = NOT_INCLUDED_COUNT
  let confusablesCount = 0

  const codepointsEnd = bin.readUint16LE()
  const caseSensitiveCodepointsEnd = bin.readUint16LE(2)
  const caseSensitiveCodepoints = []
  let offset = codepointsEnd

  for (; offset < caseSensitiveCodepointsEnd; offset += 5) {
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

    if (
      (integer & STRING_TRANSLATION_MASK) !== 0 ||
      ((integer >> 20) & 0x7f) !== 0
    ) {
      confusablesCount += toAdd
    }

    codepointsCount += toAdd
  }

  for (offset = 6; offset < codepointsEnd; offset += 5) {
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

    if (
      (integer & STRING_TRANSLATION_MASK) !== 0 ||
      ((integer >> 20) & 0x7f) !== 0
    ) {
      confusablesCount += toAdd
    }

    codepointsCount += toAdd
  }

  console.log('- [readme] reading README.md...')

  const readme = await readFile(join(ROOT_DIR, 'core', 'README.md'))
  const sizeExponent = Math.floor(Math.log2(bin.byteLength) / 10)

  await writeFile(
    join(ROOT_DIR, 'core', 'README.md'),
    readme
      .toString()
      .trim()
      .replace(
        /\*\*[\d,]+ \(\d+\.\d{2}%\) different unicode codepoints\*\*/,
        `**${codepointsCount.toLocaleString()} (${(
          (codepointsCount / 0x10ffff) *
          100
        ).toFixed(2)}%) different unicode codepoints**`
      )
      .replace(
        /\*\*[\d,]+ different unicode confusables\*\*/,
        `**${confusablesCount.toLocaleString()} different unicode confusables**`
      )
      .replace(
        /customized [\d\.]+ \w?B binary file/,
        `customized ${(bin.byteLength / Math.pow(1000, sizeExponent)).toFixed(
          2
        )} ${sizeExponent > 0 ? 'KMG'[sizeExponent - 1] : ''}B binary file`
      )
  )

  console.log('- [readme] updated')
}

async function prettier() {
  try {
    await execute('npm list -g prettier')
  } catch {
    await execute('npm i -g prettier')
  }

  await execute('npx prettier **/*.{js,ts,mjs,cjs,json} --write', {
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
  console.log('- [clang-format] running...')

  await execute('clang-format -i decancer.h test.c', {
    cwd: join(ROOT_DIR, 'bindings', 'native')
  })

  console.log('- [clang-format] completed')
}

void (await Promise.all([
  cargo(join(ROOT_DIR, 'core')),
  cargo(join(ROOT_DIR, 'bindings', 'node')),
  cargo(join(ROOT_DIR, 'bindings', 'wasm')),
  cargo(join(ROOT_DIR, 'bindings', 'native')),
  clangFormat(),
  prettier(),
  updateReadme()
]))
