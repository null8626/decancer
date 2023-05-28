use crate::{
  similar::SIMILAR_START,
  translation::Translation,
  util::{read_u16_le, read_u32_le},
};
use core::cmp::Ordering;

pub(crate) const CODEPOINTS: *const u8 = include_bytes!("../bin/codepoints.bin").as_ptr();

const CASE_SENSITIVE_CODEPOINTS_OFFSET: u16 = read_u16_le(CODEPOINTS);
pub(crate) const CODEPOINTS_COUNT: u16 = ((CASE_SENSITIVE_CODEPOINTS_OFFSET - 6) / 5) - 1;
pub(crate) const CASE_SENSITIVE_CODEPOINTS_COUNT: u16 =
  ((SIMILAR_START - CASE_SENSITIVE_CODEPOINTS_OFFSET) / 5) - 1;

const CODEPOINT_MASK: u32 = 0x1fffff;
const RANGE_MASK: u32 = 0x20000000;
const STRING_TRANSLATION_MASK: u32 = 0x40000000;

pub(crate) struct Codepoint(u32, u8);

impl Codepoint {
  pub(crate) const fn at(index: u16) -> Self {
    unsafe {
      Self(
        read_u32_le(CODEPOINTS.offset(6 + (index * 5) as isize)),
        *CODEPOINTS.offset(10 + (index * 5) as isize),
      )
    }
  }

  pub(crate) const fn case_sensitive_at(index: u16) -> Self {
    unsafe {
      Self(
        read_u32_le(CODEPOINTS.offset((CASE_SENSITIVE_CODEPOINTS_OFFSET + (index * 5)) as _)),
        *CODEPOINTS.offset((CASE_SENSITIVE_CODEPOINTS_OFFSET + 4 + (index * 5)) as _),
      )
    }
  }

  pub(crate) const fn matches(&self, other: u32) -> Ordering {
    let conf: u32 = self.0 & CODEPOINT_MASK;

    if other < conf {
      return Ordering::Less;
    }

    let mut max = conf;

    if (self.0 & RANGE_MASK) != 0 {
      max += (self.1 & 0x7f) as u32;
    }

    if other > max {
      Ordering::Greater
    } else {
      Ordering::Equal
    }
  }

  pub(crate) const fn translation(&self, other: u32) -> Translation {
    if (self.0 & STRING_TRANSLATION_MASK) != 0 {
      Translation::string(self.0, self.1)
    } else {
      let mut code = (self.0 >> 21) & 0xff;

      if code == 0 {
        return Translation::None;
      } else if self.1 >= 0x80 {
        code += other - (self.0 & CODEPOINT_MASK);
      }

      Translation::character(code)
    }
  }
}
