/* eslint-disable */

'use strict'

import { readdirSync, readFileSync, writeFileSync } from 'node:fs'
import { execSync } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = dirname(fileURLToPath(import.meta.url))
const HTML_DIR = join(ROOT_DIR, 'html')
let XMLParser

try {
  XMLParser = (await import('fast-xml-parser')).XMLParser
} catch {
  try {
    execSync('npm init -y && npm i fast-xml-parser --save')
  } catch {
    process.exit(1)
  }

  XMLParser = (await import('fast-xml-parser')).XMLParser
}

function renderAPIHTML(parts) {
  let rendered =
    '<div id="apis"><div id="api-not-found">Not such query exists :(</div>'

  for (const [name, members] of Object.entries(parts)) {
    rendered += `<div id="apitype">${name}</div><div id="apilist">`

    for (const member of members) {
      rendered += `<a id="api" href="${member.href}">${member.name}</a>`
    }

    rendered += '</div>'
  }

  return `${rendered}</div>`
}

try {
  execSync('doxygen', {
    cwd: ROOT_DIR,
    stdio: 'inherit'
  })
} catch {
  process.exit(1)
}

const parser = new XMLParser({
  ignoreAttributes: false
})

const index = parser.parse(
  readFileSync(join(ROOT_DIR, 'xml', 'index.xml')).toString()
)

const typeDefinitions = []
const macros = []
const functions = []

for (const compound of index.doxygenindex.compound) {
  if (compound['@_kind'] === 'struct') {
    typeDefinitions.push({
      name: compound.name,
      href: `${compound['@_refid']}.html`
    })
  } else if (compound.name === 'decancer.h') {
    for (const member of compound.member) {
      const data = {
        name: member.name,
        href: member['@_refid'].replace(
          /_1([a-f0-9]+)$/,
          (_, x) => `.html#${x}`
        )
      }

      switch (member['@_kind']) {
        case 'define':
          macros.push(data)
          break

        case 'typedef':
          typeDefinitions.push(data)
          break

        case 'function':
          functions.push(data)
      }
    }
  }
}

const renderedAPIHTML = renderAPIHTML({
  Functions: functions,
  'Type definitions': typeDefinitions,
  Macros: macros
})

for (const htmlFile of readdirSync(HTML_DIR).filter(file =>
  file.endsWith('.html')
)) {
  const htmlFilePath = join(HTML_DIR, htmlFile)
  const htmlFileContents = readFileSync(htmlFilePath).toString()

  try {
    writeFileSync(
      htmlFilePath,
      htmlFileContents.replace('$apis', renderedAPIHTML)
    )
  } catch {}
}
