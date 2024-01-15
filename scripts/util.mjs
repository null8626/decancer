export async function request(url) {
  console.log(`- requesting to ${url}...`)

  const req = await fetch(url)
  const text = await req.text()

  console.log(`- parsing data returned from ${url}...`)
  return text
}

export function containsInclusive(value, rangeStart, rangeEnd) {
  return value >= rangeStart && value <= rangeEnd
}

export function strongAssert(expr, ...rest) {
  if (!expr) {
    console.error('- fatal error:', ...rest)
    process.exit(1)
  }
}

export function isCaseSensitive(x) {
  return String.fromCodePoint(x).toLowerCase().codePointAt() !== x
}

function merge(a, b, recurse = true) {
  if (a.includes(b)) {
    return a
  } else if (b.includes(a)) {
    return b
  }

  const minimumLength = Math.min(a.length, b.length)
  let maxLimit

  for (let limit = 1; limit <= minimumLength; limit++) {
    if (a.slice(0, limit) === b.slice(-limit)) {
      maxLimit = limit
    }
  }

  if (maxLimit === undefined) {
    if (recurse) {
      return merge(b, a, false)
    }
  } else {
    return b.slice(0, -maxLimit) + a
  }
}

export function mergeArray(arr, recurse = true) {
  const mergedSections = []

  while (true) {
    let index = 0

    for (; index < arr.length; index++) {
      if (arr[index] !== undefined) {
        break
      }
    }

    if (index === arr.length) {
      break
    }

    let section = arr[index]
    arr[index] = undefined

    for (index++; index < arr.length; index++) {
      if (arr[index] === undefined) {
        continue
      }

      const newSection = merge(section, arr[index])

      if (newSection) {
        section = newSection
        arr[index] = undefined
      }
    }

    mergedSections.push(section)
  }

  if (recurse) {
    return mergeArray(mergedSections, false)
  } else {
    return mergedSections.reduce((a, b) => a + b, '')
  }
}

export function removeFromSet(array, set) {
  for (const part of set) array.splice(array.indexOf(part), 1)

  return array
}

export function binarySearchExists(arr, val) {
  let start = 0
  let end = arr.length - 1

  while (start <= end) {
    const mid = Math.floor((start + end) / 2)

    if (arr[mid] === val) {
      return true
    } else if (val < arr[mid]) {
      end = mid - 1
    } else {
      start = mid + 1
    }
  }

  return false
}

const RETURNS_ITSELF = x => x

export class SortedSet {
  #mapFn
  #array

  constructor(mapFn = RETURNS_ITSELF) {
    this.#mapFn = mapFn
    this.#array = []
  }

  push(val) {
    const cmpVal = this.#mapFn(val)

    let start = 0
    let end = this.#array.length - 1

    while (start <= end) {
      const mid = Math.floor((start + end) / 2)
      const other = this.#mapFn(this.#array[mid])

      if (other === cmpVal) {
        return
      } else if (cmpVal < other) {
        end = mid - 1
      } else {
        start = mid + 1
      }
    }

    this.#array.splice(start, 0, val)
  }

  get array() {
    return this.#array
  }
}
