// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

package io.github.null8626.decancer;

/**
 * A configuration class where you can customize decancer's behavior.
 *
 * <p>By default, decancer cures as much characters as possible and turns all of the output
 * characters to lowercase.
 *
 * @author null8626
 * @version 3.3.3
 * @since 3.0.0
 */
public final class Options {

  /**
   * Raw native bitflags.
   *
   * @since 3.0.0
   */
  protected int inner;

  /**
   * Predefined configuration with all options enabled.
   *
   * @since 3.2.0
   */
  public static Options ALL = new Options(0x3ffffff);

  /**
   * Predefined configuration that prevents decancer from curing characters from major foreign
   * writing systems, including diacritics.
   *
   * @since 3.0.0
   */
  public static Options PURE_HOMOGLYPH = new Options(0x7ffff8);

  /**
   * Creates a new Options object with decancer's default options.
   *
   * <p>By default, all options here are disabled, which means that decancer cures as much
   * characters as possible and turns all of the output characters to lowercase.
   *
   * @since 3.0.0
   */
  public Options() {
    inner = 0;
  }

  private Options(final int inner) {
    this.inner = inner;
  }

  /**
   * Prevents decancer from changing all characters to lowercase. Therefore, if the input character
   * is in uppercase, the output character will be in uppercase as well.
   *
   * <p>Many confusables are neither an uppercase or a lowercase character.<br>
   * Therefore, the decancer defaults to displaying the translation in lowercase.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainCapitalization() {
    inner |= 1;
    return this;
  }

  /**
   * Prevents decancer from applying the Unicode Bidirectional Algorithm. Use this only when you
   * don't expect any right-to-left characters.
   *
   * <p><b>WARNING:</b> This speeds up the function call, but can BREAK right-to-left characters.
   * It's highly recommended to also use retainArabic() and retainHebrew().
   *
   * @see retainArabic
   * @see retainHebrew
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options disableBidi() {
    inner |= (1 << 1);
    return this;
  }

  /**
   * Prevents decancer from applying leetspeak comparisons in comparison methods.
   *
   * @since 4.0.0
   */
  public Options disableLeetspeak() {
    inner |= (1 << 2);
    return this;
  }

  /**
   * Prevents decancer from curing characters *with* diacritics or accents.
   *
   * <p>Decancer can still cure standalone diacritic characters, which is used in Zalgo texts.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainDiacritics() {
    inner |= (1 << 3);
    return this;
  }

  /**
   * Prevents decancer from curing all greek characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainGreek() {
    inner |= (1 << 4);
    return this;
  }

  /**
   * Prevents decancer from curing all cyrillic characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainCyrillic() {
    inner |= (1 << 5);
    return this;
  }

  /**
   * Prevents decancer from curing all hebrew characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainHebrew() {
    inner |= (1 << 6);
    return this;
  }

  /**
   * Prevents decancer from curing all arabic characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainArabic() {
    inner |= (1 << 7);
    return this;
  }

  /**
   * Prevents decancer from curing all devanagari characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainDevanagari() {
    inner |= (1 << 8);
    return this;
  }

  /**
   * Prevents decancer from curing all bengali characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainBengali() {
    inner |= (1 << 9);
    return this;
  }

  /**
   * Prevents decancer from curing all armenian characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainArmenian() {
    inner |= (1 << 10);
    return this;
  }

  /**
   * Prevents decancer from curing all gujarati characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainGujarati() {
    inner |= (1 << 11);
    return this;
  }

  /**
   * Prevents decancer from curing all tamil characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainTamil() {
    inner |= (1 << 12);
    return this;
  }

  /**
   * Prevents decancer from curing all thai characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainThai() {
    inner |= (1 << 13);
    return this;
  }

  /**
   * Prevents decancer from curing all lao characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainLao() {
    inner |= (1 << 14);
    return this;
  }

  /**
   * Prevents decancer from curing all burmese characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainBurmese() {
    inner |= (1 << 15);
    return this;
  }

  /**
   * Prevents decancer from curing all khmer characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainKhmer() {
    inner |= (1 << 16);
    return this;
  }

  /**
   * Prevents decancer from curing all mongolian characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainMongolian() {
    inner |= (1 << 17);
    return this;
  }

  /**
   * Prevents decancer from curing all chinese characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainChinese() {
    inner |= (1 << 18);
    return this;
  }

  /**
   * Prevents decancer from curing all katakana and hiragana characters.
   *
   * <p>To also prevent decancer from curing kanji characters, use retainChinese().
   *
   * @see retainChinese
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainJapanese() {
    inner |= (1 << 19);
    return this;
  }

  /**
   * Prevents decancer from curing all korean characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainKorean() {
    inner |= (1 << 20);
    return this;
  }

  /**
   * Prevents decancer from curing all braille characters.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainBraille() {
    inner |= (1 << 21);
    return this;
  }

  /**
   * Prevents decancer from curing all emojis.
   *
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.0.0
   */
  public Options retainEmojis() {
    inner |= (1 << 22);
    return this;
  }

  /**
   * Prevents decancer from curing all turkish characters.
   *
   * <p>To also prevent decancer from curing the uppercase dotted i character (İ), use
   * retainCapitalization().
   *
   * @see retainCapitalization
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.2.4
   */
  public Options retainTurkish() {
    inner |= (1 << 23);
    return this;
  }

  /**
   * Removes all non-ASCII characters from the result.
   *
   * @see alphanumericOnly
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.2.0
   */
  public Options asciiOnly() {
    inner |= (1 << 24);
    return this;
  }

  /**
   * Removes all non-alphanumeric characters from the result.
   *
   * @see asciiOnly
   * @return Options A reference to this object to allow for method chaining.
   * @since 3.2.0
   */
  public Options alphanumericOnly() {
    inner |= (1 << 25);
    return this;
  }
}
