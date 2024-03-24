import { parentPort } from 'node:worker_threads'
import fastifyStatic from '@fastify/static'
import { createReadStream } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import fastify from 'fastify'

const CURRENT_DIR = dirname(fileURLToPath(import.meta.url))

console.log('- [server] running...')

const app = fastify()

app.register(fastifyStatic, {
  root: join(CURRENT_DIR, '..', 'bin')
})

app.get('/', (req, res) => {
  console.log('- [server] received a request.')
  res.type('text/html').send(createReadStream(join(CURRENT_DIR, 'index.html')))
})

app.listen(
  {
    port: 8080
  },
  err => {
    if (err) {
      parentPort.postMessage({
        code: 'error',
        stack: err.stack
      })
    } else {
      console.log('- [server] ready.')
      parentPort.postMessage({
        code: 'ready'
      })

      parentPort.on('message', () => {
        console.log('- [server] closing...')

        app.close().finally(() =>
          parentPort.postMessage({
            code: 'close'
          })
        )
      })
    }
  }
)
