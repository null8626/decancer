import { exec } from 'node:child_process'
import { appendFileSync } from 'node:fs'
import { EOL } from 'node:os'

const execute = (command, cwd) =>
  new Promise((resolve, reject) =>
    exec(command, { cwd }, (error, stderr, stdout) =>
      error ? reject(error?.stack) : resolve(stdout.toString().trim())
    )
  )

const [commit, filesChanged] = await Promise.all([
  execute('git log -1 --pretty=%B'),
  new Promise((resolve) =>
    execute('git diff --name-only HEAD~1 HEAD').then((out) => resolve(out.split(EOL)))
  )
])

const coreAffected = filesChanged.some(
  (file) => file.startsWith('core/src/') || file === 'core/bin/confusables.bin'
)

appendFileSync(
  process.env.GITHUB_OUTPUT,
  Object.entries({
    is_release: /^\d+\.\d+\.\d+$/.test(commit),
    core_affected: coreAffected,
    node_affected:
      filesChanged.some((file) => file.startsWith('bindings/node/src')),
    wasm_affected:
      filesChanged.some((file) => file.startsWith('bindings/wasm/src')),
    native_affected:
      filesChanged.some(
        (file) =>
          file.startsWith('bindings/native/src') ||
          file === 'bindings/native/decancer.h'
      ),
    readme_affected: filesChanged.includes('README.md')
  }).reduce((a, [k, v]) => `${a}${k}=${v}${EOL}`, '')
)