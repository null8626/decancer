use super::{BIDI, BIDI_DICTIONARY_COUNT, BIDI_DICTIONARY_OFFSET};
use crate::util::{read_u16_le, read_u32_le, CODEPOINT_MASK};
use std::mem::transmute;

use super::OverrideStatus;

#[repr(u8)]
#[allow(dead_code)]
#[cfg_attr(test, derive(Debug))]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Class {
  B,
  S,
  WS,
  ON,
  ET,
  ES,
  CS,
  EN,
  L,
  BN,
  R,
  AN,
  AL,
  LRE,
  RLE,
  PDF,
  LRO,
  RLO,
  LRI,
  RLI,
  FSI,
  PDI,
}

impl Class {
  pub(crate) const fn new(code: u32) -> Option<Self> {
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
        return Some(unsafe { transmute((kv >> 20) as u8) });
      }
    }

    None
  }

  pub(crate) const fn is_neutral_or_isolate(self) -> bool {
    matches!(self, Self::B | Self::S | Self::WS | Self::ON | Self::PDI) || self.is_isolate()
  }

  pub(crate) const fn is_rtl(self) -> bool {
    matches!(self, Self::RLE | Self::RLO | Self::RLI)
  }

  pub(crate) const fn is_isolate(self) -> bool {
    matches!(self, Self::RLI | Self::LRI | Self::FSI)
  }

  pub(crate) const fn override_status(self) -> OverrideStatus {
    match self {
      Self::RLO => OverrideStatus::RTL,
      Self::LRO => OverrideStatus::LTR,
      Self::RLI | Self::LRI | Self::FSI => OverrideStatus::Isolate,
      _ => OverrideStatus::Neutral,
    }
  }

  pub(crate) const fn removed_by_x9(self) -> bool {
    matches!(
      self,
      Self::RLE | Self::LRE | Self::RLO | Self::LRO | Self::PDF | Self::BN
    )
  }
}
