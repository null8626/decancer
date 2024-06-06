package com.github.null8626.decancer;

import com.fizzed.jne.NativeTarget;
import com.fizzed.jne.OperatingSystem;
import cz.adamh.utils.NativeUtils;

/**
 * A small wrapper around the String data type for comparison purposes.
 *
 * <p>
 * This is used because imperfections from translations can happen, thus this is used to provide comparison functions that are not as strict and can detect similar-looking characters (e.g: letter I and lowercase L)
 *
 * @see <a href="https://github.com/null8626/decancer">github.com/null8626/decancer</a>
 * @author null8626
 */
public class CuredString {

  private long inner;

  private static boolean isJUnit() {
    boolean foundJUnit = false;
    boolean local = true;

    for (StackTraceElement element : Thread.currentThread().getStackTrace()) {
      final String className = element.getClassName();

      if (className.startsWith("org.junit")) {
        foundJUnit = true;
      } else if (!className.startsWith("com.github.null8626.decancer") &&
                 !className.contains("org.gradle") &&
                 !className.startsWith("jdk.") &&
                 !className.startsWith("java.")) {
        local = false;
      }
    }

    return foundJUnit && local;
  }

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

      if (CuredString.isJUnit()) {
        System.loadLibrary("decancer-" + rustTarget);
      } else {
        NativeUtils.loadLibraryFromJar(
          "/" + libPrefix + "decancer-" + rustTarget + "." + fileExtension
        );
      }
    } catch (Throwable err) {
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
   *
   * @param other The other string to match with.
   * @return An array of Match objects containing every similar-looking match.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native Match[] find(String other);

  /**
   * Iterates throughout this string and returns an array of every similar-looking match. Unlike find, this method also takes note of overlapping matches and merges them together.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The list of strings to match with.
   * @return An array of Match objects containing every similar-looking match.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native Match[] findMultiple(String[] other);

  /**
   * Censors every match of a string with a repetition of a character in-place.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to match with.
   * @param with The character to repeat.
   * @throws IllegalArgumentException If the character to repeat is a UTF-16 surrogate.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native void censor(String other, char with);

  /**
   * Censors every matches from an array of strings with a repetition of a character in-place.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The list of strings to match with.
   * @param with The character to repeat.
   * @throws IllegalArgumentException If the character to repeat is a UTF-16 surrogate.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native void censorMultiple(String[] other, char with);

  /**
   * Replaces every match of a string with another string in-place.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to match with.
   * @param with The other string to replace with.
   * @return Another instance of a CuredString.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native void replace(String other, String with);

  /**
   * Replaces every matches from an array of strings with another string in-place.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The list of strings to match with.
   * @param with The other string to replace with.
   * @return Another instance of a CuredString.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native void replaceMultiple(String[] other, String with);

  /**
   * Checks if this object is similar with another string.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to match with.
   * @return Whether this object is similar with another string.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native boolean equals(String other);

  /**
   * Checks if this object similarly starts with another string.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to match with.
   * @return Whether this object similarly starts with another string.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native boolean startsWith(String other);

  /**
   * Checks if this object similarly ends with another string.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to match with.
   * @return Whether this object similarly ends with another string.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native boolean endsWith(String other);

  /**
   * Checks if this object similarly contains another string.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to match with.
   * @return Whether this object similarly contains another string.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native boolean contains(String other);

  /**
   * Coerces this object to a String.
   *
   * <p>
   * NOTE: It's highly NOT recommended to use Java's comparison methods after calling this. The string output is NOT meant to be displayed visually.
   *
   * @return The String representation of this object.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native String toString();

  /**
   * Destroys and frees the memory used by this object.
   *
   * <p>
   * Repeated calls to this method is fine and does not guarantee a double-free.
   * Any subsequent String objects from toString() calls can still be used after this.
   *
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native void destroy();

  /**
   * Cures a string with decancer's default options.
   *
   * <p>
   * Output will always be in lowercase and bidirectionally reordered in order to treat right-to-left characters. Therefore, the output of this function should NOT be displayed visually.
   *
   * @param input The string to cure.
   * @throws IllegalArgumentException If the string is malformed to the point where it's not possible to apply unicode's bidirectional algorithm to it.
   * @throws RuntimeException If a Rust panic occurs.
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
   */
  public CuredString(String input, Options options) {
    this.inner = CuredString.cure(input, options.inner);
  }
}
