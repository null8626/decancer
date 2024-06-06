/* eslint-disable */

'use strict'

import { readFile, writeFile, readdir, stat, rm } from 'node:fs/promises'
import { dirname, join, sep } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const EXCLUDED = [
  'jitpack.yml',
  'LICENSE',
  ['bindings', 'java', 'src', 'main'],
  ['bindings', 'java', 'src', 'test'],
  ['bindings', 'java', 'bin'],
  ['bindings', 'java', 'gradle.build'],
  ['bindings', 'java', 'gradlew'],
  ['bindings', 'java', 'gradlew.bat'],
  ['bindings', 'java', 'gradle'],
  ['.git']
]

function lookInside(fullPath) {
  for (const ne of EXCLUDED) {
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
  for (const ne of EXCLUDED) {
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
