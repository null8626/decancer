import { readdir, rename } from 'node:fs/promises'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const TARGET = process.argv[2]
const IS_MOVE = process.argv.some(argv => argv === '--move')
const IS_JAVA = process.argv.some(argv => argv === '--java')

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const TARGET_DIR = join(
  ROOT_DIR,
  'bindings',
  IS_JAVA ? 'java' : 'native',
  TARGET
)
const ARTIFACTS_DIR = join(ROOT_DIR, 'artifacts')

if (IS_MOVE) {
  const artifacts = await readdir(join(TARGET_DIR, 'release'))
  const promises = []

<<<<<<< HEAD
  for (const artifact of artifacts) {
    try {
      const ext = artifact.match(/\.\w+$/)[0].slice(1)

      if (ext === 'lib' || ext === 'dll' || ext === 'so' || ext === 'dylib') {
        promises.push(
          rename(
            join(TARGET_DIR, 'release', artifact),
            join(ARTIFACTS_DIR, artifact.replace('.dll.lib', '.lib'))
          )
=======
const [artifacts] = await Promise.all([
  readdir(join(ROOT_DIR, 'bindings', 'native', process.argv[2], 'release')),
  mkdir(join(ROOT_DIR, 'artifacts'))
])

const promises = []

for (const artifact of artifacts) {
  try {
    const ext = artifact.match(/\.\w+$/)[0].slice(1)

    if (ext === 'lib' || ext === 'dll' || ext === 'so' || ext === 'dylib') {
      promises.push(
        rename(
          join(
            ROOT_DIR,
            'bindings',
            'native',
            process.argv[2],
            'release',
            artifact
          ),
          join(ROOT_DIR, 'artifacts', artifact.replace('.dll.lib', '.lib'))
>>>>>>> 8eebe0798723e40034af6216ad47c419ff0bc11c
        )
      }
    } catch {
      continue
    }
<<<<<<< HEAD
  }

  if (promises.length === 0) {
    console.error('error: target directory is empty')
    process.exit(1)
=======
  } catch {
    continue
>>>>>>> 8eebe0798723e40034af6216ad47c419ff0bc11c
  }
}

if (IS_JAVA) {
  const binaries = await readdir(ARTIFACTS_DIR)

  void (await Promise.all(
    binaries.map(binary =>
      rename(
        join(ARTIFACTS_DIR, binary),
        join(ARTIFACTS_DIR, binary.replace('decancer', `decancer-${TARGET}`))
      )
    )
  ))
}
