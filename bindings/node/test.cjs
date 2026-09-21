// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

const { strict, strictEqual } = require('node:assert')
const { describe, it } = require('node:test')
const retainData = require('./retain_data.json')
const decancer = require('./src/lib.js')

class TestContext {
  #input
  #inner

  constructor() {
    this.#input = 'vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣'
    this.#inner = decancer(this.#input)
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

  testSuggestions() {
    it('suggestions', () => {
      const inputs = [...this.#input]
      const cured = this.#inner.toString()
      const suggestions = this.#inner.getSuggestions()

      for (
        let i = 0, oldIndex = 0;
        i < cured.length;
        oldIndex += Buffer.from(inputs[i], 'utf8').length, i++
      ) {
        const suggestion = suggestions[i]

        strictEqual(suggestion.oldIndex, oldIndex)
        strictEqual(suggestion.newIndex, i)
        strictEqual(suggestion.translation, cured[i])
      }
    })

    return this
  }
}

describe('cure', () => {
  new TestContext()
    .test('equals', true, 'very funny text')
    .test('startsWith', true, 'very')
    .test('endsWith', true, 'text')
    .test('contains', true, 'funny')
    .test('toString', 'very funny text')
    .testFind()
    .testSuggestions()
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
