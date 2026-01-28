/* eslint-disable */

import {
  BIDI_CLASSES,
  BLACKLISTED_CODEPOINTS,
  CORE_DIR,
  ROOT_DIR
} from './constants.mjs'
import { containsInclusive, request, strongAssert, SortedSet } from './util.mjs'
import { writeFile } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { promisify } from 'node:util'
import { Buffer } from 'node:buffer'
import { serialize } from 'node:v8'
import { join } from 'node:path'

const execute = promisify(exec)

function* unicodeIter(unicode) {
  for (let i = 0; i < unicode.length; i++) {
    if (unicode[i][1].endsWith('Last>')) {
      const end = parseInt(unicode[i][0], 16)
      const rest = unicode[i].slice(1)

      for (let start = parseInt(unicode[i - 1][0], 16); start <= end; start++) {
        yield [start.toString(16), ...rest]
      }
    } else {
      yield unicode[i]
    }
  }
}

function* bidiDerivedIter(bidiDerived) {
  for (let i = 0; i < bidiDerived.length; i++) {
    const second = bidiDerived[i][1]

    if (bidiDerived[i][0].includes('..')) {
      const [startString, endString] = bidiDerived[i][0].split('..')
      const end = parseInt(endString.trimEnd(), 16)

      for (let start = parseInt(startString, 16); start <= end; start++) {
        yield [start, second]
      }
    } else {
      yield [parseInt(bidiDerived[i][0].trimEnd(), 16), second]
    }
  }
}

function validCodepoint(codepoint) {
  return codepoint < 0xe01f0 && (codepoint < 0xd800 || codepoint > 0xf8ff)
}

function install(packageName) {
  return execute(`npm i ${packageName}`, {
    stdio: 'inherit'
  })
}

console.log('- installing dependencies...')

await Promise.all([install('cheerio'), install('twemoji-parser')])

const bidiExpanded = new SortedSet(x => x[0])

console.log(
  '- importing dependencies and fetching unicode data from Unicode...'
)

const [unicodeResponse, cheerio, twemojiParser] = await Promise.all([
  request('https://unicode.org/Public/UNIDATA/UnicodeData.txt'),
  import('cheerio'),
  import('twemoji-parser')
])

const unicode = unicodeResponse
  .trimEnd()
  .split('\n')
  .map(x => x.split(';'))

const cache = {
  alreadyHandledCount: 0,
  blocks: [],
  diacritics: [],
  emojis: [],
  expected: []
}

const onDecomps = {}

console.log(
  `- iterating through ${unicode.length.toLocaleString('en-US')} grouped unicode characters...`
)

for (const data of unicodeIter(unicode)) {
  const codepoint = parseInt(data[0], 16)
  const bidiIndex = BIDI_CLASSES.indexOf(data[4])
  let notHandled = bidiIndex !== -1

  if (validCodepoint(codepoint) && notHandled) {
    notHandled = BIDI_CLASSES[bidiIndex] !== 'WS'

    if (
      !BLACKLISTED_CODEPOINTS.some(([start, end]) =>
        containsInclusive(codepoint, start, end)
      ) &&
      notHandled
    ) {
      if (/LETTER \w* WITH /.test(data[1])) {
        cache.diacritics.push(codepoint)
      } else if (
        twemojiParser.parse(String.fromCodePoint(codepoint)).length > 0
      ) {
        cache.emojis.push(codepoint)
      }

      cache.expected.push(codepoint)
    }

    const decomp = data[5]

    if (decomp.length !== 0 && !decomp.includes(' ')) {
      onDecomps[codepoint] = parseInt(decomp, 16)
    }

    bidiExpanded.push([codepoint, bidiIndex])
  }

  if (!notHandled) {
    cache.alreadyHandledCount += 1
  }
}

console.log('- fetching unicode blocks...')

const blocksResponse = await fetch(
  'https://en.wikipedia.org/wiki/Unicode_block'
)
const $ = cheerio.load(await blocksResponse.text())

// we do a little scraping
$('tr').each((i, element) => {
  if (element.children?.length === 12) {
    const tags = element.children.filter(y => y.type === 'tag')

    try {
      const [start, end] = tags[1].children
        .find(y => y.name === 'span')
        .children[0].data.split('..')
        .map(z => parseInt(z.slice(2), 16))

      cache.blocks.push({
        start,
        end,
        name: tags[2].children
          .find(y => y.name === 'a')
          .children[0].data.toLowerCase()
      })
    } catch {}
  }
})

