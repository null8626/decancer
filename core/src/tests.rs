// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

#[cfg(feature = "options")]
use crate::Options;
use crate::{
  bidi::{IsolatingRunSequence, Paragraph},
  Class, Level,
};
use proptest::prelude::*;
use std::ops::Range;

proptest! {
  #![proptest_config(ProptestConfig::with_cases(2000))]

  #[test]
  fn character_crash(c in any::<char>()) {
    let _ = crate::cure_char!(c);
  }

  #[test]
  fn string_crash(s in "\\PC*") {
    let _ = crate::cure!(&s);
  }
}

macro_rules! assert_matches {
  ($input:literal,$find:literal,$expected:expr) => {{
    let cured = $crate::cure!($input).unwrap();
    let matches = cured.find($find).collect::<Vec<_>>();

    assert_eq!(matches, $expected);
  }};
}

macro_rules! assert_no_matches {
  ($input:literal,$find:literal) => {{
    let cured = $crate::cure!($input).unwrap();
    let matches = cured.find($find).collect::<Vec<_>>();

    assert!(matches.is_empty());
  }};
}

#[test]
#[allow(clippy::single_range_in_vec_init)]
fn similar_equal() {
  assert_matches!("h", "h", [0..1]);
  assert_matches!("he", "he", [0..2]);
  assert_matches!("h3", "he", [0..2]);

  assert_matches!("hello", "hello", [0..5]);
  assert_matches!("hhheeeeelllloo", "hello", [0..14]);
  assert_matches!("?asdf-hhheeeeelllloo", "hello", [6..20]);

  assert_matches!("-hello", "hello", [1..6]);
  assert_matches!("hello-", "hello", [0..5]);
  assert_matches!("---hello", "hello", [3..8]);
  assert_matches!("---hello-", "hello", [3..8]);

  assert_matches!("hhheeeeelllloo!!", "hello", [0..14]);

  assert_matches!("-!?hel$2-hello?", "hello", [9..14]);
  assert_matches!("-!?hel$2-hhheeeeelllloo!!", "hello", [9..23]);

  assert_matches!("wow hell  wow heellllo", "hello", [14..22]);
  assert_matches!("wow hell  wow heellllo!", "hello", [14..22]);

  #[cfg(feature = "separators")]
  {
    assert_matches!("hh-he  e eeell/l/lo//o", "hello", [0..22]);
    assert_matches!(" shhhiii/iiiiitttttt/ttttt ", "shit", [1..26]);
    assert_matches!("hh-he  e eeell/l/lo-?", "hello", [0..19]);
    assert_matches!("shhhiii/iiiiitttttt/ttttt/", "shit", [0..25]);
    assert_matches!("hh-he  e ee,e ll/l/lo//o-?", "hello", [0..24]);
    assert_matches!("-!?hel$2-hh-he  e ee,e,ll/l/lo//o-?", "hello", [9..33]);
  }

  #[cfg(feature = "leetspeak")]
  {
    assert_matches!("|-|3|_I_0", "hello", [0..9]);
    assert_matches!("|-|3|aI_0", "helalo", [0..9]);
    assert_matches!("|-|3|_|_0", "he|_lo", [0..9]);
    assert_matches!("|--|3e33|__|_I_I_0()O[]", "hello", [0..23]);

    assert_no_matches!("|-|3|_|_0", "he+lo");
  }

  assert_no_matches!("", "");
  assert_no_matches!("h", "");
  assert_no_matches!("", "h");
  assert_no_matches!("", "he");
  assert_no_matches!("h", "he");
  assert_no_matches!("-", "hello");
  assert_no_matches!("- !?", "hello");
  assert_no_matches!("ello", "hello");
  assert_no_matches!("eel", "ell");
  assert_no_matches!("ell", "eel");
  assert_no_matches!("-!?hel", "hell");
  assert_no_matches!("ello?", "hello");
}

#[test]
fn censor() {
  let mut cured = crate::cure!("word word this is a word").unwrap();

  cured.censor("word", '*');

  assert_eq!(cured, "**** **** this is a ****");

  let mut cured2 = crate::cure!("wordword this is a word").unwrap();

  cured2.censor("word", '*');

  assert_eq!(cured2, "******** this is a ****");
}

