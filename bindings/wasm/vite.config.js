'use strict'

import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

import { viteStaticCopy } from 'vite-plugin-static-copy'
import topLevelAwait from 'vite-plugin-top-level-await'
import license from 'rollup-plugin-license'
import { defineConfig } from 'vite'

const CURRENT_DIR = dirname(fileURLToPath(import.meta.url))

export default defineConfig({
  plugins: [
    topLevelAwait(),
    viteStaticCopy({
      targets: [
        {
          src: './pkg/decancer_bg.wasm',
          dest: '.',
          rename: 'decancer.wasm'
        }
      ]
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
