use proptest::prelude::*;

proptest! {
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
