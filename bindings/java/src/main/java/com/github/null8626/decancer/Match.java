package io.github.null8626.decancer;

/**
 * A match yielded by the CuredString.find method.
 *
 * @see <a href="https://github.com/null8626/decancer">github.com/null8626/decancer</a>
 * @author null8626
 */
public class Match {

  /**
   * The UTF-8 byte offset to the beginning of the match.
   */
  public final long start;

  /**
   * The UTF-8 byte offset to the end of the match (non-inclusive).
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
   */
  @Override
  public String toString() {
    return this.matched;
  }
}
