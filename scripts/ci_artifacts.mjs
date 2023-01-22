import { readdir, rename, mkdir } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { promisify } from 'node:util'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const NODE_ARTIFACTS = join(ROOT_DIR, 'bindings', 'node', 'artifacts')
const ARTIFACTS = join(ROOT_DIR, 'artifacts')

const execute = promisify(exec)

const [artifacts] = await Promise.all([
  readdir(ARTIFACTS),
  mkdir(NODE_ARTIFACTS)
])

await Promise.all(
  artifacts.map(artifact =>
    artifact.startsWith('native-')
      ? execute(`zip ../decancer-${artifact.slice(7)}.zip ./${artifact}/*`, {
          cwd: ARTIFACTS
        })
      : new Promise(resolve => {
          const artifactsDir = join(
            NODE_ARTIFACTS,
            artifact.replace(/^node-/, 'bindings-')
          )
          const originDir = join(ARTIFACTS, artifact)

          Promise.all([readdir(originDir), mkdir(artifactsDir)]).then(
            ([[nodeBinary]]) =>
              mkdir(artifactsDir).then(() =>
                rename(
                  join(originDir, nodeBinary),
                  join(artifactsDir, nodeBinary)
                ).then(resolve)
              )
          )
        })
  )
)
