const { strictEqual } = require('node:assert')
const { describe, it } = require('node:test')

describe('decancer', () => {
  const decancer = require('./src/lib.js')
  const test = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')

  it('equals', () => strictEqual(test.equals('very funny text'), true))
  it('startsWith', () => strictEqual(test.startsWith('very'), true))
  it('endsWith', () => strictEqual(test.endsWith('text'), true))
  it('contains', () => strictEqual(test.contains('funny'), true))
  it('toString', () => strictEqual(test.toString(), 'very funny text'))
})
