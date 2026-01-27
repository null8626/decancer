/* eslint-disable */

'use strict'

import { BINDINGS_DIR, GITHUB_PAGES_IGNORE, ROOT_DIR } from './constants.mjs'
import { readFile, writeFile, readdir, stat, rm } from 'node:fs/promises'
import { join, sep } from 'node:path'

const MINIFIED_JS = join(BINDINGS_DIR, 'wasm', 'bin', 'decancer.min.js')

const editedMinifiedJsContents = (await readFile(MINIFIED_JS))
  .toString()
  .replace(
    /https\:\/\/cdn\.jsdelivr\.net\/gh\/null8626\/decancer@v[\d\.]+\/bindings\/wasm\/bin\/decancer\.wasm/,
    'https://null8626.github.io/decancer/bindings/wasm/bin/decancer.wasm'
  )
await writeFile(MINIFIED_JS, editedMinifiedJsContents)

function lookInside(fullPath) {
  for (const ne of GITHUB_PAGES_IGNORE) {
    if (typeof ne === 'string') {
      if (fullPath === join(ROOT_DIR, ne)) {
        return false
      }
    } else {
      const pathSplitLength = fullPath
        .replace(ROOT_DIR + sep, '')
        .split(sep).length

      if (pathSplitLength < ne.length) {
        const joined = [ROOT_DIR]
        let matched = -1

        for (let i = 0; i < ne.length; i++) {
          joined.push(ne[i])

          if (fullPath.startsWith(join(...joined))) {
            matched = i
          }
        }

        return matched === pathSplitLength - 1
      }
    }
  }

  return false
}

function isExcluded(fullPath) {
  for (const ne of GITHUB_PAGES_IGNORE) {
    if (typeof ne === 'string') {
      if (fullPath === join(ROOT_DIR, ne)) {
        return false
      }
    } else if (fullPath === join(ROOT_DIR, ...ne)) {
      return false
    }
  }

  return true
}

async function resolveDirectory(directoryName) {
  const files = await readdir(directoryName)

  void (await Promise.all(
    files
      .map(path => join(directoryName, path))
      .map(async path => {
        const fstat = await stat(path)
        const isDirectory = fstat.isDirectory()

        if (isDirectory && lookInside(path)) {
          return await resolveDirectory(path)
        } else if (isExcluded(path)) {
          await rm(path, { recursive: isDirectory, force: true })
        }
      })
  ))
}

void (await resolveDirectory(ROOT_DIR))
