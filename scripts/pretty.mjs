import { readFile, writeFile } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { promisify } from 'node:util'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')

const execute = promisify(exec)

async function readme() {
  console.log('- [readme] reading confusables.bin...')

  const bin = await readFile(join(ROOT_DIR, 'core', 'bin', 'confusables.bin'))

  console.log('- [readme] parsing confusables.bin...')

  let confusablesCount = 0
  const confusablesEnd = bin.readUint16LE()
  const caseSensitiveConfusablesEnd = bin.readUint16LE(2)
  const caseSensitiveConfusables = []
  let offset = confusablesEnd

  for (; offset < caseSensitiveConfusablesEnd; offset += 5) {
    const integer = bin.readUint32LE(offset)

    if ((integer & 0x40000000) === 0 && ((integer >> 21) & 0xff) === 0) {
      continue
    }

    const codepoint = integer & 0x1fffff
    let toAdd = 1

    caseSensitiveConfusables.push(codepoint)

    if ((integer & 0x20000000) !== 0) {
      const rangeUntil = bin.readUint8(offset + 4) & 0x7f

      caseSensitiveConfusables.push(
        ...Array.from({ length: rangeUntil }, (_, i) => codepoint + 1 + i)
      )
      toAdd += rangeUntil
    }

    confusablesCount += toAdd
  }

  for (offset = 6; offset < confusablesEnd; offset += 5) {
    const integer = bin.readUint32LE(offset)

    if ((integer & 0x40000000) === 0 && ((integer >> 21) & 0xff) === 0) {
      continue
    }

    const codepoint = integer & 0x1fffff
    let toAdd = 1

    if ((integer & 0x20000000) !== 0) toAdd += bin.readUint8(offset + 4) & 0x7f

    const uppercasedCodepoint = String.fromCodePoint(codepoint)
      .toUpperCase()
      .codePointAt()

    if (
      uppercasedCodepoint !== codepoint &&
      !caseSensitiveConfusables.includes(uppercasedCodepoint)
    )
      toAdd *= 2

    confusablesCount += toAdd
  }

  console.log('- [readme] reading README.md...')

  const readme = await readFile(join(ROOT_DIR, 'README.md'))
  const sizeExponent = Math.floor(Math.log2(bin.byteLength) / 10)

  return readme
    .toString()
    .trim()
    .replace(
      /\*\*[\d,]+ different confusables\*\*/,
      `**${confusablesCount.toLocaleString()} different confusables**`
    )
    .replace(
      /customized [\d\.]+ \w?B binary file/,
      `customized ${(bin.byteLength / Math.pow(1000, sizeExponent)).toFixed(
        2
      )} ${sizeExponent > 0 ? 'KMG'[sizeExponent - 1] : ''}B binary file`
    )
}

async function librs() {
  const contents = await readFile(join(ROOT_DIR, 'core', 'src', 'lib.rs'))

  return contents.toString().replace(/\/\/!.*?\n/g, '')
}

async function updateReadme() {
  const [readmeContents, librsContents] = await Promise.all([readme(), librs()])

  void (await Promise.all([
    writeFile(join(ROOT_DIR, 'README.md'), readmeContents),
    writeFile(
      join(ROOT_DIR, 'core', 'src', 'lib.rs'),
      `${readmeContents
        .split('\n')
        .map(line => `//! ${line}`)
        .join('\n')}\n${librsContents}`
    )
  ]))

  console.log('- [readme] updated readme and lib.rs')
}

async function prettier() {
  try {
    await execute('npm list -g prettier')
  } catch {
    await execute('npm i -g prettier')
  }

  await execute('npx prettier **/*.{ts,mjs,cjs,json} --write', {
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

async function core() {
  await cargo(join(ROOT_DIR, 'core'))
  await updateReadme()
}

async function clangFormat() {
  console.log('- [clang-format] running...')

  await execute('clang-format -i decancer.h test.c', {
    cwd: join(ROOT_DIR, 'bindings', 'native')
  })

  console.log('- [clang-format] completed')
}

void (await Promise.all([
  core(),
  cargo(join(ROOT_DIR, 'bindings', 'node')),
  cargo(join(ROOT_DIR, 'bindings', 'wasm')),
  cargo(join(ROOT_DIR, 'bindings', 'native')),
  prettier(),
  clangFormat()
]))
