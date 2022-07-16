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
  
  save(filename) {
    write(this, filename);
  }
}
