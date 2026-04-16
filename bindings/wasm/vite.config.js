'use strict'

import { BINDINGS_DIR } from '../../scripts/constants.mjs'
import { renameSync } from 'node:fs'
import { join } from 'node:path'

import topLevelAwait from 'vite-plugin-top-level-await'
import license from 'rollup-plugin-license'
import { defineConfig } from 'vite'

const CURRENT_DIR = join(BINDINGS_DIR, 'wasm')

export default defineConfig({
  plugins: [
    topLevelAwait(),
    defineConfig({
      name: 'move-decancer-wasm',
      apply: 'build',
      closeBundle: () =>
        renameSync(
          join(CURRENT_DIR, 'pkg', 'decancer_bg.wasm'),
          join(CURRENT_DIR, 'bin', 'decancer.wasm')
        )
    }),
    license({
      banner: {
        commentStyle: 'ignored',
        content:
          'SPDX-License-Identifier: MIT\nSPDX-FileCopyrightText: 2021-2026 null8626'
      }
    })
  ],
  build: {
    outDir: 'bin',
    lib: {
      entry: join(CURRENT_DIR, 'src', 'glue.js'),
      name: 'decancer',
      fileName: () => 'decancer.min.js',
      formats: ['es']
    },
    minify: 'terser',
    target: 'esnext'
  }
})
