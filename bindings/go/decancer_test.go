// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

package decancer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCureChar(t *testing.T) {
	cured := CureChar('\uFF25', Default)

	assert.Equal(t, "e", cured, "CureChar should return \"e\"")

	cured = CureChar('\u04D5', Default)

	assert.Equal(t, "ae", cured, "CureChar should return \"ae\"")

	cured = CureChar('\u0000', Default)

	assert.Equal(t, "", cured, "CureChar should return an empty string")
}

func newCuredStringSample(t *testing.T) *CuredString {
	cured, err := Cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣", Default)

	assert.Nil(t, err, "curing should not fail")

	return cured
}

func TestCure(t *testing.T) {
	cured := newCuredStringSample(t)

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
	matches, err := cured.FindMultiple(keywords)

	assert.Nil(t, err, "FindMultiple should not fail")
	assert.Equal(t, 2, len(matches), "FindMultiple matches should be 2")
	assert.Equal(t, 0, matches[0].Start, "FindMultiple match[0] start should be 0")
	assert.Equal(t, 4, matches[0].End, "FindMultiple match[0] end should be 4")
	assert.Equal(t, 5, matches[1].Start, "FindMultiple match[1] start should be 5")
	assert.Equal(t, 10, matches[1].End, "FindMultiple match[1] end should be 10")
}

func TestCensor(t *testing.T) {
	cured := newCuredStringSample(t)

	defer cured.Close()

	assert.Nil(t, cured.Censor("funny", '*'), "Censor should not fail")
	assert.True(t, cured.Equals("very ***** text"), "Censor should actually censor")
}

func TestCensorMultiple(t *testing.T) {
	cured := newCuredStringSample(t)

	defer cured.Close()

	assert.Nil(t, cured.CensorMultiple([]string{"very", "funny"}, '*'), "CensorMultiple should not fail")
	assert.True(t, cured.Equals("**** ***** text"), "CensorMultiple should actually censor multiple")
}

func TestReplace(t *testing.T) {
	cured := newCuredStringSample(t)

	defer cured.Close()

	assert.Nil(t, cured.Replace("very", "not"), "Replace should not fail")
	assert.True(t, cured.Equals("not funny text"), "Replace should actually replace")
}

func TestReplaceMultiple(t *testing.T) {
	cured := newCuredStringSample(t)

	defer cured.Close()

	assert.Nil(t, cured.ReplaceMultiple([]string{"very", "funny"}, "sussy"), "ReplaceMultiple should not fail")
	assert.True(t, cured.Equals("sussy sussy text"), "ReplaceMultiple should actually replace multiple")
}
