package io.github.null8626.decancer;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

public class DecancerTest {

  private CuredString cured;

  @BeforeEach
  public void cure() {
    this.cured = new CuredString("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  }

  @Test
  public void censor() throws Exception {
    try (
      final CuredString string = new CuredString("wow heellllo wow hello wow!")
    ) {
      string.censor("hello", '*');

      Assertions.assertEquals("wow ******** wow ***** wow!", string.toString());
    }
  }

  @Test
  public void censorMultiple() throws Exception {
    try (
      final CuredString string = new CuredString("helloh yeah")
    ) {
      final String[] keywords = { "hello", "oh yeah" };
      string.censorMultiple(keywords, '*');

      Assertions.assertEquals("***********", string.toString());
    }
  }

  @Test
  public void replace() throws Exception {
    try (
      final CuredString string = new CuredString("wow hello wow heellllo!")
    ) {
      string.replace("hello", "world");

      Assertions.assertEquals("wow world wow world!", string.toString());
    }
  }

  @Test
  public void replaceMultiple() throws Exception {
    try (final CuredString string = new CuredString("helloh yeah")) {
      final String[] keywords = { "hello", "oh yeah" };
      string.replaceMultiple(keywords, "world");

      Assertions.assertEquals("world", string.toString());
    }
  }

  @Test
  public void find() {
    final Match[] match = this.cured.find("funny");

    Assertions.assertEquals(1, match.length, 1);
    Assertions.assertEquals(5, match[0].start, 5);
    Assertions.assertEquals(10, match[0].end, 10);
    Assertions.assertEquals("funny", match[0].toString());
  }

  @Test
  public void equals() {
    Assertions.assertTrue(this.cured.equals("very funny text"));
  }

  @Test
  public void startsWith() {
    Assertions.assertTrue(this.cured.startsWith("very"));
  }

  @Test
  public void endsWith() {
    Assertions.assertTrue(this.cured.endsWith("text"));
  }

  @Test
  public void contains() {
    Assertions.assertTrue(this.cured.contains("funny"));
  }

  @Test
  @DisplayName("toString()")
  public void toStringTest() {
    Assertions.assertEquals("very funny text", this.cured.toString());
  }

  @AfterEach
  public void cleanup() {
    this.cured.close();
  }
}
