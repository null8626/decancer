package com.github.null8626.decancer;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class DecancerTest {

  @Test
  public void test() {
    CuredString cured = null;

    try {
      cured = new CuredString("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

      Assertions.assertTrue(cured.equals("very funny text"), "equals failed");
      Assertions.assertTrue(cured.startsWith("very"), "startsWith failed");
      Assertions.assertTrue(cured.endsWith("text"), "endsWith failed");
      Assertions.assertTrue(cured.contains("funny"), "contains failed");

      Assertions.assertEquals(
        "very funny text",
        cured.toString(),
        "toString failed"
      );

      cured.destroy();
    } catch (Throwable error) {
      if (cured != null) {
        cured.destroy();
      }

      throw error;
    }
  }
}
