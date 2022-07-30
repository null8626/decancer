import { execSync } from 'node:child_process'
import { createReadStream, createWriteStream, readFileSync, writeFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const NEWLINE_REGEX = /\r?\n/g
const LIB_RS_PATH = resolve(__dirname, '..', 'core', 'src', 'lib.rs')

const readmes = [
  resolve(__dirname, '..', 'README.md'),
  resolve(__dirname, '..', 'node', 'README.md')
]

const files = execSync('git log --name-only --pretty=%b', {
  cwd: resolve(__dirname, '..')
})
  .toString()
  .trim()
  .split(NEWLINE_REGEX)
  .filter(x => x.endsWith(".md"))
  .map(x => resolve(__dirname, '..', x))

if (files.length !== 1) {
  process.exit()
}

const copied = readmes
  .filter(x => x !== files[0])
  .map(x => createWriteStream(x))

const libRs = readFileSync(LIB_RS_PATH)
  .toString()
  .split(NEWLINE_REGEX)
  .filter(x => !x.startsWith('//!'))
  .join('\n')
  .trim()

let readmeComment = ''

await new Promise(resolve => {
  const read = createReadStream(files[0])

  read.on('data', chunk => {
    copied.forEach(stream => stream.write(chunk))
    readmeComment += chunk.toString()
  })

  read.once('end', () => {
    writeFileSync(LIB_RS_PATH, `//! ${readmeComment.trim().replace(NEWLINE_REGEX, x => x + '//! ')}\n${libRs}`)
    resolve()
  })
})