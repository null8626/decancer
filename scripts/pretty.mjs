/* eslint-disable */

'use strict'

import { isAffected, options } from './util.mjs'
import { dirname, join } from 'node:path'
import { exec } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const CORE_DIR = join(ROOT_DIR, 'core')
const BINDINGS_DIR = join(ROOT_DIR, 'bindings')

const OPTIONS = options(process.argv.slice(2))

const execute = promisify(exec)

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
  execute('go fmt setup_go_binding.go', {
    cwd: join(ROOT_DIR, 'scripts')
  }),
  execute('go fmt', {
    cwd: join(BINDINGS_DIR, 'go')
  }),
  prettier()
]))
