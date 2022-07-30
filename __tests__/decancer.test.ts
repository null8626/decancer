import { execSync } from 'node:child_process'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))

it('works with Rust', () => void execSync('cargo test', {
  stdio: 'inherit',
  cwd: resolve(__dirname, '..', 'core')
}))

it('compiles in Node.js', () => void execSync('npm run build', {
  stdio: 'inherit',
  cwd: resolve(__dirname, '..', 'node')
}))

it('works with Node.js', async () => {
  const decancer = await import(resolve(__dirname, '..', 'node', 'src', 'lib'))
  const decancered = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣')

  expect(decancered).toBe('very funny text')
  expect(decancer.contains(decancered, 'funny')).toBe(true)
  expect(decancer.contains('this is a piece of $h1t', 'shit')).toBe(true)
})

it('works with Deno', () => void execSync(`deno run ${resolve(__dirname, 'deno.ts')} --allow-net`, {
  stdio: 'inherit',
  cwd: __dirname
}))