import { execSync } from 'node:child_process'
import { existsSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
/**
 * Promise.all but better
 * @arg {Promise[]} promises An array of `Promise`s.
 * @returns {Promise<void>}
 */
function spawnPromises(promises) {
  return new Promise((res, rej) => {
    let count = 0

    for (const promise of promises) {
      promise.then(() => {
        count++

        if (count === promises.length) {
          res()
        }
      }).catch(rej)
    }
  })
}

const __dirname = dirname(fileURLToPath(import.meta.url))

if (existsSync(resolve(__dirname, '..', 'node_modules'))) {
  process.exit()
}

console.log('[root] installing...')

await spawnPromises([
  new Promise(res => {
    console.log('[root] installing it\'s own devDependencies...')
    
    console.time('[root] done')
    
    execSync('npm i --save-dev', {
      cwd: resolve(__dirname, '..')
    })

    console.timeEnd('[root] done')
    res()
  }),

  new Promise(res => {
    console.log('[node] installing dependencies...')
    
    console.time('[node] done')
    
    execSync('npm i --save-dev', {
      cwd: resolve(__dirname, '..', 'node')
    })

    console.timeEnd('[node] done')
    res()
  }),

  new Promise(res => {
    console.log('[wasm] installing wasm-pack...')
    
    console.time('[wasm] done')
    
    execSync('cargo install wasm-pack', {
      cwd: resolve(__dirname, '..', 'wasm')
    })

    console.timeEnd('[wasm] done')
    res()
  })
])

console.log('[root] done installing everything')