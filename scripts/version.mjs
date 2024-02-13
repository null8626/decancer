import { readdir, readFile, writeFile } from 'node:fs/promises'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const CORE_DIR = join(ROOT_DIR, 'core')
const NODE_DIR = join(ROOT_DIR, 'bindings', 'node')

async function update(filename, callback) {
  await writeFile(filename, callback(await readFile(filename, 'utf-8')))
  console.log(`- updated file: ${filename}`)
}

function updateJsonFunc(str) {
  const json = JSON.parse(str)
  json.version = process.argv[2]

  return JSON.stringify(json, null, 2)
}

const updateTomlFunc = x =>
  x.replace(/version = "\d+\.\d+\.\d+"/, `version = "${process.argv[2]}"`)
const directUpdateFunc = x => x.replace(/(\d\.\d\.\d)/g, process.argv[2])

void (await Promise.all([
  update(join(CORE_DIR, 'Cargo.toml'), updateTomlFunc),
  update(join(NODE_DIR, 'Cargo.toml'), updateTomlFunc),
  update(join(ROOT_DIR, 'bindings', 'wasm', 'Cargo.toml'), updateTomlFunc),
  update(join(ROOT_DIR, 'bindings', 'native', 'Cargo.toml'), updateTomlFunc),
  update(join(NODE_DIR, 'package.json'), updateJsonFunc),
  update(
    join(ROOT_DIR, 'bindings', 'wasm', 'bin', 'decancer.min.js'),
    directUpdateFunc
  ),
  update(join(ROOT_DIR, 'index.html'), directUpdateFunc),
  update(join(CORE_DIR, 'README.md'), directUpdateFunc),
  update(join(CORE_DIR, 'src', 'lib.rs'), directUpdateFunc),
  new Promise(resolve => {
    readdir(join(NODE_DIR, 'npm')).then(files => {
      Promise.all(
        files.map(file =>
          update(join(NODE_DIR, 'npm', file, 'package.json'), updateJsonFunc)
        )
      ).then(resolve)
    })
  })
]))
