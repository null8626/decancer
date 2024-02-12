import { readdir, rename, mkdir } from 'node:fs/promises'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')

const [artifacts] = await Promise.all([
  readdir(join(ROOT_DIR, 'bindings', 'native', 'target', 'release')),
  mkdir(join(ROOT_DIR, 'artifacts'))
])

const promises = []

for (const artifact of artifacts) {
  try {
    console.log(artifact)
    const ext = artifact.match(/\.\w+$/)[0].slice(1)

    if (ext === 'lib' || ext === 'dll' || ext === 'so' || ext === 'dylib') {
      promises.push(
        rename(
          join(ROOT_DIR, 'bindings', 'native', 'target', 'release', artifact),
          join(ROOT_DIR, 'artifacts')
        )
      )
    }
  } catch (err) {
    console.log(err.stack)
    
    continue
  }
}

if (promises.length === 0) {
  throw new Error('target directory is empty')
} else {
  void (await Promise.all(promises))
}