import { readFileSync, writeFileSync } from 'node:fs'
import { inspect } from 'node:util'
import assert from 'node:assert'

if (typeof process.argv[2] !== 'string') {
  console.error('error: missing json file path.')
  process.exit(1)
}

const { confusables, similar } = JSON.parse(readFileSync(process.argv[2]))

assert(Array.isArray(confusables), 'confusables must be an array')
assert(Array.isArray(similar) && similar.every(x => Array.isArray(x) && x.every(y => y.length === 1 && y.codePointAt() <= 0xff)), 'similar must be an array of an array of ASCII strings')

const isCaseSensitive = x => String.fromCodePoint(x).toLowerCase() !== String.fromCodePoint(x)

console.log(`- checking, expanding, and sorting ${confusables.length} confusables...`)

let expanded = []

for (const conf of confusables) {
  assert(Number.isSafeInteger(conf.codepoint) && conf.codepoint >= 0 && conf.codepoint < 0x110000, 'codepoint must be a valid number')
  assert(typeof conf.translation === 'string' && conf.translation.length >= 1, 'translation must be a string')

  if (typeof conf.rangeUntil === 'number') {
    assert(Number.isSafeInteger(conf.rangeUntil) && conf.rangeUntil >= 0 && conf.rangeUntil < 0x110000 && (conf.rangeUntil - conf.codepoint) <= 0x7f, 'rangeUntil must be a valid number')
    assert(conf.rangeUntil > conf.codepoint, `rangeUntil must be greater than codepoint. (rangeUntil: ${conf.rangeUntil}, codepoint: ${conf.codepoint})`)

    const ogTranslationCode = conf.syncedTranslation ? conf.translation.charCodeAt() : conf.translation
  
    for (let c = conf.codepoint; c <= conf.rangeUntil; c++) {
      expanded.push({
        codepoint: c,
        translation: typeof ogTranslationCode === 'number' ? String.fromCharCode(ogTranslationCode + (c - conf.codepoint)) : ogTranslationCode,
        caseSensitive: isCaseSensitive(typeof ogTranslationCode === 'number' ? (ogTranslationCode + (c - conf.codepoint)) : ogTranslationCode.charCodeAt())
      })
    }
  } else {
    expanded.push({
      codepoint: conf.codepoint,
      translation: conf.translation,
      caseSensitive: isCaseSensitive(conf.codepoint)
    })
  }
}

console.log(`- expanded to a grand total of ${expanded.length} confusables.\n- searching for collisions...`)

function retrieveCollisions(array, set) {
  for (const part of set)
    array.splice(array.indexOf(part), 1)

  return array
}

{
  const set = [...new Set(expanded.map(x => x.codepoint))]
  assert(expanded.length === set.length, `discovered ${expanded.length - set.length} collisions. at codepoints: ${inspect(retrieveCollisions(expanded.map(x => x.codepoint), set))}`)
}

const notSyncedSequences = [], syncedSequences = [], rest = []

for (let i = 0, curr = null; i < expanded.length; i++) {
  const n = expanded[i]
  
  if (n.translation.length === 1) {
    const next = expanded[i + 1]
    const ordered = (n.codepoint + 1) === next?.codepoint && n.caseSensitive === next.caseSensitive

    if (curr !== null) {
      if (ordered && (curr.syncedTranslation ? ((n.translation.charCodeAt() + 1) === next.translation.charCodeAt()) : next.translation === n.translation)) {
        curr.rangeUntil++
        continue
      }
    
      if (curr.syncedTranslation) {
        syncedSequences.push(curr)
      } else {
        notSyncedSequences.push(curr)
      }
    
      curr = null
      continue
    }
  
    const synced = (n.translation.charCodeAt() + 1) === next?.translation?.charCodeAt()
  
    if (ordered && (synced || next?.translation === n.translation)) {
      curr = n
      curr.syncedTranslation = synced
      curr.rangeUntil = n.codepoint + 1
      continue
    }
  }

  n.syncedTranslation = false
  n.rangeUntil = null
  rest.push(n)
}

const sequenceReduceFunc = (a, b) => a + (b.rangeUntil - b.codepoint) + 1
console.log(`- discovered ${syncedSequences.length} (${Math.round((syncedSequences.reduce(sequenceReduceFunc, 0) / expanded.length) * 100)}%) synced sequences and ${notSyncedSequences.length} (${Math.round((notSyncedSequences.reduce(sequenceReduceFunc, 0) / expanded.length) * 100)}%) unsynced sequences.`)

const grandTotal = [...syncedSequences, ...notSyncedSequences, ...rest].sort((a, b) => a.codepoint - b.codepoint)

writeFileSync(process.argv[2].replace(/\.json$/i, 'Optimized.json'), JSON.stringify({ confusables: grandTotal, similar }, null, 2))

console.log(`- condensed down from ${expanded.length} to ${grandTotal.length} (${Math.round((grandTotal.length / expanded.length) * 100)}%). (wrote refactored JSON output to ${process.argv[2].replace(/\.json$/i, 'Refactored.json')})`)

const similarBytes = Buffer.concat(similar.map(x => Buffer.from([x.length, ...x.map(y => y.charCodeAt())])))
const strings = []
const confusablesBuffers = []

for (const { codepoint, translation, caseSensitive, rangeUntil, syncedTranslation } of grandTotal) {
  const buf = Buffer.alloc(5)
  let integer = 0x100000000n | BigInt(codepoint)
  let secondByte = 0
  
  if (caseSensitive)
    integer |= 0x40000000n
  
  if (syncedTranslation)
    secondByte = 0x80
  
  if (rangeUntil !== null) {
    integer |= 0x10000000n
    secondByte |= (rangeUntil - codepoint)
  }
  
  if (translation.length > 1) {
    if (!strings.includes(translation))
      strings.push(translation)
    
    integer |= 0x20000000n
    integer |= (BigInt(strings.indexOf(translation)) << 21n)
  } else {
    integer |= (BigInt(translation.charCodeAt()) << 21n)
  }
  
  buf.writeUint32LE(Number(integer & 0xffffffffn))
  buf.writeUint8(secondByte, 4)
  
  confusablesBuffers.push(buf)
}

const headers = Buffer.alloc(4)
headers.writeUint16LE(4 + (confusablesBuffers.length * 5))
headers.writeUint16LE(headers.readUint16LE() + similarBytes.length, 2)

writeFileSync('output.bin', Buffer.concat([
  headers,
  Buffer.concat(confusablesBuffers),
  similarBytes,
  Buffer.from(strings.map(x => [x.length, ...x.split('').map(y => y.charCodeAt())]).flat())
]))

console.log('- wrote to output.bin.')