#[test]
fn bidi_class() {
  assert_eq!(Class::new(0x0000), Some(Class::BN));
  assert_eq!(Class::new(0x0040), Some(Class::ON));
  assert_eq!(Class::new(0x0041), Some(Class::L));
  assert_eq!(Class::new(0x0062), Some(Class::L));
  assert_eq!(Class::new(0x007f), Some(Class::BN));

  assert_eq!(Class::new(0x05d0), Some(Class::R));
  assert_eq!(Class::new(0x05d1), Some(Class::R));

  assert_eq!(Class::new(0x0600), Some(Class::AN));
  assert_eq!(Class::new(0x0627), Some(Class::AL));

  assert_eq!(Class::new(0x07c0), Some(Class::R));
  assert_eq!(Class::new(0x0860), Some(Class::AL));
  assert_eq!(Class::new(0x08a0), Some(Class::AL));
  assert_eq!(Class::new(0x089f), None);
  assert_eq!(Class::new(0x08ff), None);

  assert_eq!(Class::new(0x20a0), Some(Class::ET));

  assert_eq!(Class::new(0xfb1d), Some(Class::R));
  assert_eq!(Class::new(0xfb4f), Some(Class::R));
  assert_eq!(Class::new(0xfb50), Some(Class::AL));
  assert_eq!(Class::new(0xfdf0), Some(Class::AL));
  assert_eq!(Class::new(0xfe70), Some(Class::AL));
  assert_eq!(Class::new(0xfeff), Some(Class::BN));

  assert_eq!(Class::new(0x10800), Some(Class::R));
  assert_eq!(Class::new(0x1e800), Some(Class::R));
  assert_eq!(Class::new(0x1ee00), Some(Class::AL));

  assert_eq!(Class::new(0x30000), Some(Class::L));
}

fn level_runs(levels: &[Level], original_classes: &[Class]) -> Vec<Range<usize>> {
  let mut runs = vec![];

  let mut current_run_level = levels[0];
  let mut current_run_start = 0;

  for i in 1..levels.len() {
    if !original_classes[i].removed_by_x9() && levels[i] != current_run_level {
      runs.push(current_run_start..i);
      current_run_level = levels[i];
      current_run_start = i;
    }
  }

  runs.push(current_run_start..levels.len());
  runs
}

fn irs_sorted(
  paragraph: &Paragraph,
  levels: &[Level],
  classes: &[Class],
) -> Vec<IsolatingRunSequence> {
  let level_runs = level_runs(levels, classes);
  let mut sequences = vec![];

  paragraph
    .isolating_run_sequences(levels, &level_runs, classes, &mut sequences)
    .unwrap();

  sequences.sort_by(|a, b| a.runs[0].clone().cmp(b.runs[0].clone()));

  sequences
}

#[allow(clippy::needless_pass_by_value)]
fn test_irs_runs(
  paragraph: &Paragraph,
  classes: &[Class],
  levels: &[Level],
  expected: Vec<Vec<Range<usize>>>,
) {
  let sequences = irs_sorted(paragraph, levels, classes);

  assert_eq!(
    sequences.iter().map(|s| s.runs.clone()).collect::<Vec<_>>(),
    expected,
  );
}

fn test_irs(
  paragraph: &Paragraph,
  classes: &[Class],
  levels: &[Level],
  expected: &[IsolatingRunSequence],
) {
  let sequences = irs_sorted(paragraph, levels, classes);

  assert_eq!(sequences.len(), expected.len());

  for (i, seq) in sequences.iter().enumerate() {
    assert_eq!(seq, &expected[i]);
  }
}

