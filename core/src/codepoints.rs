use crate::{
  similar::SIMILAR_START,
  translation::Translation,
  util::{read_u16_le, read_u32_le, CODEPOINT_MASK},
  Options,
};
use std::cmp::Ordering;

pub(crate) const CODEPOINTS: *const u8 = include_bytes!("../bin/codepoints.bin").as_ptr();

pub(crate) const CASE_SENSITIVE_CODEPOINTS_COUNT: u16 =
  ((SIMILAR_START - CASE_SENSITIVE_CODEPOINTS_OFFSET) / 6) - 1;
pub(crate) const CASE_SENSITIVE_CODEPOINTS_OFFSET: u16 = read_u16_le(CODEPOINTS);
pub(crate) const CODEPOINTS_COUNT: u16 = ((CASE_SENSITIVE_CODEPOINTS_OFFSET - 6) / 6) - 1;

const RANGE_MASK: u32 = 0x0800_0000;
const STRING_TRANSLATION_MASK: u32 = 0x1000_0000;

#[derive(Copy, Clone)]
pub(crate) struct Codepoint(u32, u8, u8);

impl Codepoint {
  const fn get_codepoint(self) -> u32 {
    self.0 & CODEPOINT_MASK
  }

  const fn range_end(self) -> Option<u32> {
    if (self.0 & RANGE_MASK) != 0 {
      Some((self.1 & 0x7f) as _)
    } else {
      None
    }
  }

  const fn is_string_translation(self) -> bool {
    (self.0 & STRING_TRANSLATION_MASK) != 0
  }

  const fn ascii_translation(self) -> u32 {
    (self.0 >> 20) & 0x7f
  }

  const fn is_translation_synced(self) -> bool {
    self.1 >= 0x80
  }

  pub(crate) const fn at(offset: i32) -> Self {
    unsafe {
      Self(
        read_u32_le(CODEPOINTS.offset(offset as _)),
        *CODEPOINTS.offset((4 + offset) as _),
        *CODEPOINTS.offset((5 + offset) as _),
      )
    }
  }

  pub(crate) const fn matches(self, other: u32, options: Options) -> Option<Ordering> {
    let mut conf = self.get_codepoint();

    if other < conf {
      return Some(Ordering::Less);
    } else if let Some(range_end) = self.range_end() {
      conf += range_end;
    }

    if other > conf {
      Some(Ordering::Greater)
    } else if options.refuse_cure(self.2) {
      None
    } else {
      Some(Ordering::Equal)
    }
  }

  pub(crate) const fn translation(self, other: u32) -> Translation {
    if self.is_string_translation() {
      Translation::string(self.0, self.1)
    } else {
      let mut code = self.ascii_translation();

      if code == 0 {
        return Translation::None;
      } else if self.is_translation_synced() {
        code += other - self.get_codepoint();
      }

      Translation::character(code)
    }
  }
}
