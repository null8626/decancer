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
  public void getSuggestions() {
    final CureSuggestion[] suggestions = CURED.getSuggestions();

    Assertions.assertEquals(15, suggestions.length);

    Assertions.assertEquals(0, suggestions[0].oldIndex);
    Assertions.assertEquals(0, suggestions[0].newIndex);
    Assertions.assertEquals("v", suggestions[0].translation);

    Assertions.assertEquals(1, suggestions[1].oldIndex);
    Assertions.assertEquals(1, suggestions[1].newIndex);
    Assertions.assertEquals("e", suggestions[1].translation);

    Assertions.assertEquals(4, suggestions[2].oldIndex);
    Assertions.assertEquals(2, suggestions[2].newIndex);
    Assertions.assertEquals("r", suggestions[2].translation);

    Assertions.assertEquals(7, suggestions[3].oldIndex);
    Assertions.assertEquals(3, suggestions[3].newIndex);
    Assertions.assertEquals("y", suggestions[3].translation);

    Assertions.assertEquals(11, suggestions[4].oldIndex);
    Assertions.assertEquals(4, suggestions[4].newIndex);
    Assertions.assertEquals(suggestions[4].translation, " ");

    Assertions.assertEquals(12, suggestions[5].oldIndex);
    Assertions.assertEquals(5, suggestions[5].newIndex);
    Assertions.assertEquals("f", suggestions[5].translation);

    Assertions.assertEquals(16, suggestions[6].oldIndex);
    Assertions.assertEquals(6, suggestions[6].newIndex);
    Assertions.assertEquals("u", suggestions[6].translation);

    Assertions.assertEquals(20, suggestions[7].oldIndex);
    Assertions.assertEquals(7, suggestions[7].newIndex);
    Assertions.assertEquals("n", suggestions[7].translation);

    Assertions.assertEquals(22, suggestions[8].oldIndex);
    Assertions.assertEquals(8, suggestions[8].newIndex);
    Assertions.assertEquals("n", suggestions[8].translation);

    Assertions.assertEquals(25, suggestions[9].oldIndex);
    Assertions.assertEquals(9, suggestions[9].newIndex);
    Assertions.assertEquals("y", suggestions[9].translation);

    Assertions.assertEquals(28, suggestions[10].oldIndex);
    Assertions.assertEquals(10, suggestions[10].newIndex);
    Assertions.assertEquals(suggestions[10].translation, " ");

    Assertions.assertEquals(29, suggestions[11].oldIndex);
    Assertions.assertEquals(11, suggestions[11].newIndex);
    Assertions.assertEquals("t", suggestions[11].translation);

    Assertions.assertEquals(31, suggestions[12].oldIndex);
    Assertions.assertEquals(12, suggestions[12].newIndex);
    Assertions.assertEquals("e", suggestions[12].translation);

    Assertions.assertEquals(34, suggestions[13].oldIndex);
    Assertions.assertEquals(13, suggestions[13].newIndex);
    Assertions.assertEquals("x", suggestions[13].translation);

    Assertions.assertEquals(38, suggestions[14].oldIndex);
    Assertions.assertEquals(14, suggestions[14].newIndex);
    Assertions.assertEquals("t", suggestions[14].translation);
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
