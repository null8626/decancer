const { strictEqual } = require('node:assert')
const { describe, it } = require('node:test')

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
}

describe('decancer', () => {
  const decancer = require('./src/lib.js')

  new TestContext(decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣'))
    .test('equals', true, 'very funny text')
    .test('startsWith', true, 'very')
    .test('endsWith', true, 'text')
    .test('contains', true, 'funny')
    .test('toString', 'very funny text')
})
