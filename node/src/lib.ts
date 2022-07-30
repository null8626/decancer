import assert from 'node:assert'
import { existsSync, readFileSync } from 'node:fs'
import { join } from 'node:path'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { createRequire } from 'node:module'
import type Decancer from './typings'

const __dirname: string = dirname(fileURLToPath(import.meta.url))

type Option<T> = T | undefined | null
type Arch =
  | string
  | {
      name: string
      musl: boolean
    }

function isMusl(): boolean {
  // For Node 10
  if (
    process.report == undefined ||
    typeof process.report.getReport !== 'function'
  ) {
    try {
      return readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
    } catch {
      return true
    }
  } else {
    // @ts-ignore
    const { glibcVersionRuntime } = process.report.getReport().header

    return !glibcVersionRuntime
  }
}

// @ts-ignore: this will NOT be null :)
let decancer: Decancer = null

function loadBinding(name: string) {
  const path: string = join(__dirname, '..', `decancer.${name}.node`)
  const require: any = createRequire(import.meta.url)

  if (existsSync(path)) {
    decancer = require(path)
  } else {
    decancer = require(`@vierofernando/decancer-${name}`)
  }
  
  // @ts-ignore
  decancer = Object.assign(decancer.decancer, { contains: decancer.contains })
}

const platforms: Record<string, Record<string, Arch>> = {
  android: { arm64: 'android-arm64' },
  win32: {
    x64: 'win32-x64-msvc',
    ia32: 'win32-ia32-msvc',
    arm64: 'win32-arm64-msvc'
  },
  darwin: { x64: 'darwin-x64', arm64: 'darwin-arm64' },
  linux: {
    x64: { name: 'linux-x64', musl: true },
    arm64: { name: 'linux-arm64', musl: true },
    arm: 'linux-arm-gnueabihf'
  }
}

try {
  const data: Option<Arch> = platforms[process.platform][process.arch]
  assert(data != null)

  if (typeof data === 'string') {
    loadBinding(data)
  } else {
    if (data.musl && isMusl()) {
      loadBinding(`${data.name}-musl`)
    } else {
      loadBinding(`${data.name}-gnu`)
    }
  }
} catch (err) {
  console.error(
    `Error: cannot load module. OS: ${process.platform} Arch: ${process.arch} may not be supported.`
  )
  throw err
}

export default decancer