/* eslint-disable */

import { BINDINGS_DIR, CORE_DIR, OPTIONS, ROOT_DIR } from './constants.mjs'
import { exec } from 'node:child_process'
import { isAffected } from './util.mjs'
import { promisify } from 'node:util'
import { join } from 'node:path'

const execute = promisify(exec)

async function prettier() {
  await execute('npm i -g prettier')

  await execute(`npx prettier **/*.{css,js,ts,mjs,cjs,json} --write`, {
    cwd: ROOT_DIR
  })

  console.log('- [prettier] completed')
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

async function java() {
  if (isAffected('java')) {
    console.log('- [googlejavaformat] running...')

    await execute('chmod +x ./gradlew && ./gradlew format', {
      cwd: join(BINDINGS_DIR, 'java')
    })

    console.log('- [googlejavaformat] completed')
  }
}

void (await Promise.all([
  cargo(CORE_DIR, 'core'),
  cargo(join(BINDINGS_DIR, 'java'), 'java'),
  cargo(join(BINDINGS_DIR, 'node'), 'node'),
  cargo(join(BINDINGS_DIR, 'wasm'), 'wasm'),
  cargo(join(BINDINGS_DIR, 'native'), 'native'),
  clangFormat(),
  java(),
  execute('go fmt setup_go_binding.go', {
    cwd: join(ROOT_DIR, 'scripts')
  }),
  execute('go fmt', {
    cwd: join(BINDINGS_DIR, 'go')
  }),
  prettier()
]))
