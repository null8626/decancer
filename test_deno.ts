import { assertStrictEquals } from 'https://deno.land/std@0.170.0/testing/asserts.ts'
import init from './mod.ts'

const decancer = await init()

Deno.test("it works", () => {
  const test = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')

  assertStrictEquals(test.equals('very funny text'), true)
  assertStrictEquals(test.startsWith('very'), true)
  assertStrictEquals(test.endsWith('text'), true)
  assertStrictEquals(test.contains('funny'), true)
  assertStrictEquals(test.toString(), 'very funny text')
})