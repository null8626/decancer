import write from './writer.mjs';

/**
 * Decancer (tm)
 *
 * The binary data.
 */
export default class Decancer {
  constructor({
    numerical,
    alphabeticalPattern,
    miscCaseSensitive,
    misc,
    alphabetical,
    similar
  }) {
    this.numerical = numerical;
    this.alphabeticalPattern = alphabeticalPattern;
    this.miscCaseSensitive = miscCaseSensitive;
    this.misc = misc;
    this.alphabetical = alphabetical;
    this.similar = similar;
  }

  /**
   * Returns the amount of characters supported.
   */
  get length() {
    return (
      this.numerical.length * 10 +
      this.alphabeticalPattern.length * 26 +
      Array.from(this.miscCaseSensitive.values()).reduce(
        (a, b) => a + b.length,
        0
      ) +
      Array.from(this.misc.values()).reduce((a, b) => a + b.length, 0) +
      this.alphabetical.reduce((a, b) => a + b.length, 0)
    );
  }

  save(filename) {
    write(this, filename);
  }
}
