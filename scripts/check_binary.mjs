import { exec } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const execute = promisify(exec)

async function sha(file) {
  console.log(`- retrieving sha256sum for ${file}...`)
  const [out] = (await execute(`sha256sum ${file}`)).split(' ')
  
  console.log(`- retrieved sha256sum for ${file}: ${out}`)
  return out
}

const [ogBinarySha] = await Promise.all([
  sha(join(ROOT_DIR, 'core', 'bin', 'confusables.bin')),
  execute(`node ${join(ROOT_DIR, 'scripts', 'decode.mjs')} ${join(ROOT_DIR, 'core', 'bin', 'confusables.bin')}`, { cwd: ROOT_DIR })
])

const [ogJsonSha] = await Promise.all([
  sha(join(ROOT_DIR, 'output.json')),
  execute(`node ${join(ROOT_DIR, 'scripts', 'encode.mjs')} ${join(ROOT_DIR, 'core', 'bin', 'output.json')}`, { cwd: ROOT_DIR })
])

const [newBinarySha] = await Promise.all([
  sha(join(ROOT_DIR, 'output.bin')),
  execute(`node ${join(ROOT_DIR, 'scripts', 'decode.mjs')} ${join(ROOT_DIR, 'output.bin')}`, { cwd: ROOT_DIR })
])

if (ogBinarySha !== newBinarySha) {
  console.error(`- error: different binary sha256sum detected - the binary must have corruption like your country\n- original: ${ogBinarySha}\n- new: ${newBinarySha}`)
  process.exit(1)
}

const newJsonSha = await sha(join(ROOT_DIR, 'output.json'))

if (ogJsonSha !== newBinarySha) {
  console.error(`- error: different json sha256sum detected - the binary must have corruption like your country\n- original: ${ogJsonSha}\n- new: ${newJsonSha}`)
  process.exit(1)
}