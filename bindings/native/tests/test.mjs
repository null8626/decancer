// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

/* eslint-disable */

import {
  BINDINGS_DIR,
  SPDX_LICENSE_COMMENTS
} from '../../../scripts/constants.mjs'
import { readdirSync, readFileSync, writeFileSync, rmSync } from 'node:fs'
import { execSync } from 'node:child_process'
import { rm } from 'node:fs/promises'
import process from 'node:process'
import { join } from 'node:path'

const CURRENT_DIR = join(BINDINGS_DIR, 'native', 'tests')
const NATIVE_DIR = join(CURRENT_DIR, '..')

rmSync(join(NATIVE_DIR, 'build'), {
  recursive: true,
  force: true
})

await Promise.all(
  readdirSync(NATIVE_DIR)
    .filter(f => f.endsWith('test.c'))
    .map(f =>
      rm(join(NATIVE_DIR, f), {
        force: true
      })
    )
)

const functions = []
let status = 0
let example = []

for (const line of readFileSync(join(NATIVE_DIR, 'decancer.h'))
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

        writeFileSync(join(CURRENT_DIR, `${functionName}_test.c`), exampleCode)
      } catch {}

      example = []
      status = 0
    }
  }
}

let testFile = `
${SPDX_LICENSE_COMMENTS}

#include <stdio.h>

#ifdef _WIN32
#pragma comment(lib, "WS2_32")
#pragma comment(lib, "Userenv")
#pragma comment(lib, "ntdll")
#endif

int extra_tests();
${functions.map(f => `int ${f}_test(void);`).join('\n')}

int main() {
  printf("testing other tests...\\n");
  if (extra_tests()) {
    fprintf(stderr, "error: other tests failed.\\n");
    return 1;
  }
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

writeFileSync(join(CURRENT_DIR, 'test.c'), testFile)

try {
  execSync('cmake -B build . && cmake --build build --config Debug', {
    cwd: CURRENT_DIR,
    stdio: 'inherit'
  })
} catch {
  process.exit(1)
}
