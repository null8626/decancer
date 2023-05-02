use crate::{cure, cure_char, CuredString, Translation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Decancered {
  cured_string: CuredString,
  translation: Translation,
}

#[test]
fn deserialize() {
  let json = r#"{"cured_string": "vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣","translation": "ӕ"}"#;
  let decancered: Decancered = serde_json::from_str(json).unwrap();

  assert_eq!(decancered.cured_string, "very funny text");
  assert!(matches!(decancered.translation, Translation::String("ae")));
}

#[test]
fn serialize() {
  let decancered = Decancered {
    cured_string: cure("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣"),
    translation: cure_char('ӕ'),
  };

  assert_eq!(
    serde_json::to_string(&decancered).unwrap(),
    r#"{"cured_string":"very funny text","translation":"ae"}"#
  )
}
