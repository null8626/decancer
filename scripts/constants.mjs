/* eslint-disable */

import { join, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'
import process from 'node:process'

export const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')

export const CORE_DIR = join(ROOT_DIR, 'core')
export const BINDINGS_DIR = join(ROOT_DIR, 'bindings')
export const CACHE_FILE = join(ROOT_DIR, '.cache.bin')

export const OPTIONS = Object.fromEntries(
  process.argv
    .slice(2)
    .map(x => x.match(/^--([\w-]+)=(.*)/)?.slice(1))
    .filter(x => x)
)

export const JRELEASER_VERSION = '1.18.0'

export const EXPECTED_NATIVE_TARGETS = [
  'aarch64-apple-darwin',
  'aarch64-apple-ios',
  'aarch64-apple-ios-sim',
  'aarch64-linux-android',
  'aarch64-pc-windows-msvc',
  'aarch64-unknown-linux-gnu',
  'aarch64-unknown-linux-musl',
  'arm-unknown-linux-gnueabi',
  'armv5te-unknown-linux-gnueabi',
  'armv7-linux-androideabi',
  'armv7-unknown-linux-gnueabi',
  'armv7-unknown-linux-gnueabihf',
  'i586-unknown-linux-gnu',
  'i686-pc-windows-msvc',
  'i686-unknown-freebsd',
  'i686-unknown-linux-gnu',
  'powerpc64le-unknown-linux-gnu',
  'riscv64gc-unknown-linux-gnu',
  's390x-unknown-linux-gnu',
  'sparcv9-sun-solaris',
  'thumbv7neon-unknown-linux-gnueabihf',
  'x86_64-apple-darwin',
  'x86_64-apple-ios',
  'x86_64-pc-windows-msvc',
  'x86_64-unknown-freebsd',
  'x86_64-unknown-illumos',
  'x86_64-unknown-linux-gnu',
  'x86_64-unknown-linux-musl'
]

export const EXPECTED_NODE_TARGETS = [
  'x86_64-apple-darwin',
  'x86_64-pc-windows-msvc',
  'i686-pc-windows-msvc',
  'x86_64-unknown-linux-gnu',
  'x86_64-unknown-linux-musl',
  'aarch64-apple-darwin',
  'aarch64-unknown-linux-gnu',
  'armv7-unknown-linux-gnueabihf',
  'aarch64-linux-android',
  'armv7-linux-androideabi',
  'aarch64-unknown-linux-musl',
  'aarch64-pc-windows-msvc',
  'freebsd-x64'
]

export const EXPECTED_JAVA_TARGETS = [
  'aarch64-apple-darwin',
  'aarch64-pc-windows-msvc',
  'aarch64-unknown-linux-gnu',
  'aarch64-unknown-linux-musl',
  'arm-unknown-linux-gnueabi',
  'armv5te-unknown-linux-gnueabi',
  'armv7-unknown-linux-gnueabi',
  'armv7-unknown-linux-gnueabihf',
  'i686-pc-windows-msvc',
  'i686-unknown-freebsd',
  'i686-unknown-linux-gnu',
  'riscv64gc-unknown-linux-gnu',
  'x86_64-apple-darwin',
  'x86_64-pc-windows-msvc',
  'x86_64-unknown-freebsd',
  'x86_64-unknown-linux-gnu',
  'x86_64-unknown-linux-musl'
]

export const GITHUB_PAGES_IGNORE = [
  'wasm_example.html',
  ['bindings', 'wasm', 'bin'],
  ['native_docs'],
  ['scripts'],
  ['.git']
]

export const MODIFIED_RETAIN_TESTS_WARNING =
  '// WARNING: This file is computer generated.'
export const MODIFIED_README_WARNING =
  '<!-- WARNING: This file is computer generated.\n     please modify the README.md file in the root directory instead. -->\n\n'
export const SPDX_LICENSE_COMMENTS = `// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626`

export const RETAIN_TESTS_SAMPLE_SIZE = 5

// 0..=9 | 14..=31 | 127 | 0xd800..=0xf8ff | 0xe01f0..=0x10ffff
export const NONE_CODEPOINTS_COUNT = 10 + 18 + 1 + 8448 + 196112

export const CODEPOINT_MASK = 0xfffff
export const STRING_TRANSLATION_MASK = 0x10000000

export const RETAINABLE_SCRIPTS = Object.entries({
  greek: {
    shift: 5,
    check: name => name.includes('greek') && !name.includes('ancient')
  },
  cyrillic: 6,
  hebrew: 7,
  arabic: 8,
  devanagari: 9,
  bengali: 10,
  armenian: 11,
  gujarati: 12,
  tamil: 13,
  thai: 14,
  lao: 15,
  burmese: {
    shift: 16,
    check: name => name.includes('myanmar')
  },
  khmer: 17,
  mongolian: 18,
  chinese: {
    shift: 19,
    check: name => name.includes('cjk') || name.includes('kangxi')
  },
  japanese: {
    shift: 20,
    check: name => name.includes('katakana') || name.includes('hiragana')
  },
  korean: {
    shift: 21,
    check: name => name.includes('hangul')
  },
  braille: 22
})

export const TURKISH_CHARACTERS = ['ç', 'ğ', 'ı', 'ö', 'ş', 'ü'] // İ is omitted here because its lowercase form is just a normal i

export const BIDI_CLASSES = [
  'B',
  'S',
  'WS',
  'ON',
  'ET',
  'ES',
  'CS',
  'EN',
  'L',
  'BN',
  'R',
  'AN',
  'AL',
  'LRE',
  'RLE',
  'PDF',
  'LRO',
  'RLO',
  'LRI',
  'RLI',
  'FSI',
  'PDI'
]

export const BLACKLISTED_CODEPOINTS = [
  [0, 0x7f],
  [0x200e, 0x200f],
  [0x202a, 0x202e],
  [0x2066, 0x2069]
]
