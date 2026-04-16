// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

'use strict'

import * as glue from '../pkg/decancer_bg.js'

const OPTIONS = {
  keys: [
    'retainCapitalization',
    'disableBidi',
    'disableLeetspeak',
    'disableAlphabeticalLeetspeak',
    'retainDiacritics',
    'retainGreek',
    'retainCyrillic',
    'retainHebrew',
    'retainArabic',
    'retainDevanagari',
    'retainBengali',
    'retainArmenian',
    'retainGujarati',
    'retainTamil',
    'retainThai',
    'retainLao',
    'retainBurmese',
    'retainKhmer',
    'retainMongolian',
    'retainChinese',
    'retainJapanese',
    'retainKorean',
    'retainBraille',
    'retainEmojis',
    'retainTurkish',
    'asciiOnly',
    'alphanumericOnly'
  ],
  overrides: { all: 0x7ffffff, pureHomoglyph: 0xfffff0 }
}

function options(opt = {}) {
  const override = Object.entries(OPTIONS.overrides).find(([key]) => opt[key])

  if (override) {
    return override[1]
  }

  let output = 0

  for (let i = 0; i < OPTIONS.keys.length; i++) {
    if (opt[OPTIONS.keys[i]]) {
      output |= 1 << i
    }
  }

  return output
}

let exports = null

export default async function init({ local } = {}) {
  if (exports !== null) {
    return exports
  }

  const wasm = await fetch(
    local
      ? './decancer.wasm'
      : 'https://cdn.jsdelivr.net/gh/null8626/decancer@v4.0.0/bindings/wasm/bin/decancer.wasm'
  )
  const { instance } = await WebAssembly.instantiate(await wasm.arrayBuffer(), {
    './decancer_bg.js': glue,
    ...glue
  })

  glue.__wbg_set_wasm(instance.exports)
  instance.exports.__wbindgen_start()

  return (exports = Object.assign(
    function cure(input, opt) {
      if (typeof opt !== 'number') {
        opt = options(opt)
      }

      return glue.cure(input, opt)
    },
    { options }
  ))
}
