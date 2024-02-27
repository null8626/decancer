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

const { files } = await response.json()

const coreAffected = files.some(
  ({ filename }) =>
    filename.startsWith('core/src/') || filename === 'core/bin/codepoints.bin'
)

appendFileSync(
  process.env.GITHUB_OUTPUT,
  Object.entries({
    release: /^\d+\.\d+\.\d+$/.test(process.env.COMMIT_MESSAGE)
      ? process.env.COMMIT_MESSAGE
      : 'null',
    core_affected: coreAffected,
    node_affected: files.some(({ filename }) =>
      filename.startsWith('bindings/node/src')
    ),
    wasm_affected:
      coreAffected ||
      files.some(({ filename }) => filename.startsWith('bindings/wasm/')),
    native_affected: files.some(
      ({ filename }) =>
        filename.startsWith('bindings/native/src') ||
        filename === 'bindings/native/decancer.h'
    ),
    java_affected:
      coreAffected ||
      files.some(({ filename }) => filename.startsWith('bindings/java/src'))
  }).reduce((a, [k, v]) => `${a}${k}=${v}${EOL}`, '')
)
