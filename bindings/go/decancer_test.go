package decancer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBasicCure(t *testing.T) {
	cured, err := Cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣", Default)

	assert.Nil(t, err, "curing should not fail")

	defer cured.Close()

	assert.True(t, cured.Equals("very funny text"), "Equals should be true")
	assert.True(t, cured.StartsWith("very"), "StartsWith should be true")
	assert.True(t, cured.EndsWith("text"), "EndsWith should be true")
	assert.True(t, cured.Contains("funny"), "Contains should be true")
	assert.Equal(t, "very funny text", cured.String(), "Direct equals should be true")

	matches := cured.Find("funny")

	assert.Equal(t, 1, len(matches), "Find matches should be 1")
	assert.Equal(t, 5, matches[0].Start, "Find match start should be 5")
	assert.Equal(t, 10, matches[0].End, "Find match end should be 10")

	keywords := []string{"very", "funny"}
	matches, err = cured.FindMultiple(keywords)

	assert.Nil(t, err, "FindMultiple should not fail")
	assert.Equal(t, 2, len(matches), "FindMultiple matches should be 2")
	assert.Equal(t, 0, matches[0].Start, "FindMultiple match[0] start should be 0")
	assert.Equal(t, 4, matches[0].End, "FindMultiple match[0] end should be 4")
	assert.Equal(t, 5, matches[1].Start, "FindMultiple match[1] start should be 5")
	assert.Equal(t, 10, matches[1].End, "FindMultiple match[1] end should be 10")
}

func TestCensor(t *testing.T) {
	cured, err := Cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣", Default)

	assert.Nil(t, err, "curing should not fail")

	defer cured.Close()

	cured.Censor("funny", '*')

	assert.True(t, cured.Equals("very ***** text"), "Censor should actually censor")
}

func TestCensorMultiple(t *testing.T) {
	cured, err := Cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣", Default)

	assert.Nil(t, err, "curing should not fail")

	defer cured.Close()

	keywords := []string{"very", "funny"}
	cured.CensorMultiple(keywords, '*')

	assert.True(t, cured.Equals("**** ***** text"), "CensorMultiple should actually censor multiple")
}

func TestReplace(t *testing.T) {
	cured, err := Cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣", Default)

	assert.Nil(t, err, "curing should not fail")

	defer cured.Close()

	cured.Replace("very", "not")

	assert.True(t, cured.Equals("not funny text"), "Replace should actually replace")
}

func TestReplaceMultiple(t *testing.T) {
	cured, err := Cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣", Default)

	assert.Nil(t, err, "curing should not fail")

	defer cured.Close()

	keywords := []string{"very", "funny"}
	cured.ReplaceMultiple(keywords, "sussy")

	assert.True(t, cured.Equals("sussy sussy text"), "ReplaceMultiple should actually replace multiple")
}
