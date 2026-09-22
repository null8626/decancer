// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

package io.github.null8626.decancer;

/**
 * A cure suggestion by decancer.
 *
 * @author null8626
 * @version 4.1.0
 * @since 4.1.0
 */
public class CureSuggestion {

  /**
   * The character's UTF-8 index in the original input string.
   *
   * @since 4.1.0
   */
  public final long oldIndex;

  /**
   * The translation's UTF-8 index in the suggested cured string or -1 if it's removed.
   *
   * @since 4.1.0
   */
  public final long newIndex;

  /**
   * The suggested translation.
   *
   * @since 4.1.0
   */
  public final String translation;

  private CureSuggestion(final long oldIndex, final long newIndex, final String translation) {
    this.oldIndex = oldIndex;
    this.newIndex = newIndex;
    this.translation = translation;
  }

  /**
   * @return String The suggested translation.
   * @since 4.1.0
   */
  @Override
  public String toString() {
    return translation;
  }
}
