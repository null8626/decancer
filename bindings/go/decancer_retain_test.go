// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

// WARNING: This file is computer generated.

package decancer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func DoRetainTest(t *testing.T, options Option, input string) {
	defaultCured, err := Cure(input, Default)

	assert.Nil(t, err, "curing should not fail")

	defer defaultCured.Close()

	assert.True(t, defaultCured.Equals(input), "Default should make decancer cure the designated characters")

	retainCured, err := Cure(input, options)

	assert.Nil(t, err, "curing should not fail")

	defer retainCured.Close()

	assert.False(t, retainCured.Equals(input), "Retain should prevent decancer from curing the designated characters")
}

func TestRetains(t *testing.T) {
	DoRetainTest(t, RetainTurkish, "çğıöşü")
	DoRetainTest(t, RetainGreek, "ͱͳʹ͵ͷϝϟϣϥϧῦ`ῲῶ´")
	DoRetainTest(t, RetainCyrillic, "абвгдӆӈӊӌӎꚕꚗꚙꚛꚜ")
	DoRetainTest(t, RetainArmenian, "ՙ՟ՠաբճմյնշֆև։֊֍")
	DoRetainTest(t, RetainHebrew, "־׀׃׆אכלםמןװױײ׳״")
	DoRetainTest(t, RetainArabic, "؅؈؉،؎ݧݪݬݮݱ𞸫𞹋𞺀𞺋𞺫")
	DoRetainTest(t, RetainDevanagari, "ःऄअआइऽािीॉ꣺ꣻ꣼ꣽꣾ")
	DoRetainTest(t, RetainBengali, "ঀংঃঌএযরষসঽ৷৸৹৻৽")
	DoRetainTest(t, RetainGujarati, "ઃઅઆઇઊદનપફબ૮૯૰૱ૹ")
	DoRetainTest(t, RetainTamil, "அஈஉஊஎயரறலள௭௰௱௴௶")
	DoRetainTest(t, RetainThai, "กขคฆชวษหฬฯ๖๗๘๚๛")
	DoRetainTest(t, RetainLao, "ກຂງຊຍວສຫອຮ໗໘໙ໜໞ")
	DoRetainTest(t, RetainBurmese, "ကခဂဃငၶၸၹၺၻꩰꩲ꩷꩹ꩽ")
	DoRetainTest(t, RetainKorean, "ᄀᄁᄂᄃᄄᇧᇨᇩᇫᇬퟵퟶퟹퟺퟻ")
	DoRetainTest(t, RetainKhmer, "កខគឃចអឤឥឧឫ៴៶៷៸៹")
	DoRetainTest(t, RetainMongolian, "᠁᠂᠃᠄᠆᠗᠘ᠪᠫᠯᢃᢄᢒᢗᢦ")
	DoRetainTest(t, RetainBraille, "⠀⠁⠃⠄⠅⡃⡄⡅⡇⡈⣤⣫⣸⣹⣻")
	DoRetainTest(t, RetainChinese, "⺀⺁⺃⺄⺅㟄㟍㟐㟪㠩﹀﹁﹅﹆﹉")
	DoRetainTest(t, RetainJapanese, "ぃいくけこチテトナニㇻㇼㇽㇾㇿ")
}
