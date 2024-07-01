/* eslint-disable */

'use strict'

import { readdirSync, readFileSync, writeFileSync, rmSync } from 'node:fs'
import { dirname, join, sep } from 'node:path'
import { execSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { rm } from 'node:fs/promises'

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), '..')
const TESTS_DIR = join(ROOT_DIR, 'tests')

rmSync(join(TESTS_DIR, 'build'), {
  recursive: true,
  force: true
})

await Promise.all(
  readdirSync(TESTS_DIR)
    .filter(f => f.endsWith('.c'))
    .map(f =>
      rm(join(TESTS_DIR, f), {
        force: true
      })
    )
)

const functions = []
let status = 0
let example = []

for (const line of readFileSync(join(ROOT_DIR, 'decancer.h'))
  .toString()
  .trim()
  .split(/\r?\n/g)
  .map(x => x.replace(/^\s*\* ?/, ''))) {
  switch (status) {
    case 0: {
      if (line.startsWith('```c')) {
        status = 1
      }

      break
    }

    case 1: {
      if (line.startsWith('```')) {
        status = 2
      } else {
        example.push(line)
      }

      break
    }

    case 2: {
      if (line.startsWith('/')) {
        status = 3
      }

      break
    }

    default: {
      try {
        const functionName = line.match(/(decancer_\w+)\(/)[1]
        const exampleCode = example
          .join('\n')
          .replace('int main(', `int ${functionName}_test(`)
        functions.push(functionName)
        
        writeFileSync(join(TESTS_DIR, `${functionName}_test.c`), exampleCode)
      } catch {}
      
      example = []
      status = 0
    }
  }
}

let testFile = `
#include <stdio.h>

#ifdef _WIN32
#pragma comment(lib, "WS2_32")
#pragma comment(lib, "Userenv")
#pragma comment(lib, "ntdll")
#endif

${functions.map(f => `int ${f}_test(void);`).join('\n')}

int main(void) {
`

for (const func of functions) {
  testFile += `
  printf("testing ${func}...\\n");
  if (${func}_test()) {
    fprintf(stderr, "error: tests for ${func} failed.\\n");
    return 1;
  }
  `
}

testFile += '\n  return 0;\n}'

writeFileSync(join(TESTS_DIR, 'test.c'), testFile)

try {
  execSync('cmake -B build .', {
    cwd: TESTS_DIR,
    stdio: 'inherit'
  })

  execSync('cmake --build build --config Debug', {
    cwd: TESTS_DIR,
    stdio: 'inherit'
  })
} catch {
  process.exit(1)
}
