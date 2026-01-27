// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

// WARNING: This file is computer generated.

#[test]
#[cfg(feature = "options")]
#[allow(clippy::unicode_not_nfc)]
fn retains() {
  let test_retain = |options: Options, test_string| {
    assert_ne!(super::cure!(test_string).unwrap(), test_string);
    assert_eq!(super::cure(test_string, options.disable_bidi()).unwrap(), test_string);
  };

  test_retain(Options::default().retain_turkish(), "çğıöşü");
  test_retain(Options::default().retain_greek(), "ͱͳʹ͵ͷϝϟϣϥϧῦ`ῲῶ´");
  test_retain(Options::default().retain_cyrillic(), "абвгдӆӈӊӌӎꚕꚗꚙꚛꚜ");
  test_retain(Options::default().retain_armenian(), "ՙ՟ՠաբճմյնշֆև։֊֍");
  test_retain(Options::default().retain_hebrew(), "־׀׃׆אכלםמןװױײ׳״");
  test_retain(Options::default().retain_arabic(), "؅؈؉،؎ݧݪݬݮݱ𞸫𞹋𞺀𞺋𞺫");
  test_retain(Options::default().retain_devanagari(), "ःऄअआइऽािीॉ꣺ꣻ꣼ꣽꣾ");
  test_retain(Options::default().retain_bengali(), "ঀংঃঌএযরষসঽ৷৸৹৻৽");
  test_retain(Options::default().retain_gujarati(), "ઃઅઆઇઊદનપફબ૮૯૰૱ૹ");
  test_retain(Options::default().retain_tamil(), "அஈஉஊஎயரறலள௭௰௱௴௶");
  test_retain(Options::default().retain_thai(), "กขคฆชวษหฬฯ๖๗๘๚๛");
  test_retain(Options::default().retain_lao(), "ກຂງຊຍວສຫອຮ໗໘໙ໜໞ");
  test_retain(Options::default().retain_burmese(), "ကခဂဃငၶၸၹၺၻꩰꩲ꩷꩹ꩽ");
  test_retain(Options::default().retain_korean(), "ᄀᄁᄂᄃᄄᇧᇨᇩᇫᇬퟵퟶퟹퟺퟻ");
  test_retain(Options::default().retain_khmer(), "កខគឃចអឤឥឧឫ៴៶៷៸៹");
  test_retain(Options::default().retain_mongolian(), "᠁᠂᠃᠄᠆᠗᠘ᠪᠫᠯᢃᢄᢒᢗᢦ");
  test_retain(Options::default().retain_braille(), "⠀⠁⠃⠄⠅⡃⡄⡅⡇⡈⣤⣫⣸⣹⣻");
  test_retain(Options::default().retain_chinese(), "⺀⺁⺃⺄⺅㟄㟍㟐㟪㠩﹀﹁﹅﹆﹉");
  test_retain(Options::default().retain_japanese(), "ぃいくけこチテトナニㇻㇼㇽㇾㇿ");
}