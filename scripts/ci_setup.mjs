/* eslint-disable */

import { appendFileSync } from 'node:fs'
import process from 'node:process'
import { EOL } from 'node:os'

let input

if (process.env.DISPATCH_INPUT.length) {
  input = JSON.parse(process.env.DISPATCH_INPUT)

  if (
    input.release_version === 'null' &&
    (input.create_release === 'true' ||
      input.publish_node === 'true' ||
      input.publish_java === 'true')
  ) {
    console.error(
      'error: release_version must NOT be null if create_release, publish_node, or publish_java are enabled'
    )

    process.exit(1)
  }
} else {
  const response = await fetch(
    'https://api.github.com/repos/null8626/decancer/compare/HEAD~1...HEAD',
    {
      headers: {
        Authorization: `Bearer ${process.env.GITHUB_TOKEN}`,
        'Content-Type': 'application/json'
      }
    }
  )

  const { files } = await response.json()

  const releaseVersion = /^\d+\.\d+\.\d+$/.test(process.env.COMMIT_MESSAGE)
    ? process.env.COMMIT_MESSAGE
    : null

  const isRelease = releaseVersion !== null

  input = {
    release_version: releaseVersion,
    create_release: isRelease,
    publish_node: isRelease,
    publish_java: isRelease,
    core_affected: files.some(
      ({ filename }) =>
        filename.startsWith('core/src/') ||
        filename === 'core/bin/codepoints.bin'
    ),
    node_affected: files.some(
      ({ filename }) =>
        filename.startsWith('bindings/node/src') ||
        filename === 'bindings/node/test.cjs'
    ),
    wasm_affected: files.some(({ filename }) =>
      filename.startsWith('bindings/wasm/')
    ),
    java_affected: files.some(({ filename }) =>
      filename.startsWith('bindings/java/src')
    ),
    native_affected: files.some(
      ({ filename }) =>
        filename.startsWith('bindings/native/src') ||
        filename === 'bindings/native/decancer.h'
    ),
    native_docs_affected: files.some(
      ({ filename }) =>
        filename.startsWith('bindings/native/docs') ||
        filename === 'bindings/native/decancer.h'
    ),
    go_affected: files.some(
      ({ filename }) =>
        filename.startsWith('bindings/go') && filename.endsWith('.go')
    )
  }
}

if (input.create_release.toString() === 'true') {
  input.core_affected = true
  input.wasm_affected = true
  input.java_affected = true
  input.native_affected = true
  input.native_docs_affected = true
  input.go_affected = true
}

if (input.publish_node.toString() === 'true') {
  input.node_affected = true
}

if (input.publish_java.toString() === 'true') {
  input.java_affected = true
}

if (input.core_affected.toString() === 'true') {
  input.wasm_affected = true
  input.java_affected = true
}

appendFileSync(
  process.env.GITHUB_OUTPUT,
  Object.entries(input).reduce((a, [k, v]) => `${a}${k}=${v}${EOL}`, '')
)
