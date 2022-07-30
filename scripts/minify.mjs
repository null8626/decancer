import { execSync } from 'node:child_process'
import { readFileSync, writeFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const MOD_TS = resolve(__dirname, '..', 'mod.ts')
const NEWLINE_REGEX = /\r?\n/g

const valid = execSync('git log --name-only --pretty=%b', {
  cwd: resolve(__dirname, '..')
})
  .toString()
  .trim()
  .split(NEWLINE_REGEX)
  .find(x => resolve(__dirname, '..', x) === MOD_TS)

if (!valid) {
  process.exit()
}

const form = new URLSearchParams()

form.append('type', 'js')
form.append('source', readFileSync(resolve(__dirname, '..', 'mod.js')).toString())

const response = await fetch('https://www.minifier.org/minify.php', {
  method: 'POST',
  body: form,
  headers: {
    'content-type': 'application/x-www-form-urlencoded; charset=UTF-8'
  }
})

const { minified } = await response.json()

writeFileSync(resolve(__dirname, '..', 'decancer.min.js'), minified)