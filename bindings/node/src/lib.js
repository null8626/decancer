'use strict'

const { existsSync, readFileSync } = require('node:fs')
const assert = require('node:assert')
const { join } = require('node:path')

const PLATFORMS = {
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
  },
  android: {
    arm64: 'android-arm64',
    arm: 'android-arm-eabi'
  }
}

function isMusl() {
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
    const { glibcVersionRuntime } = process.report.getReport().header

    return !glibcVersionRuntime
  }
}

function getBinding(name) {
  const path = join(__dirname, '..', `decancer.${name}.node`)

  return require(existsSync(path) ? path : `@vierofernando/decancer-${name}`)
}

try {
  const data = PLATFORMS[process.platform][process.arch]

  assert(
    data != null,
    `This platform (${process.platform} on a ${process.arch}) is not supported.`
  )

  module.exports = getBinding(
    typeof data === 'string'
      ? data
      : `${data.name}-${data.musl && isMusl() ? 'musl' : 'gnu'}`
  ).decancer
} catch (err) {
  console.error(
    `Error: cannot load module. OS: ${process.platform} Arch: ${process.arch} may not be supported.`
  )

  throw err
}
