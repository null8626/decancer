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

  private static boolean isJUnit() {
    for (StackTraceElement element : Thread.currentThread().getStackTrace()) {
      if (element.getClassName().startsWith("org.junit.")) {
        return true;
      }
    }

    return false;
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

  private long inner;

  private static native long cure(String input, int options);

  /**
   * Checks if this object is similar with another string.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to compare with.
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
   * @param other The other string to compare with.
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
   * @param other The other string to compare with.
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
   * @param other The other string to compare with.
   * @return Whether this object similarly contains another string.
   * @throws NullPointerException If destroy() has been called prior to this.
   * @throws RuntimeException If a Rust panic occurs.
   */
  public native boolean contains(String other);

  /**
   * Coerces this object into a String.
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
   * Any subsequent String objects from toString() calls can still be used after this.
   *
   * @throws NullPointerException If destroy() has been called prior to this.
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
