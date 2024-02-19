import { containsInclusive, request, strongAssert, SortedSet } from './util.mjs'
import { writeFile } from 'node:fs/promises'
import { exec } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'
import { existsSync } from 'node:fs'
import { serialize } from 'node:v8'

const BIDI_CLASSES = [
  'B',
  'S',
  'WS',
  'ON',
  'ET',
  'ES',
  'CS',
  'EN',
  'L',
  'BN',
  'R',
  'AN',
  'AL',
  'LRE',
  'RLE',
  'PDF',
  'LRO',
  'RLO',
  'LRI',
  'RLI',
  'FSI',
  'PDI'
]

const BLACKLISTED_RANGES = [
  [0, 0x7f],
  [0x200e, 0x200f],
  [0x202a, 0x202e],
  [0x2066, 0x2069],
  [0x11700, 0x1173f],
  [0x16f00, 0x16f9f],
  [0x118a0, 0x118ff],
  [0x10500, 0x1052f],
  [0x11480, 0x114df]
]

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')

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
      const end = parseInt(endString.trimRight(), 16)

      for (let start = parseInt(startString, 16); start <= end; start++) {
        yield [start, second]
      }
    } else {
      yield [parseInt(bidiDerived[i][0].trimRight(), 16), second]
    }
  }
}

function validCodepoint(codepoint) {
  return codepoint < 0xe01f0 && (codepoint < 0xd800 || codepoint > 0xf8ff)
}

const bidiExpanded = new SortedSet(x => x[0])

const unicode = (
  await request('https://unicode.org/Public/UNIDATA/UnicodeData.txt')
)
  .trimRight()
  .split('\n')
  .map(x => x.split(';'))

let expected = new SortedSet()
let cache = {
  alreadyHandledCount: 0,
  blocks: [],
  diacritics: []
}

const onDecomps = {}

console.log(
  `- iterating through ${unicode.length.toLocaleString()} grouped unicode characters...`
)

for (const data of unicodeIter(unicode)) {
  const codepoint = parseInt(data[0], 16)
  const bidiIndex = BIDI_CLASSES.indexOf(data[4])
  let notHandled = bidiIndex !== -1

  if (validCodepoint(codepoint) && notHandled) {
    notHandled = BIDI_CLASSES[bidiIndex] !== 'WS'

    if (
      !BLACKLISTED_RANGES.some(([start, end]) =>
        containsInclusive(codepoint, start, end)
      ) &&
      notHandled
    ) {
      if (/LETTER \w* WITH /.test(data[1])) {
        cache.diacritics.push(codepoint)
      }

      expected.push(codepoint)
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

cache.expected = expected.array

console.log('- installing cheerio...')

await execute('npm i cheerio', {
  stdio: 'inherit'
})

const cheerio = await import('cheerio')

console.log('- fetching unicode blocks...')

const blocksResponse = await fetch(
  'https://en.wikipedia.org/wiki/Unicode_block'
)
const $ = cheerio.default.load(await blocksResponse.text())

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
      .trimRight()
      .split('\n')
      .map(x => x.split(';'))
      .filter(x => x[0][0] !== '#' && x.length === 2)

    const beforeAddition = bidiExpanded.array.length
    console.log(
      `- iterating through ${bidiDerived.length.toLocaleString()} grouped bidi derived classes...`
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
      ).toLocaleString()} bidi classes from unicode's derived bidi classes list.`
    )

    const beforeMerge = bidiExpanded.array.length
    console.log(
      `- merging ${beforeMerge.toLocaleString()} bidi classes into one...`
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
      `- merged down from ${beforeMerge.toLocaleString()} to ${bidiDictionary.length.toLocaleString()} (${(
        (bidiDictionary.length / beforeMerge) *
        100
      ).toFixed(2)}%)`
    )

    const bidiBracketsBuffer = Buffer.concat(
      (await request('https://unicode.org/Public/UNIDATA/BidiBrackets.txt'))
        .trimRight()
        .split('\n')
        .map(x => x.split(';'))
        .filter(x => x.length === 3 && x[2].trimLeft().startsWith('o'))
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
      `- stored ${(
        bidiBracketsBuffer.length / 5
      ).toLocaleString()} bidi brackets.`
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
      join(ROOT_DIR, 'core', 'bin', 'bidi.bin'),
      Buffer.concat([
        bidiBufferHeader,
        bidiBracketsBuffer,
        bidiDictionaryBuffer
      ])
    )

    console.log('- wrote to bidi.bin.')
  })()
]))
