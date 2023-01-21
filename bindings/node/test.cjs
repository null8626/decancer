const { strictEqual } = require('node:assert')
const decancer = require('./src/lib.js')

const test = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')

strictEqual(test.equals('very funny text'), true)
strictEqual(test.startsWith('very'), true)
strictEqual(test.endsWith('text'), true)
strictEqual(test.contains('funny'), true)
strictEqual(test.toString(), 'very funny text')
