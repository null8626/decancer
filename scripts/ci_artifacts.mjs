import { readdir, rename, mkdir } from 'node:fs'
import { exec } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const NODE_ARTIFACTS = join(ROOT_DIR, 'bindings', 'node', 'artifacts')
const ARTIFACTS = join(ROOT_DIR, 'artifacts')

const execute = (command, cwd) =>
  new Promise((resolve, reject) =>
    exec(command, { cwd }, (error) =>
      error ? reject(error?.stack) : resolve()
    )
  )

const [artifacts] = await Promise.all([
  new Promise((resolve, reject) => readdir(ARTIFACTS, (err, data) => err ? reject(err.stack) : resolve(data))),
  new Promise(resolve => mkdir(NODE_ARTIFACTS, resolve))
])

void await Promise.all(artifacts.map(artifact => {
  if (artifact.startsWith('native-')) {
	return execute(`zip ../decancer-${artifact.slice(7)}.zip ./${artifact}/*.node`, ARTIFACTS)
  } else {
	return new Promise(resolve => {
	  const artifactsDir = join(NODE_ARTIFACTS, artifact.replace(/^node\-/, 'bindings-'))
	  const originDir = join(ARTIFACTS, artifact)
	  
	  Promise.all([
	    new Promise(resolve2 => readdir(originDir, resolve2)),
	    new Promise(resolve2 => mkdir(artifactsDir, resolve2))
	  ]).then(([[nodeBinary]]) => mkdir(artifactsDir, () => rename(join(originDir, nodeBinary), join(artifactsDir, nodeBinary), resolve)))
	})
  }
}))