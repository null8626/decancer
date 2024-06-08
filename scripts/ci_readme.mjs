import { readFileSync, writeFileSync } from 'node:fs'

const modifiedWarning = '<!-- WARNING: this markdown file is computer generated.\n     please modify the README.md file in the root directory instead. -->\n\n'
const preprocessedLines = []
const readmeDestination = process.argv[3]
const inputDefinitions = process.argv.slice(4)
let currentDefinition = null

for (const line of readFileSync(process.argv[2]).toString().trim().split(/\r?\n/g)) {
  if (line.startsWith('<!---[') && line.endsWith(']--->')) {
    for (let instruction of line.slice(6, -5).trim().split(/\s*\,\s*/)) {
      instruction = instruction.trim()
      
      if (instruction === 'end') {
        currentDefinition = null
      } else if (instruction.startsWith('begin')) {
        currentDefinition = instruction.replace(/^begin\s*/, '')
      }
    }
  } else if (currentDefinition === null || inputDefinitions.includes(currentDefinition)) {
    preprocessedLines.push(line)
  }
}

writeFileSync(process.argv[3], modifiedWarning + preprocessedLines.join('\n').replaceAll(/\n{3,}/g, '\n'))