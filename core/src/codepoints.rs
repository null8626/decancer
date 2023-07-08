use crate::{
  similar::SIMILAR_START,
  translation::Translation,
  util::{read_u16_le, read_u32_le},
};
use core::cmp::Ordering;

pub(crate) const CODEPOINTS: *const u8 = include_bytes!("../bin/codepoints.bin").as_ptr();

pub(crate) const CASE_SENSITIVE_CODEPOINTS_COUNT: u16 =
  ((SIMILAR_START - CASE_SENSITIVE_CODEPOINTS_OFFSET) / 5) - 1;
pub(crate) const CASE_SENSITIVE_CODEPOINTS_OFFSET: u16 = read_u16_le(CODEPOINTS);
pub(crate) const CODEPOINTS_COUNT: u16 = ((CASE_SENSITIVE_CODEPOINTS_OFFSET - 6) / 5) - 1;

const CODEPOINT_MASK: u32 = 0x000f_ffff;
const RANGE_MASK: u32 = 0x0800_0000;
const STRING_TRANSLATION_MASK: u32 = 0x1000_0000;

pub(crate) struct Codepoint(u32, u8);

impl Codepoint {
  pub(crate) const fn at(offset: i32) -> Self {
    unsafe {
      Self(
        read_u32_le(CODEPOINTS.offset(offset as _)),
        *CODEPOINTS.offset((4 + offset) as _),
      )
    }
  }

  pub(crate) const fn matches(&self, other: u32) -> Ordering {
    let mut conf: u32 = self.0 & CODEPOINT_MASK;

    if other < conf {
      return Ordering::Less;
    } else if (self.0 & RANGE_MASK) != 0 {
      conf += (self.1 & 0x7f) as u32;
    }

    if other > conf {
      Ordering::Greater
    } else {
      Ordering::Equal
    }
  }

  pub(crate) const fn translation(&self, other: u32) -> Translation {
    if (self.0 & STRING_TRANSLATION_MASK) == 0 {
      let mut code = (self.0 >> 20) & 0x7f;

      if code == 0 {
        return Translation::None;
      } else if self.1 >= 0x80 {
        code += other - (self.0 & CODEPOINT_MASK);
      }

      Translation::character(code)
    } else {
      Translation::string(self.0, self.1)
    }
  }
}
