let wasm;
let initiated = false;
let WASM_VECTOR_LEN = 0;
let cachedUint8Memory0;

function getUint8Memory0() {
  return cachedUint8Memory0.byteLength === 0 ? (cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer)) : cachedUint8Memory0;
}

const cachedTextEncoder = new TextEncoder('utf-8');
const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
  ? function (arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
} : function (arg, view) {
  
  const buf = cachedTextEncoder.encode(arg);
  view.set(buf);
  
  return {
    read: arg.length,
    written: buf.length
  };
});

function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length);
    getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }
  
  let len = arg.length;
  let ptr = malloc(len);
  const mem = getUint8Memory0();
  let offset = 0;
  
  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7F) break;
    mem[ptr + offset] = code;
  }
  
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3);
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);
    offset += ret.written;
  }
  
  WASM_VECTOR_LEN = offset;
  return ptr;
}

let cachedInt32Memory0;

function getInt32Memory0() {
  if (cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  
  return cachedInt32Memory0;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

export function decancer(raw_input) {
  if (typeof raw_input !== "string") {
    throw new TypeError(`expected 'raw_input' to be string, got ${typeof raw_input}`);
  }
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passStringToWasm0(raw_input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.decancer(retptr, ptr0, len0);
    var r0 = getInt32Memory0()[retptr / 4 + 0];
    var r1 = getInt32Memory0()[retptr / 4 + 1];
    return getStringFromWasm0(r0, r1);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_free(r0, r1);
  }
}

export function contains(input, other) {
  if (typeof input !== "string") {
    throw new TypeError(`expected 'input' to be string, got ${typeof input}`);
  } else if (typeof other !== "string") {
    throw new TypeError(`expected 'other' to be string, got ${typeof other}`);
  }
  
  const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ptr1 = passStringToWasm0(other, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len1 = WASM_VECTOR_LEN;
  
  return wasm.contains(ptr0, len0, ptr1, len1) !== 0;
}

async function load(module, imports) {
  if (typeof Response === 'function' && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === 'function') {
      return await WebAssembly.instantiateStreaming(module, imports);
    }
  
    const bytes = await module.arrayBuffer();
    return await WebAssembly.instantiate(bytes, imports);
  } else {
    const instance = await WebAssembly.instantiate(module, imports);
  
    if (instance instanceof WebAssembly.Instance) {
      return { instance, module };
    } else {
      return instance;
    }
  }
}

export default async function init() {
  if (initiated) {
    return decancer;
  }
  
  const { instance, module } = await load(await fetch(new URL('./bin/decancer.wasm', import.meta.url)), {
    wbg: {}
  });
  
  wasm = instance.exports;
  init.__wbindgen_wasm_module = module;
  
  cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  
  Object.assign(decancer, { contains });
  return decancer;
}