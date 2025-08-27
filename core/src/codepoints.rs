#[cfg(feature = "options")]
use crate::Options;
use crate::{
  similar::SIMILAR_START,
  translation::Translation,
  util::{Binary, CODEPOINT_MASK},
};
use std::cmp::Ordering;

pub(crate) const CODEPOINTS: Binary = Binary::new(include_bytes!("../bin/codepoints.bin"));

// - 1 because we're only using them in binary search
pub(crate) const CASE_SENSITIVE_CODEPOINTS_COUNT: u16 =
  ((SIMILAR_START - CASE_SENSITIVE_CODEPOINTS_OFFSET) / 6) - 1;
pub(crate) const CASE_SENSITIVE_CODEPOINTS_OFFSET: u16 = CODEPOINTS.u16_at(0);
pub(crate) const CODEPOINTS_COUNT: u16 = ((CASE_SENSITIVE_CODEPOINTS_OFFSET - 6) / 6) - 1;

const STRING_TRANSLATION_MASK: u32 = 0x10000000;

#[derive(Copy, Clone)]
#[cfg_attr(not(feature = "options"), allow(dead_code))]
pub(crate) struct Codepoint(u32, u8, u8);

impl Codepoint {
  const fn get_codepoint(self) -> u32 {
    self.0 & CODEPOINT_MASK
  }

  const fn range_size(self) -> u32 {
    if self.is_string_translation() {
      0
    } else {
      (self.1 & 0x7f) as _
    }
  }

  const fn is_string_translation(self) -> bool {
    self.0 >= STRING_TRANSLATION_MASK
  }

  const fn ascii_translation(self) -> u32 {
    (self.0 >> 20) & 0x7f
  }

  const fn is_translation_synced(self) -> bool {
    self.1 >= 0x80
  }

  pub(crate) const fn at(offset: i32) -> Self {
    Self(
      CODEPOINTS.u32_at(offset as _),
      CODEPOINTS.at((4 + offset) as _),
      CODEPOINTS.at((5 + offset) as _),
    )
  }

  pub(crate) const fn matches(
    self,
    other: u32,
    #[cfg(feature = "options")] options: Options,
  ) -> Option<Ordering> {
    let mut conf = self.get_codepoint();

    if other < conf {
      return Some(Ordering::Less);
    }

    conf += self.range_size();

    if other > conf {
      return Some(Ordering::Greater);
    }

    #[cfg(feature = "options")]
    if options.refuse_cure(self.2) {
      return None;
    }

    Some(Ordering::Equal)
  }

  pub(crate) fn translation(self, other: u32) -> Translation {
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
