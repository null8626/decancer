import { readFileSync, writeFileSync, readdirSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const update = (filename, callback) => {
  writeFileSync(filename, callback(readFileSync(filename, 'utf-8')))
  console.log(`- updated file: ${filename}`)
}

const updateTomlFunc = (x) =>
  x.replace(/version = "\d\.\d\.\d"/, `version = "${process.argv[2]}"`)

update(join(ROOT_DIR, 'core', 'Cargo.toml'), updateTomlFunc)
update(join(ROOT_DIR, 'node', 'Cargo.toml'), updateTomlFunc)
update(join(ROOT_DIR, 'wasm', 'Cargo.toml'), updateTomlFunc)
update(join(ROOT_DIR, 'native', 'Cargo.toml'), updateTomlFunc)

const updateJsonFunc = (x) => {
  const json = JSON.parse(x)
  json.version = process.argv[2]

  return JSON.stringify(json, null, 2)
}

update(join(ROOT_DIR, 'node', 'package.json'), updateJsonFunc)

for (const dir of readdirSync(join(ROOT_DIR, 'node', 'npm')))
  update(join(ROOT_DIR, 'node', 'npm', dir, 'package.json'), updateJsonFunc)

update(join(ROOT_DIR, 'README.md'), (x) =>
  x.replace(/(\d\.\d\.\d)/g, process.argv[2])
)
