use proptest::prelude::*;

proptest! {
    // Locally, try running a more exhaustive battery with an environment
    // variable setting like PROPTEST_CASES=1000000.
    #![proptest_config(ProptestConfig::with_cases(2000))]

    #[test]
    fn character_crash(c in any::<char>(), i in any::<u32>()) {
        let _ = crate::cure_char(c);
        let _ = crate::cure_char(i);
    }

    #[test]
    #[cfg(feature = "std")]
    fn string_crash(s in "\\PC*") {
        let _ = crate::cure(&s);
    }
}

#[test]
fn regression_crash() {
    let _ = crate::cure_char('Æ');
    let _ = crate::cure("Æ");

    let _ = crate::cure_char('ˑ');
    let _ = crate::cure("ˑ");
}
