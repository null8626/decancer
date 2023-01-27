import { readFile, writeFile } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { promisify } from 'node:util'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const NON_BINARY_CONFUSABLES_COUNT = 181

const PRETTIERRC = JSON.stringify({
  semi: false,
  singleQuote: true,
  trailingComma: 'none',
  arrowParens: 'avoid',
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

const execute = promisify(exec)

function retrieveReadmePromise(resolve) {
  console.log('- [readme] reading confusables.bin...')

  readFile(join(ROOT_DIR, 'core', 'bin', 'confusables.bin')).then(bin => {
    console.log('- [readme] parsing confusables.bin...')

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
      )
        toAdd *= 2

      confusablesCount += toAdd
    }

    console.log('- [readme] reading README.md...')
    readFile(join(ROOT_DIR, 'README.md')).then(readme =>
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
  readFile(join(ROOT_DIR, 'core', 'src', 'lib.rs')).then(libRs => {
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
        `${readme
          .split('\n')
          .map(line => `//! ${line}`)
          .join('\n')}\n${libRs}`
      )
    ]).then(() => {
      console.log('- [readme] updated readme and lib.rs')
      resolve()
    })
  )
}

function prettierPromise(resolve) {
  console.log('- [prettier] setting up prettier...')

  Promise.all([
    execute('npm i prettier', { cwd: ROOT_DIR }),
    writeFile(join(ROOT_DIR, '.prettierrc.json'), PRETTIERRC),
    writeFile(join(ROOT_DIR, '.prettierignore'), PRETTIERIGNORE)
  ]).then(() => {
    execute('npx prettier **/*.{ts,mjs,cjs,json} --write', {
      cwd: ROOT_DIR
    }).then(() =>
      execute('git restore yarn.lock', { cwd: ROOT_DIR }).then(() => {
        console.log('- [prettier] completed prettifying files')
        resolve()
      })
    )
  })
}

async function handleCargo(cwd) {
  console.log(`- [cargo -> ${cwd}] running clippy and rustfmt...`)

  await execute('cargo clippy --fix --allow-dirty', { cwd })
  await execute('cargo fmt', { cwd })

  console.log(`- [cargo -> ${cwd}] completed`)
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
  new Promise(resolve => {
    console.log('- [clang-format] running...')

    execute('clang-format -i decancer.h test.c', {
      cwd: join(ROOT_DIR, 'bindings', 'native')
    }).then(() => {
      console.log('- [clang-format] completed')
      resolve()
    })
  })
]))
