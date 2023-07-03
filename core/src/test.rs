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
fn regression_crash() {
  let _ = crate::cure_char('\u{c0}');
  let _ = crate::cure_char('\u{16e5f}');
  let _ = crate::cure_char(char::MAX);
}
