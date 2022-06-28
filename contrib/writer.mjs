import Decancer from "./cancer.mjs";
import { writeFileSync } from "node:fs";

/**
 * The decancer writer - converting a Decancer object into a stream of uncompressed bytes.
 */
class Writer {
  #buffer;
  
  constructor() {
    this.#buffer = Buffer.allocUnsafe(12); // 12 bytes is header size.
  }

  /**
   * Writes to the buffer header.
   * @param {number} index The zero based index. NOT in bytes.
   * @param {number} data The data, min: 0, max: 65535
   */
  writeHeader(index, data) {
    this.#buffer.writeUint16LE(data, index * 2); // 2 is the size of u16.
  }

  /**
   * Writes a u32 array format to the buffer.
   * @param {number[]} arr The unsigned 32-bit array.
   * @returns {number} An offset pointing to the array length (u8) then the array a byte later.
   */
  writeUInt32Array(arr) {
    const outputOffset = this.#buffer.byteLength;
    
    this.#buffer = Buffer.concat([
      this.#buffer,
      new Uint8Array([arr.length]),
      new Uint8Array((new Uint32Array(arr)).buffer)
    ]);

    return outputOffset;
  }

  /**
   * Writes a u8 array format to the buffer.
   * @param {number[]} arr The unsigned 8-bit array.
   * @returns {number} An offset pointing to the array length (u8) then the array a byte later.
   */
  writeUInt8Array(arr) {
    const outputOffset = this.#buffer.byteLength;
    
    this.#buffer = Buffer.concat([
      this.#buffer,
      new Uint8Array([arr.length, ...arr])
    ]);
    
    return outputOffset;
  }

  /**
   * Writes a byte to the writer.
   * @param {number} byte The byte.
   * @returns {number} The offset pointing to the byte.
   */
  writeByte(byte) {
    const outputOffset = this.#buffer.byteLength;
    this.#buffer = Buffer.concat([
      this.#buffer,
      new Uint8Array([byte])
    ]);

    return outputOffset;
  }

  /**
   * Writes the output buffer into a binary file.
   * @param {string} filename The file name.
   */
  write(filename) {
    writeFileSync(filename, this.#buffer);
  }
}

/**
 * Writes it all back to a binary.
 * @param {Decancer} decancer The decancer instance. 
 * @param {string} filename The file name.
 */
export default function write(decancer, filename) {
  const writer = new Writer();

  writer.writeHeader(0, writer.writeUInt32Array(decancer.numerical));
  
  writer.writeHeader(1, writer.writeByte(decancer.miscCaseSensitive.size));
  for (const [translation, confusables] of [...decancer.miscCaseSensitive.entries()].map(([K, V]) => [K.split('').map(x => x.charCodeAt()), V])) {
    writer.writeUInt8Array(translation);
    writer.writeUInt32Array(confusables);
  }

  writer.writeHeader(2, writer.writeByte(decancer.misc.size));
  for (const [translation, confusables] of [...decancer.misc].map(([K, V]) => [K.charCodeAt(), V])) {
    writer.writeByte(translation);
    writer.writeUInt32Array(confusables); 
  }

  writer.writeHeader(3, writer.writeUInt32Array(decancer.alphabeticalPattern));

  writer.writeHeader(4, writer.writeUInt32Array(decancer.alphabetical[0]));
  for (let i = 1; i < 26; i++) {
    writer.writeUInt32Array(decancer.alphabetical[i]);
  }

  writer.writeHeader(5, writer.writeByte(decancer.similar.length));
  for (const sim of decancer.similar.map(x => x.map(y => y.charCodeAt()))) {
    writer.writeUInt8Array(sim);
  }

  writer.write(filename);
}