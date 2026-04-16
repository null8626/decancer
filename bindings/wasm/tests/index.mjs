// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

import { BINDINGS_DIR } from '../../../scripts/constants.mjs'
import { Worker } from 'node:worker_threads'
import { readFileSync } from 'node:fs'
import process from 'node:process'
import puppeteer from 'puppeteer'
import { join } from 'node:path'

const CURRENT_DIR = join(BINDINGS_DIR, 'wasm', 'tests')
const RETAIN_DATA = JSON.parse(
  readFileSync(join(CURRENT_DIR, 'retain_data.json'))
)

function error(message) {
  process.exitCode = 1
  console.error(message)
}

console.log('- [client] running worker...')

const server = new Worker(join(CURRENT_DIR, 'server.mjs'))

server.on('message', async message => {
  switch (message.code) {
    case 'ready': {
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
          error(`- [client] failed to launch brower after ${tries} tries.`)

          if (tries === 5) {
            error(
              `- [client] aborting browser launching process due to error:\n${err.stack}`
            )

            return server.postMessage(null)
          }
        }
      }

      console.log('- [client] launching browser page...')
      const page = await browser.newPage()

      page.on('console', msg =>
        console.log(`- [client] console: ${msg.text()}`)
      )
      page.on('pageerror', err =>
        error(`- [client] error while testing wasm binding:\n${err.stack}`)
      )

      console.log('- [client] requesting to localhost:8080...')
      await page.goto('http://localhost:8080', {
        waitFor: 'load'
      })

      console.log('- [client] running tests...')
      const err = await page.evaluate(async retainData => {
        let decancer

        class TestContext {
          #err
          #object

          constructor(object) {
            this.#err = null
            this.#object = object
          }

          #assert(received, expected, functionName) {
            if (this.#err === null && received !== expected) {
              this.#err = {
                expected,
                received,
                functionName
              }
            }
          }

          test(expected, functionName, ...args) {
            if (this.#err === null) {
              this.#assert(
                this.#object[functionName](...args),
                expected,
                functionName
              )
            }

            return this
          }

          testModifications() {
            if (this.#err === null) {
              this.#object.replace('text', 'other')
              this.#assert(
                this.#object.toString(),
                'very funny other',
                true,
                'replace'
              )

              this.#object.replaceMultiple(['very ', ' funny'], 'asdf')
              this.#assert(
                this.#object.toString(),
                'asdf other',
                true,
                'replaceMultiple'
              )

              this.#object.censor('asdf', '*')
              this.#assert(
                this.#object.toString(),
                '**** other',
                true,
                'censor'
              )

              this.#object.censorMultiple(['**** ', ' other'], '*')
              this.#assert(
                this.#object.toString(),
                '**********',
                true,
                'censorMultiple'
              )
            }

            return this
          }

          testFind() {
            if (this.#err === null) {
              const match = this.#object.find('funny')

              this.#assert(match.length, 1, 'find:match.length')
              this.#assert(match[0].start, 5, 'find:match[0].start')
              this.#assert(match[0].end, 10, 'find:match[0].end')
              this.#assert(
                match[0].toString(),
                'funny',
                'find:match[0].toString()'
              )

              const matches = this.#object.findMultiple(['very ', ' funny'])

              this.#assert(matches.length, 1, 'findMultiple:matches.length')
              this.#assert(matches[0].start, 0, 'findMultiple:matches[0].start')
              this.#assert(matches[0].end, 10, 'findMultiple:matches[0].end')
              this.#assert(
                matches[0].toString(),
                'very funny',
                'findMultiple:matches[0].toString()'
              )
            }

            return this
          }

          testRetain() {
            if (this.#err === null) {
              for (const [option, testString] of Object.entries(retainData)) {
                let cured = decancer(testString, {
                  [option]: true,
                  disableBidi: true
                })

                this.#assert(cured.equals(testString), true, option)

                cured = decancer(testString)

                this.#assert(cured.equals(testString), false, `!${option}`)
              }
            }

            return this
          }

          testRetainCapitalization() {
            if (this.#err === null) {
              const cured = decancer('decÁncer', {
                retainCapitalization: true
              })

              this.#assert(cured.toString(), 'decAncer', 'retainCapitalization')
            }

            return this
          }

          testLeetspeak() {
            if (this.#err === null) {
              let cured = decancer('|-|3|_I_0', {
                disableLeetspeak: true
              })

              this.#assert(
                cured.equals('hello'),
                false,
                'disableLeetspeak:option'
              )

              cured.disableLeetspeak(false)
              cured.disableAlphabeticalLeetspeak(true)

              this.#assert(
                cured.equals('helI_o'),
                true,
                'disableAlphabeticalLeetspeak:method'
              )

              cured = decancer('|-|3|_I_0', {
                disableAlphabeticalLeetspeak: true
              })

              this.#assert(
                cured.equals('helI_o'),
                true,
                'disableAlphabeticalLeetspeak:option'
              )

              cured.disableLeetspeak(true)
              cured.disableAlphabeticalLeetspeak(false)

              this.#assert(
                cured.equals('hello'),
                false,
                'disableLeetspeak:method'
              )
            }

            return this
          }

          finish() {
            return this.#err
          }
        }

        try {
          decancer = await window.init({
            local: true
          })

          return new TestContext(decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣'))
            .test(true, 'equals', 'very funny text')
            .test(true, 'startsWith', 'very')
            .test(true, 'endsWith', 'text')
            .test(true, 'contains', 'funny')
            .test('very funny text', 'toString')
            .testFind()
            .testModifications()
            .testRetain()
            .testRetainCapitalization()
            .testLeetspeak()
            .finish()
        } catch (err) {
          return err.stack
        }
      }, RETAIN_DATA)

      if (err !== null) {
        if (typeof err === 'string') {
          error(
            `- [client] error while testing wasm binding:\n${decodeURIComponent(
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
    }

    case 'error': {
      error(`- [client] error while starting server:\n${message.stack}`)

      break
    }

    case 'close': {
      server.terminate()
    }
  }
})
