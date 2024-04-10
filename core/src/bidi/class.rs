use super::{OverrideStatus, BIDI, BIDI_DICTIONARY_COUNT, BIDI_DICTIONARY_OFFSET};
use crate::util::{read_u16_le, read_u32_le, CODEPOINT_MASK};

macro_rules! class_constants {
  ($($name:ident: $value:literal,)*) => {
    $(pub(crate) const $name: Class = Class($value);)*
  };
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct Class(u8);

class_constants! {
  B: 0,
  S: 1,
  WS: 2,
  ON: 3,
  ET: 4,
  ES: 5,
  CS: 6,
  EN: 7,
  L: 8,
  BN: 9,
  R: 10,
  AN: 11,
  AL: 12,
  LRE: 13,
  RLE: 14,
  PDF: 15,
  LRO: 16,
  RLO: 17,
  LRI: 18,
  RLI: 19,
  FSI: 20,
  PDI: 21,
}

impl Class {
  pub(crate) fn new(code: u32) -> Option<Self> {
    let mut start = 0i32;
    let mut end = BIDI_DICTIONARY_COUNT as i32;

    while start <= end {
      let mid = (start + end) / 2;
      let offset = ((BIDI_DICTIONARY_OFFSET as i32) + (mid * 6)) as isize;

      let kv = read_u32_le(unsafe { BIDI.offset(offset) });

      let other = kv & CODEPOINT_MASK;

      if code < other {
        end = mid - 1;
      } else if code > (other + read_u16_le(unsafe { BIDI.offset(offset + 4) }) as u32) {
        start = mid + 1;
      } else {
        return Some(Self((kv >> 20) as _));
      }
    }

    None
  }

  pub(crate) const fn is_neutral_or_isolate(self) -> bool {
    matches!(self, B | S | WS | ON | PDI) || self.is_isolate()
  }

  pub(crate) const fn is_rtl(self) -> bool {
    matches!(self, RLE | RLO | RLI)
  }

  pub(crate) const fn is_isolate(self) -> bool {
    matches!(self, RLI | LRI | FSI)
  }

  pub(crate) const fn override_status(self) -> OverrideStatus {
    match self {
      RLO => OverrideStatus::RTL,
      LRO => OverrideStatus::LTR,
      RLI | LRI | FSI => OverrideStatus::Isolate,
      _ => OverrideStatus::Neutral,
    }
  }

  pub(crate) const fn removed_by_x9(self) -> bool {
    matches!(self, RLE | LRE | RLO | LRO | PDF | BN)
  }
}
