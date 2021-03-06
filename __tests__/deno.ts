// @ts-ignore
import { assertEquals } from 'https://deno.land/std@0.150.0/testing/asserts.ts'

// @ts-ignore
import init from '../mod.ts'

// @ts-ignore
Deno.test("it works with Deno", async () => {
  const decancer = await init()
  const decancered = decancer('vοΌ₯β‘π π½πΕβο½ Ε£δΉππ£')

  assertEquals(decancered, 'very funny text')
  assertEquals(true, decancer.contains(decancered, 'funny'))
  assertEquals(true, decancer.contains('this is a piece of $h1t', 'shit'))
});