import { readdir, readFile, writeFile } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const NON_BINARY_CONFUSABLES_COUNT = 181
const PRETTIERRC = JSON.stringify({
  semi: false,
  singleQuote: true,
  trailingComma: 'none',
  allowParens: 'avoid',
  htmlWhitespaceSensitivity: 'ignore'
})
const PRETTIERIGNORE = `
**/target/**
**/node_modules/**
**/package-lock.json
bindings/node/src/lib.js
bindings/node/src/lib.d.ts
bindings/node/index.js
bindings/node/index.d.ts
bindings/wasm/pkg/**
.prettierrc.json
`.trim()

const execute = (command, cwd) =>
  new Promise((resolve, reject) =>
    exec(command, { cwd }, (error) =>
      error ? reject(error?.stack) : resolve()
    )
  )

function retrieveReadmePromise(resolve) {
  readFile(join(ROOT_DIR, 'core', 'bin', 'confusables.bin')).then((bin) => {
    let confusablesCount = NON_BINARY_CONFUSABLES_COUNT
    const confusablesEnd = bin.readUint16LE()

    for (let offset = 4; offset < confusablesEnd; offset += 5) {
      const integer = bin.readUint32LE(offset)
      const codepoint = integer & 0x1fffff
      let toAdd = 1

      if ((integer & 0x10000000) !== 0) {
        const secondByte = bin.readUint8(offset + 4)

        toAdd += secondByte & 0x7f
      }

      if (
        (integer & 0x40000000) === 0 &&
        String.fromCodePoint(codepoint).toUpperCase().codePointAt() !==
          codepoint
      ) {
        toAdd *= 2
      }

      confusablesCount += toAdd
    }

    readFile(join(ROOT_DIR, 'README.md')).then((readme) =>
      resolve(
        readme
          .toString()
          .trim()
          .replace(
            /\*\*[\d,]+ different confusables\*\*/,
            `**${confusablesCount.toLocaleString()} different confusables**`
          )
      )
    )
  })
}

function retrieveLibRsPromise(resolve) {
  readFile(join(ROOT_DIR, 'core', 'src', 'lib.rs')).then((libRs) => {
    resolve(libRs.toString().replace(/\/\/!.*?\n/g, ''))
  })
}

function updateReadmePromise(resolve) {
  Promise.all([
    new Promise(retrieveReadmePromise),
    new Promise(retrieveLibRsPromise)
  ]).then(([readme, libRs]) =>
    Promise.all([
      writeFile(join(ROOT_DIR, 'README.md'), readme),
      writeFile(
        join(ROOT_DIR, 'core', 'src', 'lib.rs'),
        readme
          .split('\n')
          .map((line) => `//! ${line}`)
          .join('\n') + '\n' + libRs
      )
    ]).then(resolve)
  )
}

function prettierPromise(resolve) {
  Promise.all([
    execute('npm i prettier', ROOT_DIR),
    writeFile(join(ROOT_DIR, '.prettierrc.json'), PRETTIERRC),
    writeFile(join(ROOT_DIR, '.prettierignore'), PRETTIERIGNORE)
  ]).then(() => {
    Promise.all([
      execute('npx prettier **/*.ts --write', ROOT_DIR),
      execute('npx prettier **/*.mjs --write', ROOT_DIR),
      execute('npx prettier **/*.cjs --write', ROOT_DIR),
      execute('npx prettier **/*.json --write', ROOT_DIR)
    ]).then(resolve)
  })
}

async function handleCargo(cwd) {
  await execute('cargo clippy --fix --allow-dirty', cwd)
  await execute('cargo fmt', cwd)
}

async function handleCore() {
  await handleCargo(join(ROOT_DIR, 'core'))
  await new Promise(updateReadmePromise)
}

void (await Promise.all([
  handleCore(),
  handleCargo(join(ROOT_DIR, 'bindings', 'node')),
  handleCargo(join(ROOT_DIR, 'bindings', 'wasm')),
  handleCargo(join(ROOT_DIR, 'bindings', 'native')),
  new Promise(prettierPromise),
  Promise.all([
    execute('clang-format -i decancer.h', join(ROOT_DIR, 'bindings', 'native')),
    execute('clang-format -i test.cpp', join(ROOT_DIR, 'bindings', 'native'))
  ])
]))
