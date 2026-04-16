// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

package io.github.null8626.decancer;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.Reader;
import java.lang.reflect.InvocationTargetException;
import java.util.Map;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

public class DecancerTests {
  private static CuredString CURED = null;
  private static JsonObject RETAIN_DATA = null;

  @BeforeAll
  public static void setup() throws IOException {
    CURED = new CuredString("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

    final Gson gson = new Gson();

    try (final InputStream inputStream =
        DecancerTests.class.getResourceAsStream("/retain_data.json")) {
      if (inputStream == null) {
        throw new IOException("Unable to read retain_data.json.");
      }

      try (final Reader reader = new InputStreamReader(inputStream)) {
        RETAIN_DATA = gson.fromJson(reader, JsonObject.class);
      }
    }
  }

  @Test
  public void censor() throws Exception {
    try (final CuredString string = new CuredString("wow heellllo wow hello wow!")) {
      string.censor("hello", '*');

      Assertions.assertEquals("wow ******** wow ***** wow!", string.toString());
    }
  }

  @Test
  public void censorMultiple() throws Exception {
    try (final CuredString string = new CuredString("helloh yeah")) {
      final String[] keywords = {"hello", "oh yeah"};
      string.censorMultiple(keywords, '*');

      Assertions.assertEquals("***********", string.toString());
    }
  }

  @Test
  public void replace() throws Exception {
    try (final CuredString string = new CuredString("wow hello wow heellllo!")) {
      string.replace("hello", "world");

      Assertions.assertEquals("wow world wow world!", string.toString());
    }
  }

  @Test
  public void replaceMultiple() throws Exception {
    try (final CuredString string = new CuredString("helloh yeah")) {
      final String[] keywords = {"hello", "oh yeah"};
      string.replaceMultiple(keywords, "world");

      Assertions.assertEquals("world", string.toString());
    }
  }

  @Test
  public void find() {
    final Match[] match = CURED.find("funny");

    Assertions.assertEquals(1, match.length, 1);
    Assertions.assertEquals(5, match[0].start, 5);
    Assertions.assertEquals(10, match[0].end, 10);
    Assertions.assertEquals("funny", match[0].toString());
  }

  @Test
  public void equals() {
    Assertions.assertTrue(CURED.equals("very funny text"));
  }

  @Test
  public void startsWith() {
    Assertions.assertTrue(CURED.startsWith("very"));
  }

  @Test
  public void endsWith() {
    Assertions.assertTrue(CURED.endsWith("text"));
  }

  @Test
  public void contains() {
    Assertions.assertTrue(CURED.contains("funny"));
  }

  @Test
  @DisplayName("toString()")
  public void toStringTest() {
    Assertions.assertEquals("very funny text", CURED.toString());
  }

  @Test
  public void retain()
      throws IllegalAccessException, InvocationTargetException, NoSuchMethodException {
    for (final Map.Entry<String, JsonElement> entry : RETAIN_DATA.entrySet()) {
      final String testString = entry.getValue().getAsString();

      Options options = new Options().disableBidi();

      options = (Options) Options.class.getMethod(entry.getKey()).invoke(options);

      try (final CuredString cured = new CuredString(testString, options)) {
        Assertions.assertTrue(cured.equals(testString));
      }

      try (final CuredString cured = new CuredString(testString)) {
        Assertions.assertFalse(cured.equals(testString));
      }
    }
  }

  @Test
  public void retainCapitalization() {
    try (final CuredString cured =
        new CuredString("decÁncer", new Options().retainCapitalization())) {
      Assertions.assertTrue(cured.toString().equals("decAncer"));
    }
  }

  @Test
  public void disableLeetspeak() {
    try (final CuredString cured = new CuredString("|-|3|_I_0", new Options().disableLeetspeak())) {
      Assertions.assertFalse(cured.equals("hello"));

      cured.disableLeetspeak(false);
      cured.disableAlphabeticalLeetspeak(true);

      Assertions.assertTrue(cured.equals("helI_o"));
    }

    try (final CuredString cured =
        new CuredString("|-|3|_I_0", new Options().disableAlphabeticalLeetspeak())) {
      Assertions.assertTrue(cured.equals("helI_o"));

      cured.disableLeetspeak(true);
      cured.disableAlphabeticalLeetspeak(false);

      Assertions.assertFalse(cured.equals("hello"));
    }
  }

  @AfterAll
  public static void cleanup() {
    if (CURED != null) {
      CURED.close();
    }
  }
}
