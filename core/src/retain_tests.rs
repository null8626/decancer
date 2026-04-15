// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

// WARNING: This file is computer generated.

fn do_retain_test(options: Options, test_string: &str) {
  assert_ne!(super::cure!(test_string).unwrap(), test_string);
  assert_eq!(super::cure(test_string, options.disable_bidi()).unwrap(), test_string);
}

#[test]
#[cfg(feature = "options")]
#[allow(clippy::unicode_not_nfc)]
fn retains() {
  do_retain_test(Options::default().retain_turkish(), "çğıöşü");
  do_retain_test(Options::default().retain_greek(), "ͱͳʹ͵ͷϝϟϣϥϧῦ`ῲῶ´");
  do_retain_test(Options::default().retain_cyrillic(), "абвгдӆӈӊӌӎꚕꚗꚙꚛꚜ");
  do_retain_test(Options::default().retain_armenian(), "ՙ՟ՠաբճմյնշֆև։֊֍");
  do_retain_test(Options::default().retain_hebrew(), "־׀׃׆אכלםמןװױײ׳״");
  do_retain_test(Options::default().retain_arabic(), "؅؈؉،؎ݧݪݬݮݱ𞸫𞹋𞺀𞺋𞺫");
  do_retain_test(Options::default().retain_devanagari(), "ःऄअआइऽािीॉ꣺ꣻ꣼ꣽꣾ");
  do_retain_test(Options::default().retain_bengali(), "ঀংঃঌএযরষসঽ৷৸৹৻৽");
  do_retain_test(Options::default().retain_gujarati(), "ઃઅઆઇઊદનપફબ૮૯૰૱ૹ");
  do_retain_test(Options::default().retain_tamil(), "அஈஉஊஎயரறலள௭௰௱௴௶");
  do_retain_test(Options::default().retain_thai(), "กขคฆชวษหฬฯ๖๗๘๚๛");
  do_retain_test(Options::default().retain_lao(), "ກຂງຊຍວສຫອຮ໗໘໙ໜໞ");
  do_retain_test(Options::default().retain_burmese(), "ကခဂဃငၶၸၹၺၻꩰꩲ꩷꩹ꩽ");
  do_retain_test(Options::default().retain_korean(), "ᄀᄁᄂᄃᄄᇧᇨᇩᇫᇬퟵퟶퟹퟺퟻ");
  do_retain_test(Options::default().retain_khmer(), "កខគឃចអឤឥឧឫ៴៶៷៸៹");
  do_retain_test(Options::default().retain_mongolian(), "᠁᠂᠃᠄᠆᠗᠘ᠪᠫᠯᢃᢄᢒᢗᢦ");
  do_retain_test(Options::default().retain_braille(), "⠀⠁⠃⠄⠅⡃⡄⡅⡇⡈⣤⣫⣸⣹⣻");
  do_retain_test(Options::default().retain_chinese(), "⺀⺁⺃⺄⺅㟄㟍㟐㟪㠩﹀﹁﹅﹆﹉");
  do_retain_test(Options::default().retain_japanese(), "ぃいくけこチテトナニㇻㇼㇽㇾㇿ");
}