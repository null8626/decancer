package com.github.null8626.decancer;

import com.fizzed.jne.NativeTarget;
import com.fizzed.jne.OperatingSystem;
import cz.adamh.utils.NativeUtils;

public class CuredString {
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
        case OperatingSystem.WINDOWS:
          {
            libPrefix = "";
            fileExtension = "dll";
            break;
          }
        case OperatingSystem.MACOS:
          {
            fileExtension = "dylib";
          }
      }

      NativeUtils.loadLibraryFromJar(
        "/" + libPrefix + "decancer-" + rustTarget + "." + fileExtension
      );
    } catch (Throwable err) {
      throw new RuntimeException(
        "[" +
        rustTarget +
        "] this operating system (" +
        osName +
        ") and/or architecture (" +
        archName +
        ") is not supported."
      );
    }
  }

  private long inner;

  private static native long cure(String input);

  public native boolean equals(String other);

  public native boolean startsWith(String other);

  public native boolean endsWith(String other);

  public native boolean contains(String other);

  public native String toString();

  public native void destroy();

  public CuredString(String input) {
    this.inner = CuredString.cure(input);
  }
}