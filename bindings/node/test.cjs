const { strictEqual } = require('node:assert')
const { describe, it } = require('node:test')

const assert = (expected, func, ...arguments) => it(func.name, () => strictEqual(func(...arguments), expected))

describe('decancer', () => {
  const decancer = require('./src/lib.js')
  const cured = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')

  assert(true, cured.equals, 'very funny text')
  assert(true, cured.startsWith, 'very')
  assert(true, cured.endsWith, 'text')
  assert(true, cured.contains, 'funny')
  assert('very funny text', cured.toString)
})
