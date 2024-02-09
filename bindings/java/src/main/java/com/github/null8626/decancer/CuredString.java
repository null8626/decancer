package com.github.null8626.decancer;

import cz.adamh.utils.NativeUtils;
import java.io.*;
import java.nio.file.*;

/**
 * A small wrapper around the String data type for comparison purposes.
 *
 * <p>
 * This is used because imperfections from translations can happen,
 * thus this is used to provide comparison functions that are not as strict and can detect similar-looking characters (e.g: `i` and `l`)
 *
 * @author null8626
 * @version v2.0.1
 * @since 2024-02-09
 */
public class CuredString {

  private long inner;

  private static boolean nestedContains(String value, String... array) {
    for (String alias : array) {
      if (alias.contains(value)) {
        return true;
      }
    }

    return false;
  }

  private static void loadLibrary(String fileName, String extension)
    throws IOException {
    for (StackTraceElement element : Thread.currentThread().getStackTrace()) {
      if (element.getClassName().startsWith("org.junit.")) {
        System.loadLibrary(fileName);
        return;
      }
    }

    NativeUtils.loadLibraryFromJar("/" + fileName + extension);
  }

  static {
    final String osName = System.getProperty("os.name").toLowerCase();
    final String osArch = System.getProperty("os.arch").toLowerCase();

    try {
      final String abiType = System.getProperty("sun.arch.abi");
      final String bootLibPath = System
        .getProperty("sun.boot.library.path", "")
        .toLowerCase();

      String target = null;
      String libPrefix = "lib";
      String extension = null;

      if (osName.contains("windows")) {
        target = "win";
        libPrefix = "";
        extension = ".dll";
      } else if (CuredString.nestedContains(osArch, "mac", "darwin")) {
        target = "mac";
        extension = ".dylib";
      } else if (osName.contains("linux")) {
        extension = ".so";

        if (CuredString.nestedContains(osArch, "default", "gnu")) {
          target = "gnu";
        } else if (osArch.contains("musl")) {
          target = "musl";
        } else if (
          CuredString.nestedContains(osArch, "armhf", "arm32v7", "armv7l") ||
          (CuredString.nestedContains(osArch, "arm", "aarch32") &&
            ("gnueabihf".equals(abiType) ||
              CuredString.nestedContains(bootLibPath, "armhf", "aarch32hf")))
        ) {
          target = "armhf";
        } else {
          final Path libDir = Paths.get("/lib/");

          try {
            if (Files.exists(libDir)) {
              final File[] mapFiles = libDir.toFile().listFiles();

              if (mapFiles != null) {
                for (File mapFile : mapFiles) {
                  final String name = mapFile.getName().toLowerCase();

                  if (name.contains("musl")) {
                    target = "musl";
                    break;
                  } else if (
                    CuredString.nestedContains(
                      name,
                      "armhf",
                      "arm-linux-gnueabihf"
                    )
                  ) {
                    target = "armhf";
                    break;
                  }
                }
              }
            }
          } catch (Throwable ignored) {}
        }
      }

      if (target != null) {
        if (CuredString.nestedContains(osArch, "arm64", "aarch64")) {
          target = "arm" + target;
        } else if (
          CuredString.nestedContains(osArch, "x64", "x86_64", "amd64")
        ) {
          target = "64" + target;
        } else if (
          CuredString.nestedContains(
            osArch,
            "x32",
            "x86",
            "i386",
            "i586",
            "i686"
          )
        ) {
          target = "32" + target;
        } else if (!"armhf".equals(target)) {
          target = null;
        }
      }

      CuredString.loadLibrary(libPrefix + target, extension);
    } catch (Throwable err) {
      throw new RuntimeException(
        String.format(
          "%s %s is not supported.\nOriginal error message: %s",
          osArch,
          osName,
          err.getMessage()
        )
      );
    }
  }

  private static native long cure(String other) throws IllegalArgumentException;

  /**
   * Checks if this object is similar to another string.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to compare.
   * @throws NullPointerException If destroy() is already called prior to this.
   * @return Whether this object similarly contains another string.
   * @version v2.0.1
   * @since 2024-02-09
   */
  public native boolean equals(String other);

  /**
   * Checks if this object similarly starts with another string.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to compare.
   * @throws NullPointerException If destroy() is already called prior to this.
   * @return Whether this object similarly contains another string.
   * @version v2.0.1
   * @since 2024-02-09
   */
  public native boolean startsWith(String other);

  /**
   * Checks if this object similarly ends with another string.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to compare.
   * @throws NullPointerException If destroy() is already called prior to this.
   * @return Whether this object similarly contains another string.
   * @version v2.0.1
   * @since 2024-02-09
   */
  public native boolean endsWith(String other);

  /**
   * Checks if this object similarly contains another string.
   *
   * <p>
   * This comparison is case-insensitive.
   *
   * @param other The other string to compare.
   * @throws NullPointerException If destroy() is already called prior to this.
   * @return Whether this object similarly contains another string.
   * @version v2.0.1
   * @since 2024-02-09
   */
  public native boolean contains(String other);

  /**
   * Creates a new Java String from this object.
   *
   * <p>
   * It's highly NOT recommended to use Java's comparison methods after calling this.
   * The string output is NOT meant to be displayed visually.
   *
   * @throws NullPointerException If destroy() is already called prior to this.
   * @return The Java String.
   * @version v2.0.1
   * @since 2024-02-09
   */
  public native String toString();

  /**
   * Frees the allocated string inside this object.
   *
   * <p>
   * Strings created by the toString method can still be used.
   * @throws NullPointerException If destroy() is already called prior to this.
   * @version v2.0.1
   * @since 2024-02-09
   */
  public native void destroy();

  /**
   * Cures the specified string.
   *
   * <p>
   * Output will always be in lowercase and [bidirectionally reordered](https://en.wikipedia.org/wiki/Bidirectional_text) in order to treat right-to-left characters.
   * Therefore, the output of this function *should not* be displayed visually.
   *
   * @param input The string to cure.
   * @throws IllegalArgumentException If the string is malformed to the point where it's not possible to apply unicode's bidirectional algorithm to it.
   * @version v2.0.1
   * @since 2024-02-09
   */
  public CuredString(String input) throws IllegalArgumentException {
    this.inner = CuredString.cure(input);
  }
}
