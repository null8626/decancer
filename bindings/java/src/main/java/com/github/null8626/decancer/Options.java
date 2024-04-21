package com.github.null8626.decancer;

/**
 * A configuration class where you can customize decancer's behavior.
 *
 * <p>
 * By default, decancer cures as much characters as possible and turns all of the output characters to lowercase.
 *
 * @see <a href="https://github.com/null8626/decancer">github.com/null8626/decancer</a>
 * @author null8626
 */
public final class Options {

  /**
   * Raw native bitflags.
   */
  protected int inner;

  /**
   * Predefined configuration with all options enabled.
   */
  public static Options ALL = new Options(0xffffff);

  /**
   * Predefined configuration that prevents decancer from curing characters from major foreign writing systems, including diacritics.
   */
  public static Options PURE_HOMOGLYPH = new Options(0x1ffffc);

  /**
   * Creates a new Options object with decancer's default options.
   *
   * <p>
   * By default, all options here are disabled, which means that decancer cures as much characters as possible and turns all of the output characters to lowercase.
   */
  public Options() {
    this.inner = 0;
  }

  private Options(final int inner) {
    this.inner = inner;
  }

  /**
   * Prevents decancer from changing all characters to lowercase. Therefore, if the input character is in uppercase, the output character will be in uppercase as well.
   *
   * <p>
   * NOTE: Many confusables are neither an uppercase or a lowercase character.
   * Therefore, the decancer defaults to displaying the translation in lowercase.
   */
  public Options retainCapitalization() {
    this.inner |= (1 << 0);
    return this;
  }

  /**
   * Prevents decancer from applying the Unicode Bidirectional Algorithm. Use this only when you don't expect any right-to-left characters.
   *
   * <p>
   * NOTE: This speeds up the function call, but can break right-to-left characters.
   * It's highly recommended to also use retainArabic() and retainHebrew().
   */
  public Options disableBidi() {
    this.inner |= (1 << 1);
    return this;
  }

  /**
   * Prevents decancer from curing characters *with* diacritics or accents.
   *
   * <p>
   * NOTE: Decancer can still cure standalone diacritic characters, which is used in Zalgo texts.
   */
  public Options retainDiacritics() {
    this.inner |= (1 << 2);
    return this;
  }

  /**
   * Prevents decancer from curing all greek characters.
   */
  public Options retainGreek() {
    this.inner |= (1 << 3);
    return this;
  }

  /**
   * Prevents decancer from curing all cyrillic characters.
   */
  public Options retainCyrillic() {
    this.inner |= (1 << 4);
    return this;
  }

  /**
   * Prevents decancer from curing all hebrew characters.
   */
  public Options retainHebrew() {
    this.inner |= (1 << 5);
    return this;
  }

  /**
   * Prevents decancer from curing all arabic characters.
   */
  public Options retainArabic() {
    this.inner |= (1 << 6);
    return this;
  }

  /**
   * Prevents decancer from curing all devanagari characters.
   */
  public Options retainDevanagari() {
    this.inner |= (1 << 7);
    return this;
  }

  /**
   * Prevents decancer from curing all bengali characters.
   */
  public Options retainBengali() {
    this.inner |= (1 << 8);
    return this;
  }

  /**
   * Prevents decancer from curing all armenian characters.
   */
  public Options retainArmenian() {
    this.inner |= (1 << 9);
    return this;
  }

  /**
   * Prevents decancer from curing all gujarati characters.
   */
  public Options retainGujarati() {
    this.inner |= (1 << 10);
    return this;
  }

  /**
   * Prevents decancer from curing all tamil characters.
   */
  public Options retainTamil() {
    this.inner |= (1 << 11);
    return this;
  }

  /**
   * Prevents decancer from curing all thai characters.
   */
  public Options retainThai() {
    this.inner |= (1 << 12);
    return this;
  }

  /**
   * Prevents decancer from curing all lao characters.
   */
  public Options retainLao() {
    this.inner |= (1 << 13);
    return this;
  }

  /**
   * Prevents decancer from curing all burmese characters.
   */
  public Options retainBurmese() {
    this.inner |= (1 << 14);
    return this;
  }

  /**
   * Prevents decancer from curing all khmer characters.
   */
  public Options retainKhmer() {
    this.inner |= (1 << 15);
    return this;
  }

  /**
   * Prevents decancer from curing all mongolian characters.
   */
  public Options retainMongolian() {
    this.inner |= (1 << 16);
    return this;
  }

  /**
   * Prevents decancer from curing all chinese characters.
   */
  public Options retainChinese() {
    this.inner |= (1 << 17);
    return this;
  }

  /**
   * Prevents decancer from curing all katakana and hiragana characters.
   *
   * <p>
   * NOTE: To also provent decancer from curing kanji characters, use retainChinese().
   */
  public Options retainJapanese() {
    this.inner |= (1 << 18);
    return this;
  }

  /**
   * Prevents decancer from curing all korean characters.
   */
  public Options retainKorean() {
    this.inner |= (1 << 19);
    return this;
  }

  /**
   * Prevents decancer from curing all braille characters.
   */
  public Options retainBraille() {
    this.inner |= (1 << 20);
    return this;
  }

  /**
   * Prevents decancer from curing all emojis.
   */
  public Options retainEmojis() {
    this.inner |= (1 << 21);
    return this;
  }

  /**
   * Removes all non-ASCII characters from the result.
   */
  public Options asciiOnly() {
    this.inner |= (1 << 22);
    return this;
  }

  /**
   * Removes all non-alphanumeric characters from the result.
   */
  public Options alphanumericOnly() {
    this.inner |= (1 << 23);
    return this;
  }
}
