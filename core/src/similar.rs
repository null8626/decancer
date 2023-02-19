use super::{confusables::CONFUSABLES, util::read_u16_le};

pub(crate) const SIMILAR_START: u16 = read_u16_le(unsafe { CONFUSABLES.offset(2) });
pub(crate) const SIMILAR_END: u16 = read_u16_le(unsafe { CONFUSABLES.offset(4) });

pub(crate) fn is(a: u32, b: u32) -> bool {
  if a > 0x7f || b > 0x7f {
    a == b
  } else if a == b {
    true
  } else {
    let mut offset = SIMILAR_START;
    let mut contains_a = false;
    let mut contains_b = false;

    loop {
      let cur = unsafe { *(CONFUSABLES.offset(offset as _)) };
      let sim = if cur >= 0x80 { cur & 0x7f } else { cur };

      if sim == (a as u8) {
        contains_a = true;
      }

      if sim == (b as u8) {
        contains_b = true;
      }

      if contains_a && contains_b {
        return true;
      }

      if cur >= 0x80 {
        contains_a = false;
        contains_b = false;
      }

      offset += 1;
      if offset == SIMILAR_END {
        return false;
      }
    }
  }
}
