// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

const { strict, strictEqual } = require('node:assert')
const { describe, it } = require('node:test')
const retainData = require('./retain_data.json')
const decancer = require('./src/lib.js')

class TestContext {
  #inner

  constructor(result) {
    this.#inner = result
  }

  test(functionName, expected, ...args) {
    it(functionName, () =>
      strictEqual(this.#inner[functionName](...args), expected)
    )

    return this
  }

  testFind() {
    it('find', () => {
      const match = this.#inner.find('funny')

      strictEqual(match.length, 1)
      strictEqual(match[0].start, 5)
      strictEqual(match[0].end, 10)
      strictEqual(match[0].toString(), 'funny')
    })

    return this
  }
}

describe('cure', () => {
  new TestContext(decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣'))
    .test('equals', true, 'very funny text')
    .test('startsWith', true, 'very')
    .test('endsWith', true, 'text')
    .test('contains', true, 'funny')
    .test('toString', 'very funny text')
    .testFind()
})

it('retain', () => {
  for (const [option, testString] of Object.entries(retainData)) {
    let cured = decancer(testString, {
      [option]: true,
      disableBidi: true
    })

    strict(cured.equals(testString))

    cured = decancer(testString)

    strict(!cured.equals(testString))
  }
})

it('retain capitalization', () => {
  const cured = decancer('decÁncer', {
    retainCapitalization: true
  })

  strictEqual(cured.toString(), 'decAncer')
})

it('disable leetspeak', () => {
  let cured = decancer('|-|3|_I_0', {
    disableLeetspeak: true
  })

  strict(!cured.equals('hello'))

  cured.disableLeetspeak(false)
  cured.disableAlphabeticalLeetspeak(true)

  strict(cured.equals('helI_o'))

  cured = decancer('|-|3|_I_0', {
    disableAlphabeticalLeetspeak: true
  })

  strict(cured.equals('helI_o'))

  cured.disableLeetspeak(true)
  cured.disableAlphabeticalLeetspeak(false)

  strict(!cured.equals('hello'))
})
