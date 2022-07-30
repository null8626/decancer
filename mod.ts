import type Decancer from './node/src/typings'

let wasm: any
let initiated = false
let WASM_VECTOR_LEN = 0
let cachedUint8Memory0: Uint8Array

function getUint8Memory() {
  return cachedUint8Memory0.byteLength === 0 ? (cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer)) : cachedUint8Memory0
}

// @ts-ignore
const cachedTextEncoder = new TextEncoder('utf-8')

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function' ? cachedTextEncoder.encodeInto : function (arg: string, view: Uint8Array) {
  const buf = cachedTextEncoder.encode(arg)
  view.set(buf)

  return {
    read: arg.length,
    written: buf.length
  }
})

function passStringToWasm(arg: any, malloc: any, realloc: any) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg)
    const ptr = malloc(buf.length)

    getUint8Memory().subarray(ptr, ptr + buf.length).set(buf)
    WASM_VECTOR_LEN = buf.length
    
    return ptr
  }

  let len = arg.length
  let ptr = malloc(len)
  
  const mem = getUint8Memory()
  let offset = 0
  
  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset)
    if (code > 0x7F) {
      break
    }
    
    mem[ptr + offset] = code
  }

  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset)
    }
    
    ptr = realloc(ptr, len, len = offset + arg.length * 3)
    
    const view = getUint8Memory().subarray(ptr + offset, ptr + len)
    const ret = encodeString(arg, view)
    offset += ret.written!
  }

  WASM_VECTOR_LEN = offset
  return ptr
}

let cachedInt32Memory0: Int32Array

function getInt32Memory() {
  if (cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer)
  }
  return cachedInt32Memory0
}

const cachedTextDecoder = new TextDecoder('utf-8', {
  ignoreBOM: true,
  fatal: true
})

cachedTextDecoder.decode()

function getStringFromWasm(ptr: number, len: number) {
  return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len))
}

export function decancer(raw_input: string): string {
  if (typeof raw_input !== 'string') {
    throw new TypeError(`expected 'raw_input' to be string, got ${typeof raw_input}`)
  }

  let r0: number, r1: number

  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16)
    const ptr0 = passStringToWasm(raw_input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc)
    const len0 = WASM_VECTOR_LEN
    
    wasm.decancer(retptr, ptr0, len0)
    r0 = getInt32Memory()[retptr / 4 + 0]
    r1 = getInt32Memory()[retptr / 4 + 1]
    
    return getStringFromWasm(r0, r1)
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16)

    // @ts-ignore
    wasm.__wbindgen_free(r0, r1)
  }
}

export function contains(input: string, other: string): boolean {
  if (typeof input !== 'string') {
    throw new TypeError(`expected 'input' to be string, got ${typeof input}`)
  } else if (typeof other !== 'string') {
    throw new TypeError(`expected 'other' to be string, got ${typeof other}`)
  }

  const ptr0 = passStringToWasm(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc)
  const len0 = WASM_VECTOR_LEN
  
  const ptr1 = passStringToWasm(other, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc)
  const len1 = WASM_VECTOR_LEN

  return wasm.contains(ptr0, len0, ptr1, len1) !== 0
}

async function load(module: Response) {
  return await WebAssembly.instantiate(await module.arrayBuffer())
}

export default async function init() {
  if (initiated) {
    return decancer
  }
  
  const {
    instance,
    module
  } = await load(await fetch('https://raw.githubusercontent.com/null8626/decancer/main/wasm/bin/decancer.wasm'))
  
  wasm = instance.exports

  // @ts-ignore
  init.__wbindgen_wasm_module = module
  
  cachedInt32Memory0 = new Int32Array(wasm.memory.buffer)
  cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer)
  
  Object.assign(decancer, {
    contains
  })
  
  initiated = true
  return decancer
}

export type { Decancer }