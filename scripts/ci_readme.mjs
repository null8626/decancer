/* eslint-disable */

'use strict'

import { readFileSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')

const modifiedWarning =
  '<!-- WARNING: this markdown file is computer generated.\n     please modify the README.md file in the root directory instead. -->\n\n'
const preprocessedLines = []
const inputDefinitions = process.argv.slice(3)
let currentDefinition = null

for (const line of readFileSync(join(ROOT_DIR, 'README.md'))
  .toString()
  .trim()
  .split(/\r?\n/g)) {
  if (line.startsWith('<!---[') && line.endsWith(']--->')) {
    for (let instruction of line
      .slice(6, -5)
      .trim()
      .split(/\s*\,\s*/)) {
      instruction = instruction.trim()

      if (instruction === 'end') {
        currentDefinition = null
      } else if (instruction.startsWith('begin')) {
        currentDefinition = instruction.replace(/^begin\s*/, '')
      }
    }
  } else if (
    currentDefinition === null ||
    inputDefinitions.includes(currentDefinition)
  ) {
    preprocessedLines.push(line)
  }
}

writeFileSync(
  process.argv[2],
  modifiedWarning + preprocessedLines.join('\n').replaceAll(/\n{3,}/g, '\n')
)