#[test]
fn isolating_run_sequences() {
  macro_rules! classes {
    ($($rest:tt),*) => {
      &[$(Class::$rest),*]
    }
  }

  macro_rules! levels {
    ($($rest:tt),*) => {
      &[$(Level($rest)),*]
    }
  }

  macro_rules! runs {
    ($([$($start:literal..$end:literal),*]),*) => {
      vec![$(vec![$($start..$end),*]),*]
    }
  }

  macro_rules! irs {
    ($(
      [[$($start:literal..$end:literal),*],$sos:ident,$eos:ident]
    ),*) => {
      &[$(IsolatingRunSequence {
        runs: vec![$($start..$end),*],
        start_class: Class::$sos,
        end_class: Class::$eos,
      }),*]
    }
  }

  let mock_paragraph = Paragraph {
    range: 0..1,
    level: Level::ltr(),
    pure_ltr: false,
    has_isolate_controls: true,
  };

  test_irs_runs(
    &mock_paragraph,
    classes!(L, RLE, L, PDF, RLE, L, PDF, L),
    levels!(0, 1, 1, 1, 1, 1, 1, 0),
    runs!([0..2], [2..7], [7..8]),
  );

  test_irs_runs(
    &mock_paragraph,
    classes!(L, RLI, L, PDI, RLI, L, PDI, L),
    levels!(0, 0, 1, 0, 0, 1, 0, 0),
    runs!([0..2, 3..5, 6..8], [2..3], [5..6]),
  );

  test_irs_runs(
    &mock_paragraph,
    classes!(L, RLI, L, LRI, L, RLE, L, PDF, L, PDI, L, PDI, L),
    levels!(0, 0, 1, 1, 2, 3, 3, 3, 2, 1, 1, 0, 0),
    runs!([0..2, 11..13], [2..4, 9..11], [4..6], [6..8], [8..9]),
  );

  test_irs(
    &mock_paragraph,
    classes!(L, RLE, L, LRE, L, PDF, L, PDF, RLE, L, PDF, L),
    levels!(0, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 0),
    irs! {
      [[0..2], L, R],
      [[2..4], R, L],
      [[4..6], L, L],
      [[6..11], L, R],
      [[11..12], R, L]
    },
  );

  test_irs(
    &mock_paragraph,
    classes!(L, RLI, L, LRI, L, PDI, L, PDI, RLI, L, PDI, L),
    levels!(0, 0, 1, 1, 2, 1, 1, 0, 0, 1, 0, 0),
    irs! {
      [[0..2, 7..9, 10..12], L, L],
      [[2..4, 5..7], R, R],
      [[4..5], L, L],
      [[9..10], R, R]
    },
  );
}

#[test]
#[cfg(feature = "options")]
fn retain_capitalization() {
  assert_eq!(
    crate::cure("decÁncer", Options::default().retain_capitalization()).unwrap(),
    "decAncer"
  );
}

#[cfg(feature = "options")]
fn test_reorder(input: &str, expected: &str) {
  assert_eq!(
    crate::cure(input, Options::default().retain_hebrew().retain_arabic()).unwrap(),
    expected
  );
}

#[test]
#[cfg(feature = "options")]
fn reorder() {
  test_reorder("abc\ndef\nghi", "abc\ndef\nghi");
  test_reorder("ab1\nde2\ngh3", "ab1\nde2\ngh3");

  test_reorder(concat!("א", "ב", "ג", "abc"), concat!("abc", "ג", "ב", "א"));

  test_reorder("abc\nابج", concat!("abc\n", "جبا"));
  test_reorder(
    "\u{0627}\u{0628}\u{062C}\nabc",
    "\n\u{062C}\u{0628}\u{0627}abc",
  );

  test_reorder("1.-2", "1.-2");
  test_reorder("1-.2", "1-.2");

  test_reorder("abc אבג", "abc גבא");

  test_reorder("123 \u{05D0}\u{05D1}\u{05D2}", "גבא 123");

  test_reorder("abc\u{202A}def", "abc\u{202A}def");
  test_reorder("abc\u{202A}def\u{202C}ghi", "abc\u{202A}def\u{202C}ghi");
  test_reorder("abc\u{2066}def\u{2069}ghi", "abc\u{2066}def\u{2069}ghi");

  test_reorder("\u{202B}abc אבג\u{202C}", "\u{202b}גבא abc\u{202c}");
  test_reorder("\u{05D0}בג? אבג", "גבא ?גבא");

  test_reorder("A אבג?", "A גבא?");
  test_reorder("A אבג?\u{200F}", "A \u{200F}?גבא");

  test_reorder("\u{05D0}בג abc", "abc גבא");
  test_reorder("abc\u{2067}.-\u{2069}ghi", "abc\u{2067}-.\u{2069}ghi");
  test_reorder(
    "Hello, \u{2068}\u{202E}world\u{202C}\u{2069}!",
    "Hello, \u{2068}\u{202E}\u{202C}dlrow\u{2069}!",
  );
  test_reorder("\u{05D0}(ב)ג.", ".ג)ב(א");
  test_reorder("\u{05D0}ב(גד[&ef].)gh", "gh).]ef&[דג(בא");
}
