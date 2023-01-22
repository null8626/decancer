import { appendFileSync } from 'node:fs'
import { EOL } from 'node:os'

const response = await fetch('https://api.github.com/repos/null8626/decancer/compare/HEAD~1...HEAD', {
  headers: {
	Authorization: `Bearer ${process.env.GITHUB_TOKEN}`,
	'Content-Type': 'application/json'
  }
})

const { base_commit: { message }, files } = await response.json()

console.log(
  Object.entries({
    is_release: /^\d+\.\d+\.\d+$/.test(message),
    core_affected: coreAffected,
    node_affected:
      coreAffected ||
      files.some(({ filename }) => filename.startsWith('bindings/node/src')),
    wasm_affected:
      coreAffected ||
      files.some(({ filename }) => filename.startsWith('bindings/wasm/src')),
    native_affected:
      coreAffected ||
      filesChanged.some(
        ({ filename }) =>
          filename.startsWith('bindings/native/src') ||
          filename === 'bindings/native/decancer.h'
      ),
    readme_affected: filename.some(({ filename }) => filename === 'README.md')
  }).reduce((a, [k, v]) => `${a}${k}=${v}${EOL}`, '')
)

appendFileSync(
  process.env.GITHUB_OUTPUT,
  Object.entries({
    is_release: /^\d+\.\d+\.\d+$/.test(commit),
    core_affected: coreAffected,
    node_affected:
      coreAffected ||
      filesChanged.some((file) => file.startsWith('bindings/node/src')),
    wasm_affected:
      coreAffected ||
      filesChanged.some((file) => file.startsWith('bindings/wasm/src')),
    native_affected:
      coreAffected ||
      filesChanged.some(
        (file) =>
          file.startsWith('bindings/native/src') ||
          file === 'bindings/native/decancer.h'
      ),
    readme_affected: filesChanged.includes('README.md')
  }).reduce((a, [k, v]) => `${a}${k}=${v}${EOL}`, '')
)
