import com.github.null8626.decancer.CuredString;
import com.github.null8626.decancer.Match;
import org.junit.jupiter.api.*;

public class DecancerTest {

  private CuredString cured;

  @BeforeEach
  public void cure() {
    this.cured = new CuredString("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  }
  
  @Test
  public void find() {
    final Match[] match = this.cured.find("funny");
    
    Assertions.assertEquals(match.length, 1);
    Assertions.assertEquals(match[0].start, 5);
    Assertions.assertEquals(match[0].end, 10);
    Assertions.assertEquals(match[0].toString(), "funny");
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
    this.cured.destroy();
  }
}
