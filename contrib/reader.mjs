import { readFileSync } from 'node:fs';
import Decancer from './cancer.mjs';

/**
 * Reads the array structure `unsigned char, unsigned int[]` from the buffer.
 * The first 8 bytes being the array length, and the rest being 32-bit array.
 * @param {Buffer} buffer The buffer.
 * @param {number} offset The offset in bytes.
 * @returns {number[]} The unsigned 32-bit array.
 */
function readSimpleArray(buffer, offset) {
  return Array.from({ length: buffer[offset] }, (_, i) =>
    buffer.readUInt32LE(offset + 1 + i * 4)
  );
}

export default function read(filename) {
  // The file
  const buf = readFileSync(filename);

  // Offsets
  const numericalOffset = buf.readUInt16LE(0);
  const miscCaseSensitiveOffset = buf.readUInt16LE(2);
  const miscOffset = buf.readUInt16LE(4);
  const alphabeticalPatternOffset = buf.readUInt16LE(6);
  const alphabeticalOffset = buf.readUInt16LE(8);
  const similarOffset = buf.readUInt16LE(10);

  // Simple arrays
  const numerical = readSimpleArray(buf, numericalOffset);
  const alphabeticalPattern = readSimpleArray(buf, alphabeticalPatternOffset);

  // Misc case sensitive
  const miscCaseSensitiveSize = buf[miscCaseSensitiveOffset];
  const miscCaseSensitive = new Map();
  let currentOffset = miscCaseSensitiveOffset + 1;

  for (let i = 0; i < miscCaseSensitiveSize; i++) {
    const translation = Array.from(
      buf.subarray(currentOffset + 1, currentOffset + 1 + buf[currentOffset])
    );
    currentOffset += 1 + translation.length;

    const confusables = readSimpleArray(buf, currentOffset);
    currentOffset += 1 + confusables.length * 4;

    miscCaseSensitive.set(String.fromCharCode(...translation), confusables);
  }
  
  // Misc
  const miscSize = buf[miscOffset];
  const misc = new Map();
  currentOffset = miscOffset + 1;

  for (let i = 0; i < miscSize; i++) {
    const translation = Array.from(
      buf.subarray(currentOffset + 1, currentOffset + 1 + buf[currentOffset])
    );
    currentOffset += 1 + translation.length;

    const confusables = readSimpleArray(buf, currentOffset);
    currentOffset += 1 + confusables.length * 4;

    misc.set(String.fromCharCode(...translation), confusables);
  }

  // Alphabetical
  currentOffset = alphabeticalOffset;
  const alphabetical = Array.from({ length: 26 }, () => {
    const output = readSimpleArray(buf, currentOffset);
    currentOffset += 1 + output.length * 4;

    return output;
  });

  // Similar
  const similarLength = buf[similarOffset];
  const similar = [];
  currentOffset = similarOffset + 1;

  for (let i = 0; i < similarLength; i++) {
    const length = buf[currentOffset];
    currentOffset++;

    similar.push(
      Array.from(buf.subarray(currentOffset, currentOffset + length)).map(x =>
        String.fromCharCode(x)
      )
    );
    currentOffset += length;
  }

  return new Decancer({
    numerical,
    alphabeticalPattern,
    miscCaseSensitive,
    misc,
    alphabetical,
    similar
  });
}
