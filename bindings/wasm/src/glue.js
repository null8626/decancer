// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

'use strict'

import * as glue from '../pkg/decancer_bg.js'

const OPTIONS = {
  keys: [
    'retainCapitalization',
    'disableBidi',
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
  overrides: { all: 0x1ffffff, pureHomoglyph: 0x3ffffc }
}

function options(opt = {}) {
  let output = 0

  for (let i = 0; i < OPTIONS.keys.length; i++) {
    if (opt[OPTIONS.keys[i]]) {
      output |= 1 << i
    }
  }

  for (const [key, override] of Object.entries(OPTIONS.overrides)) {
    if (opt[key]) {
      output = override
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
      : 'https://cdn.jsdelivr.net/gh/null8626/decancer@v3.3.3/bindings/wasm/bin/decancer.wasm'
  )
  const { instance } = await WebAssembly.instantiate(await wasm.arrayBuffer(), {
    './decancer_bg.js': glue
  })

  glue.__wbg_set_wasm(instance.exports)
  instance.exports.__wbindgen_start()

  exports = Object.assign(glue.cure, { options })

  return exports
}
