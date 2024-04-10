use crate::{
  bidi::{IsolatingRunSequence, Paragraph},
  class::{self, Class},
  CuredString, Level, Matcher, Options,
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

macro_rules! make_cmp_fn {
  ($($fn_name:ident($s_ident:ident, $o_ident:ident) -> $cmp_expr:expr,)*) => {
    $(
      fn $fn_name($s_ident: &str, $o_ident: &str) -> bool {
        let $s_ident = CuredString(String::from($s_ident));

        $cmp_expr
      }
    )*
  };
}

make_cmp_fn! {
  is_contains(this, other) -> this.contains(other),
  is_starts_with(this, other) -> this.starts_with(other),
  is_ends_with(this, other) -> this.ends_with(other),
  is_equal(this, other) -> this == other,
}

#[test]
fn similar_equal() {
  assert!(is_equal("hello", "hello"));
  assert!(is_equal("hhheeeeelllloo", "hello"));
  assert!(is_equal("hh-he  e ee!e!ll/l/lo//o", "hello"));
  assert!(is_equal("shhhiii/iiiiitttttt/ttttt", "shit"));

  assert!(!is_equal("-", "hello"));
  assert!(!is_equal("- !?", "hello"));
  assert!(!is_equal("hello-", "hello"));
  assert!(!is_equal("---hello", "hello"));
  assert!(!is_equal("shhhiii/iiiiitttttt/ttttt/", "shit"));
}

#[test]
fn similar_beginning() {
  assert!(is_starts_with("hello", "hello"));
  assert!(is_starts_with("hello?", "hello"));
  assert!(is_starts_with("hhheeeeelllloo!!", "hello"));
  assert!(is_starts_with("hh-he  e ee!e!ll/l/lo//o-?", "hello"));

  assert!(!is_starts_with("---hello-", "hello"));
}

#[test]
fn similar_ending() {
  assert!(is_ends_with("hello", "hello"));
  assert!(is_ends_with("?hello", "hello"));
  assert!(is_ends_with("?asdf-hhheeeeelllloo", "hello"));
}

#[test]
fn similar_contains() {
  assert!(is_contains("hello", "hello"));
  assert!(is_contains("hello?", "hello"));
  assert!(is_contains("hhheeeeelllloo!!", "hello"));
  assert!(is_contains("hh-he  e ee!e!ll/l/lo//o-?", "hello"));

  assert!(is_contains("-!?hel$2-hello?", "hello"));
  assert!(is_contains("-!?hel$2-hhheeeeelllloo!!", "hello"));
  assert!(is_contains("-!?hel$2-hh-he  e ee!e!ll/l/lo//o-?", "hello"));

  assert!(!is_contains("eel", "ell"));
  assert!(!is_contains("ell", "eel"));
  assert!(!is_contains("-!?hel", "hell"));
  assert!(!is_contains("ello?", "hello"));
}

#[test]
fn similar_find() {
  macro_rules! test_find {
    ($(($self_str:expr, $other_str:expr) {$($expected_range:expr,)*})*) => {$({
      let mut mat = Matcher::new($self_str, $other_str);

      $(assert_eq!(mat.next(), Some($expected_range));)*
      assert_eq!(mat.next(), None);
    })*};
  }

  test_find! {
    ("wow hell  wow heellllo", "hello") {
      14..22,
    }

    ("wow hell  wow heellllo!", "hello") {
      14..22,
    }
  }
}

#[test]
#[cfg(feature = "leetspeak")]
fn similar_leetspeak() {
  assert!(is_equal("|-|3|_I_0", "hello"));
  assert!(is_equal("|--|3e33|__|_I_I_0()O[]", "hello"));
}

#[test]
fn bidi_class() {
  assert_eq!(Class::new(0x0000), Some(class::BN));
  assert_eq!(Class::new(0x0040), Some(class::ON));
  assert_eq!(Class::new(0x0041), Some(class::L));
  assert_eq!(Class::new(0x0062), Some(class::L));
  assert_eq!(Class::new(0x007f), Some(class::BN));

  assert_eq!(Class::new(0x05d0), Some(class::R));
  assert_eq!(Class::new(0x05d1), Some(class::R));

  assert_eq!(Class::new(0x0600), Some(class::AN));
  assert_eq!(Class::new(0x0627), Some(class::AL));

  assert_eq!(Class::new(0x07c0), Some(class::R));
  assert_eq!(Class::new(0x0860), Some(class::AL));
  assert_eq!(Class::new(0x08a0), Some(class::AL));
  assert_eq!(Class::new(0x089f), None);
  assert_eq!(Class::new(0x08ff), None);

  assert_eq!(Class::new(0x20a0), Some(class::ET));

  assert_eq!(Class::new(0xfb1d), Some(class::R));
  assert_eq!(Class::new(0xfb4f), Some(class::R));
  assert_eq!(Class::new(0xfb50), Some(class::AL));
  assert_eq!(Class::new(0xfdf0), Some(class::AL));
  assert_eq!(Class::new(0xfe70), Some(class::AL));
  assert_eq!(Class::new(0xfeff), Some(class::BN));

  assert_eq!(Class::new(0x10800), Some(class::R));
  assert_eq!(Class::new(0x1e800), Some(class::R));
  assert_eq!(Class::new(0x1ee00), Some(class::AL));

  assert_eq!(Class::new(0x30000), Some(class::L));
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
fn isolating_run_sequences() {
  macro_rules! classes {
    ($($rest:tt),*) => {
      &[$(class::$rest),*]
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
        start_class: class::$sos,
        end_class: class::$eos,
      },)*]
    }
  }

  let mock_paragraph = Paragraph {
    range: 0..1,
    level: Level::ltr(),
    pure_ltr: false,
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
