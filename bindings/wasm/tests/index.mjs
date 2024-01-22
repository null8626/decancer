import { Worker } from 'node:worker_threads'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'
import puppeteer from 'puppeteer'

const CURRENT_DIR = join(dirname(fileURLToPath(import.meta.url)))

function error(message) {
  process.exitCode = 1
  console.error(message)
}

console.log('- [client] running worker...')

const server = new Worker(join(CURRENT_DIR, 'server.mjs'))

server.on('message', async message => {
  switch (message.code) {
    case 'ready':
      console.log('- [client] launching browser...')
      let browser = null

      for (let tries = 0; ; tries++) {
        try {
          browser = await puppeteer.launch({
            headless: 'new',
            timeout: 12500
          })

          break
        } catch (err) {
          console.log(
            `- [client] failed to launch brower after ${tries} tries.`
          )

          if (tries === 5) {
            error(
              '- [client] aborting browser launching process due to error:\n${err.stack}'
            )

            return server.postMessage(null)
          }
        }
      }

      console.log('- [client] launching browser page...')
      const page = await browser.newPage()

      console.log('- [client] requesting to localhost:8080...')
      await page.goto('http://localhost:8080', {
        waitFor: 'load'
      })

      console.log('- [client] running tests...')
      const err = await page.evaluate(async () => {
        class TestContext {
          #err
          #object

          constructor(object) {
            this.#err = null
            this.#object = object
          }

          test(expected, functionName, ...args) {
            if (this.#err === null) {
              const received = this.#object[functionName](...args)

              if (received !== expected) {
                this.#err = {
                  expected,
                  received,
                  functionName
                }
              }
            }

            return this
          }

          finish() {
            return this.#err
          }
        }

        try {
          const decancer = await window.init({
            local: true
          })

          return new TestContext(decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣'))
            .test(true, 'equals', 'very funny text')
            .test(true, 'startsWith', 'very')
            .test(true, 'endsWith', 'text')
            .test(true, 'contains', 'funny')
            .test('very funny text', 'toString')
            .finish()
        } catch (err) {
          return err.stack
        }
      })

      if (err !== null) {
        if (typeof err === 'string') {
          error(
            `- [client] error while loading wasm binary:\n${decodeURIComponent(
              err
            )}`
          )
        } else {
          error(
            `- [client] assertion error while calling ${err.functionName}: expected '${err.expected}', got '${err.received}'`
          )
        }
      } else {
        console.log('- [client] tests were successful.')
      }

      console.log('- [client] closing browser...')

      await browser.close()
      server.postMessage(null)

      break

    case 'error':
      error(`- [client] error while starting server:\n${message.stack}`)

      break

    case 'close':
      server.terminate()
  }
})
