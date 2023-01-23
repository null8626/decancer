import { appendFileSync } from 'node:fs'
import { EOL } from 'node:os'

const response = await fetch(
  'https://api.github.com/repos/null8626/decancer/compare/HEAD~1...HEAD',
  {
    headers: {
      Authorization: `Bearer ${process.env.GITHUB_TOKEN}`,
      'Content-Type': 'application/json'
    }
  }
)

const {
  base_commit: {
    commit: { message }
  },
  files
} = await response.json()

const coreAffected = files.some(
  ({ filename }) =>
    filename.startsWith('core/src/') || filename === 'core/bin/confusables.bin'
)

appendFileSync(
  process.env.GITHUB_OUTPUT,
  Object.entries({
    release: /^\d+\.\d+\.\d+$/.test(message) ? message : 'null',
    core_affected: coreAffected,
    node_affected:
      coreAffected ||
      files.some(({ filename }) => filename.startsWith('bindings/node/src')),
    wasm_affected:
      coreAffected ||
      files.some(({ filename }) => filename.startsWith('bindings/wasm/src')),
    native_affected:
      coreAffected ||
      files.some(
        ({ filename }) =>
          filename.startsWith('bindings/native/src') ||
          filename === 'bindings/native/decancer.h'
      )
  }).reduce((a, [k, v]) => `${a}${k}=${v}${EOL}`, '')
)
