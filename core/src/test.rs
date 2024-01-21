use crate::{
  bidi::{IsolatingRunSequence, Paragraph},
  Class, Level,
};
use core::{mem::MaybeUninit, ops::Range};
use proptest::prelude::*;

proptest! {
  #![proptest_config(ProptestConfig::with_cases(2000))]

  #[test]
  fn character_crash(c in any::<char>()) {
    let _ = crate::cure_char(c);
  }

  #[test]
  #[cfg(feature = "std")]
  fn string_crash(s in "\\PC*") {
    let _ = crate::cure(&s);
  }
}

#[test]
fn bidi_class() {
  assert_eq!(Class::new(0x0000), Some(Class::BN));
  assert_eq!(Class::new(0x0040), Some(Class::ON));
  assert_eq!(Class::new(0x0041), Some(Class::L));
  assert_eq!(Class::new(0x0062), Some(Class::L));
  assert_eq!(Class::new(0x007F), Some(Class::BN));

  assert_eq!(Class::new(0x05D0), Some(Class::R));
  assert_eq!(Class::new(0x05D1), Some(Class::R));

  assert_eq!(Class::new(0x0600), Some(Class::AN));
  assert_eq!(Class::new(0x0627), Some(Class::AL));

  assert_eq!(Class::new(0x07C0), Some(Class::R));
  assert_eq!(Class::new(0x0860), Some(Class::AL));
  assert_eq!(Class::new(0x08A0), Some(Class::AL));
  assert_eq!(Class::new(0x089F), None);
  assert_eq!(Class::new(0x08FF), None);

  assert_eq!(Class::new(0x20A0), Some(Class::ET));

  assert_eq!(Class::new(0xFB1D), Some(Class::R));
  assert_eq!(Class::new(0xFB4F), Some(Class::R));
  assert_eq!(Class::new(0xFB50), Some(Class::AL));
  assert_eq!(Class::new(0xFDF0), Some(Class::AL));
  assert_eq!(Class::new(0xFE70), Some(Class::AL));
  assert_eq!(Class::new(0xFEFF), Some(Class::BN));

  assert_eq!(Class::new(0x10800), Some(Class::R));
  assert_eq!(Class::new(0x1E800), Some(Class::R));
  assert_eq!(Class::new(0x1EE00), Some(Class::AL));

  assert_eq!(Class::new(0x30000), Some(Class::L));
}

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
    [[$($start:literal..$end:literal),*],$sos:ident,$eos:ident],
  )*) => {
    &[$(IsolatingRunSequence {
      runs: vec![$($start..$end),*],
      start_class: Class::$sos,
      end_class: Class::$eos,
    },)*]
  }
}

fn irs_sorted(
  paragraph: &Paragraph,
  levels: &[Level],
  classes: &[Class],
) -> Vec<IsolatingRunSequence> {
  let mut sequences = paragraph
    .isolating_run_sequences(levels, classes)
    .collect::<Vec<_>>();

  sequences.sort_by(|a, b| a.runs[0].clone().cmp(b.runs[0].clone()));

  sequences
}

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
#[allow(invalid_value)]
fn isolating_run_sequences() {
  // SAFETY: only the level property is read in the isolating_run_sequences method.
  let mock_paragraph = unsafe {
    Paragraph {
      range: MaybeUninit::uninit().assume_init(),
      level: Level::ltr(),
      pure_ltr: false,
    }
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
      [[11..12], R, L],
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
      [[9..10], R, R],
    },
  );
}

fn test_reorder(input: &str, expected: &str) {
  assert_eq!(
    crate::reorder(input, |c, output| output.push(c)).unwrap(),
    expected
  );
}

#[test]
fn reorder() {
  test_reorder("abc\ndef\nghi", "abc\ndef\nghi");
  test_reorder("ab1\nde2\ngh3", "ab1\nde2\ngh3");

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
  // test_reorder("A אבג?\u{200F}", "A \u{200F}?גבא");

  test_reorder("\u{05D0}בג abc", "abc גבא");
  test_reorder("abc\u{2067}.-\u{2069}ghi", "abc\u{2067}-.\u{2069}ghi");
  // test_reorder("Hello, \u{2068}\u{202E}world\u{202C}\u{2069}!", "Hello, \u{2068}\u{202E}\u{202C}dlrow\u{2069}!");
  test_reorder("\u{05D0}(ב)ג.", ".ג)ב(א");
  test_reorder("\u{05D0}ב(גד[&ef].)gh", "gh).]ef&[דג(בא");
}
