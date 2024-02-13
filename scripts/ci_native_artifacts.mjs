import { readdir, rename } from 'node:fs/promises'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const TARGET = process.argv[2]
const IS_JAVA = process.argv.some(argv => argv === '--java')

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const TARGET_DIR = join(
  ROOT_DIR,
  'bindings',
  IS_JAVA ? 'java' : 'native',
  TARGET
)

const artifacts = await readdir(join(TARGET_DIR, 'release'))
const promises = []

for (const artifact of artifacts) {
  try {
    const ext = artifact.match(/\.\w+$/)[0].slice(1)

    if (ext === 'lib' || ext === 'dll' || ext === 'so' || ext === 'dylib') {
      let newPath = join(ROOT_DIR, 'artifacts', artifact.replace('.dll.lib', '.lib'))
      
      if (IS_JAVA) {
        newPath = newPath.replace('decancer', `decancer-${TARGET}`)
      }
      
      promises.push(
        rename(
          join(TARGET_DIR, 'release', artifact),
          newPath
        )
      )
    }
  } catch {
    continue
  }
}

if (promises.length === 0) {
  console.error('error: target directory is empty')
  process.exit(1)
}

void await Promise.all(promises)