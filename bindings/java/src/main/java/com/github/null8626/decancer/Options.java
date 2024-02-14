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
   * Predefined options with all options disabled. This is useful if you want to use decancer solely for formatting.
   */
  public static Options FORMATTER = new Options((1 << 21) - 1);

  /**
   * Predefined options that prevents decancer from curing characters from major foreign writing systems.
   */
  public static Options PURE_HOMOGLYPH = new Options(((1 << 21) - 1) ^ 3);

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
  public void retainCapitalization() {
    this.inner |= (1 << 0);
  }

  /**
   * Prevents decancer from applying the Unicode Bidirectional Algorithm. Use this only when you don't expect any right-to-left characters.
   *
   * <p>
   * NOTE: This speeds up the function call, but can break right-to-left characters.
   * It's highly recommended to also use retainArabic() and retainHebrew().
   */
  public void disableBidi() {
    this.inner |= (1 << 1);
  }

  /**
   * Prevents decancer from curing characters *with* diacritics or accents.
   *
   * <p>
   * NOTE: Decancer can still cure standalone diacritic characters, which is used in Zalgo texts.
   */
  public void retainDiacritics() {
    this.inner |= (1 << 2);
  }

  /**
   * Prevents decancer from curing all greek characters.
   */
  public void retainGreek() {
    this.inner |= (1 << 3);
  }

  /**
   * Prevents decancer from curing all cyrillic characters.
   */
  public void retainCyrillic() {
    this.inner |= (1 << 4);
  }

  /**
   * Prevents decancer from curing all hebrew characters.
   */
  public void retainHebrew() {
    this.inner |= (1 << 5);
  }

  /**
   * Prevents decancer from curing all arabic characters.
   */
  public void retainArabic() {
    this.inner |= (1 << 6);
  }

  /**
   * Prevents decancer from curing all devanagari characters.
   */
  public void retainDevanagari() {
    this.inner |= (1 << 7);
  }

  /**
   * Prevents decancer from curing all bengali characters.
   */
  public void retainBengali() {
    this.inner |= (1 << 8);
  }

  /**
   * Prevents decancer from curing all armenian characters.
   */
  public void retainArmenian() {
    this.inner |= (1 << 9);
  }

  /**
   * Prevents decancer from curing all gujarati characters.
   */
  public void retainGujarati() {
    this.inner |= (1 << 10);
  }

  /**
   * Prevents decancer from curing all tamil characters.
   */
  public void retainTamil() {
    this.inner |= (1 << 11);
  }

  /**
   * Prevents decancer from curing all thai characters.
   */
  public void retainThai() {
    this.inner |= (1 << 12);
  }

  /**
   * Prevents decancer from curing all lao characters.
   */
  public void retainLao() {
    this.inner |= (1 << 13);
  }

  /**
   * Prevents decancer from curing all burmese characters.
   */
  public void retainBurmese() {
    this.inner |= (1 << 14);
  }

  /**
   * Prevents decancer from curing all khmer characters.
   */
  public void retainKhmer() {
    this.inner |= (1 << 15);
  }

  /**
   * Prevents decancer from curing all mongolian characters.
   */
  public void retainMongolian() {
    this.inner |= (1 << 16);
  }

  /**
   * Prevents decancer from curing all chinese characters.
   */
  public void retainChinese() {
    this.inner |= (1 << 17);
  }

  /**
   * Prevents decancer from curing all katakana and hiragana characters.
   *
   * <p>
   * NOTE: To also provent decancer from curing kanji characters, use retainChinese().
   */
  public void retainJapanese() {
    this.inner |= (1 << 18);
  }

  /**
   * Prevents decancer from curing all korean characters.
   */
  public void retainKorean() {
    this.inner |= (1 << 19);
  }

  /**
   * Prevents decancer from curing all braille characters.
   */
  public void retainBraille() {
    this.inner |= (1 << 20);
  }
}