void (await Promise.all([
  (async () => {
    console.log('- writing to cache...')

    await writeFile(join(ROOT_DIR, '.cache.bin'), serialize(cache))

    console.log('- wrote to cache.')
  })(),

  (async () => {
    const bidiDerived = (
      await request(
        'https://www.unicode.org/Public/UNIDATA/extracted/DerivedBidiClass.txt'
      )
    )
      .trimEnd()
      .split('\n')
      .map(x => x.split(';'))
      .filter(x => x[0][0] !== '#' && x.length === 2)

    const beforeAddition = bidiExpanded.array.length
    console.log(
      `- iterating through ${bidiDerived.length.toLocaleString('en-US')} grouped bidi derived classes...`
    )

    for (const bidi of bidiDerivedIter(bidiDerived)) {
      const bidiClass = bidi[1].match(/^\s*([A-Z]+)/)[1]
      const bidiIndex = BIDI_CLASSES.indexOf(bidiClass)

      if (validCodepoint(bidi[0]) && bidiIndex !== -1) {
        bidiExpanded.push([bidi[0], bidiIndex])
      }
    }

    console.log(
      `- added an additional ${(
        bidiExpanded.array.length - beforeAddition
      ).toLocaleString(
        'en-US'
      )} bidi classes from unicode's derived bidi classes list.`
    )

    const beforeMerge = bidiExpanded.array.length
    console.log(
      `- merging ${beforeMerge.toLocaleString('en-US')} bidi classes into one...`
    )

    const bidiDictionary = []

    for (const [codepoint, bidi] of bidiExpanded.array) {
      const last = bidiDictionary.length - 1

      if (
        bidiDictionary[last]?.bidi === bidi &&
        bidiDictionary[last].end === codepoint - 1
      ) {
        bidiDictionary[last].end = codepoint
      } else {
        bidiDictionary.push({
          start: codepoint,
          end: codepoint,
          bidi
        })
      }
    }

    console.log(
      `- merged down from ${beforeMerge.toLocaleString('en-US')} to ${bidiDictionary.length.toLocaleString('en-US')} (${(
        (bidiDictionary.length / beforeMerge) *
        100
      ).toFixed(2)}%)`
    )

    const bidiBracketsBuffer = Buffer.concat(
      (await request('https://unicode.org/Public/UNIDATA/BidiBrackets.txt'))
        .trimEnd()
        .split('\n')
        .map(x => x.split(';'))
        .filter(x => x.length === 3 && x[2].trimStart().startsWith('o'))
        .map(x => {
          const codepoint1 = parseInt(x[0], 16)
          const diff = parseInt(x[1], 16) - codepoint1
          const diffAbs = Math.abs(diff)

          strongAssert(
            codepoint1 <= 0xffff,
            'found codepoint above the limit of 0xffff:',
            codepoint1
          )
          strongAssert(
            diffAbs <= 7,
            'found absolute form of diff to be above the limit of 7:',
            diffAbs
          )

          let first =
            BigInt(onDecomps[codepoint1] ?? 0) |
            (BigInt(codepoint1 & 0xff) << 20n) |
            (BigInt(diffAbs) << 28n)

          if (diff < 0) {
            first |= 0x80000000n
          }

          const buf = Buffer.alloc(5)

          buf.writeUint32LE(Number(first))
          buf.writeUint8(codepoint1 >> 8, 4)

          return buf
        })
    )

    strongAssert(
      bidiBracketsBuffer.length + 2 <= 0xffff,
      'bidi brackets buffer length over the limit:',
      bidiBracketsBuffer.length
    )

    console.log(
      `- stored ${(bidiBracketsBuffer.length / 5).toLocaleString(
        'en-US'
      )} bidi brackets.`
    )

    const bidiDictionaryBuffer = Buffer.concat(
      bidiDictionary.map(({ start, end, bidi }) => {
        const diff = end - start

        strongAssert(
          diff <= 0xffff,
          `found bidi dictionary range size to be above 0xffff: ${start} to ${end} (${BIDI_CLASSES[bidi]})`
        )

        const buf = Buffer.alloc(6)

        buf.writeUint32LE(Number(BigInt(start) | (BigInt(bidi) << 20n)))
        buf.writeUint16LE(diff, 4)

        return buf
      })
    )

    const bidiBufferHeader = Buffer.alloc(4)
    bidiBufferHeader.writeUint16LE(bidiBracketsBuffer.length + 4)
    bidiBufferHeader.writeUint16LE(bidiDictionary.length - 1, 2)

    console.log('- writing to bidi.bin...')

    await writeFile(
      join(CORE_DIR, 'bin', 'bidi.bin'),
      Buffer.concat([
        bidiBufferHeader,
        bidiBracketsBuffer,
        bidiDictionaryBuffer
      ])
    )

    console.log('- wrote to bidi.bin.')
  })()
]))
