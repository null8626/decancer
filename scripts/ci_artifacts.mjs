import { readdir, rename, mkdir } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const execute = promisify(exec)

const [artifacts] = await Promise.all([
  readdir(join(ROOT_DIR, 'artifacts')),
  mkdir(join(ROOT_DIR, 'bindings', 'node', 'artifacts'))
])

void (await Promise.all(
  artifacts.map(artifact =>
    artifact.startsWith('native-')
      ? execute(`zip ../decancer-${artifact.slice(7)}.zip ./${artifact}/*`, {
          cwd: join(ROOT_DIR, 'artifacts')
        })
      : new Promise(resolve => {
          const artifactsDir = join(
            join(ROOT_DIR, 'bindings', 'node', 'artifacts'),
            artifact.replace(/^node-/, 'bindings-')
          )
          const originDir = join(ROOT_DIR, 'artifacts', artifact)

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
))
