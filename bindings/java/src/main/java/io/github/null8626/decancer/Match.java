package io.github.null8626.decancer;

/**
 * A match yielded by the CuredString.find() method.
 *
 * @author null8626
 * @version 3.3.3
 * @since 3.1.0
 */
public class Match {

  /**
   * The UTF-8 byte offset to the beginning of the match.
   *
   * @since 3.1.0
   */
  public final long start;

  /**
   * The UTF-8 byte offset to the end of the match (non-inclusive).
   *
   * @since 3.1.0
   */
  public final long end;

  private final String matched;

  private Match(final long start, final long end, final String matched) {
    this.start = start;
    this.end = end;
    this.matched = matched;
  }

  /**
   * @return String The matched portion of the original String.
   * @since 3.1.0
   */
  @Override
  public String toString() {
    return this.matched;
  }
}
