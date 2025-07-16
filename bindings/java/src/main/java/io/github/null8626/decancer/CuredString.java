package io.github.null8626.decancer;

import com.fizzed.jne.NativeTarget;
import com.fizzed.jne.OperatingSystem;
import cz.adamh.utils.NativeUtils;

/**
 * A small wrapper around the String data type for comparison purposes.
 *
 * <p>
 * This is used because imperfections from translations can happen, thus this is used to provide comparison functions that are not as strict and can detect similar-looking characters (e.g: letter I and lowercase L)
 * </p>
 *
 * @author null8626
 * @version 3.3.3
 * @since 3.0.0
 */
public class CuredString implements AutoCloseable {

  private long inner;

  static {
    String osName = System.getProperty("os.name");
    String archName = System.getProperty("os.arch");
    String rustTarget = "<unknown>";

    try {
      final NativeTarget target = NativeTarget.detect();

      String libPrefix = "lib";
      String fileExtension = "so";

      final OperatingSystem os = target.getOperatingSystem();
      osName = os.getDescriptor();

      archName = target.getHardwareArchitecture().getDescriptor();
      rustTarget = target.toRustTarget();

      switch (os) {
        case WINDOWS:
          libPrefix = "";
          fileExtension = "dll";
          break;
        case MACOS:
          fileExtension = "dylib";
        default:
          break;
      }

      if (System.getProperty("DECANCER_TESTING", "0").equals("1")) {
        System.loadLibrary("decancer-" + rustTarget);
      } else {
        NativeUtils.loadLibraryFromJar(
          "/" + libPrefix + "decancer-" + rustTarget + "." + fileExtension
        );
      }
    } catch (final Throwable err) {
      throw new RuntimeException(
        "[" +
        rustTarget +
        "] this operating system (" +
        osName +
        ") and/or architecture (" +
        archName +
        ") is not supported.\noriginal error:\n" +
        err.getMessage()
      );
    }
  }

  private static native long cure(String input, int options);

  /**
   * Iterates throughout this string and yields every similar-looking match.
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The other string to match with.
   * @see findMultiple
   * @return Match[] An array of Match objects containing every similar-looking match.
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.1.0
   */
  public native Match[] find(String other);

  /**
   * Iterates throughout this string and returns an array of every similar-looking match. Unlike find, this method also takes note of overlapping matches and merges them together.
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The list of strings to match with.
   * @see find
   * @return Match[] An array of Match objects containing every similar-looking match.
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.1.1
   */
  public native Match[] findMultiple(String[] other);

  /**
   * Censors every match of a string with a repetition of a character in-place.
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The other string to match with.
   * @param with The character to repeat.
   * @see censorMultiple
   * @throws IllegalArgumentException If the character to repeat is a UTF-16 surrogate.
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.1.1
   */
  public native void censor(String other, char with);

  /**
   * Censors every matches from an array of strings with a repetition of a character in-place.
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The list of strings to match with.
   * @param with The character to repeat.
   * @see censor
   * @throws IllegalArgumentException If the character to repeat is a UTF-16 surrogate.
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.1.1
   */
  public native void censorMultiple(String[] other, char with);

  /**
   * Replaces every match of a string with another string in-place.
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The other string to match with.
   * @param with The other string to replace with.
   * @see replaceMultiple
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.1.1
   */
  public native void replace(String other, String with);

  /**
   * Replaces every matches from an array of strings with another string in-place.
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The list of strings to match with.
   * @param with The other string to replace with.
   * @see replace
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.1.1
   */
  public native void replaceMultiple(String[] other, String with);

  /**
   * Checks if this object is similar with another string
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The other string to match with.
   * @return boolean Whether this object is similar with another string.
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.0.0
   */
  public native boolean equals(String other);

  /**
   * Checks if this object similarly starts with another string.
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The other string to match with.
   * @return boolean Whether this object similarly starts with another string.
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.0.0
   */
  public native boolean startsWith(String other);

  /**
   * Checks if this object similarly ends with another string.
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The other string to match with.
   * @return boolean Whether this object similarly ends with another string.
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.0.0
   */
  public native boolean endsWith(String other);

  /**
   * Checks if this object similarly contains another string.
   *
   * <p>
   * This comparison is case-insensitive.
   * </p>
   *
   * @param other The other string to match with.
   * @return boolean Whether this object similarly contains another string.
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.0.0
   */
  public native boolean contains(String other);

  /**
   * Coerces this object to a String.
   *
   * <p>
   * This comparison is case-insensitive.<br>
   *
   * <b>WARNING:</b> It's highly NOT recommended to use Java's comparison methods after calling this. The string output is NOT meant to be displayed visually.
   * </p>
   *
   * @return String The String representation of this object.
   * @throws NullPointerException If close() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.0.0
   */
  @Override
  public native String toString();

  /**
   * Destroys and frees the memory used by this object.
   *
   * <p>
   * Repeated calls to this method is fine and does not guarantee a double-free.<br>
   * Any subsequent String objects from toString() calls can still be used after this.
   * </p>
   *
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.3.0
   */
  @Override
  public native void close();

  /**
   * Cures a string with decancer's default options.
   *
   * <p>
   * Output will always be in lowercase and bidirectionally reordered in order to treat right-to-left characters.<br>
   * Therefore, the output of this function should NOT be displayed visually.
   * </p>
   *
   * @param input The string to cure.
   * @throws IllegalArgumentException If the string is malformed to the point where it's not possible to apply unicode's bidirectional algorithm to it.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.0.0
   */
  public CuredString(String input) {
    this.inner = CuredString.cure(input, 0);
  }

  /**
   * Cures a string with the specified options.
   *
   * @param input The string to cure.
   * @param options The explicit options.
   * @throws IllegalArgumentException If the string is malformed to the point where it's not possible to apply unicode's bidirectional algorithm to it.
   * @throws RuntimeException If a Rust panic occurs.
   * @since 3.0.0
   */
  public CuredString(String input, Options options) {
    this.inner = CuredString.cure(input, options.inner);
  }
}
