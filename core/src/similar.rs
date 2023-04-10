use crate::{confusables::CONFUSABLES, util::read_u16_le};

pub(crate) const SIMILAR_START: u16 = read_u16_le(unsafe { CONFUSABLES.offset(2) });
pub(crate) const SIMILAR_END: u16 = read_u16_le(unsafe { CONFUSABLES.offset(4) });

pub(crate) fn is(self_char: u32, other_char: char) -> bool {
  let other_char = unsafe { other_char
    .to_lowercase()
    .next()
  .unwrap_unchecked() as u32 };
  
  if self_char > 0x7f || other_char > 0x7f {
    self_char == other_char
  } else if self_char == other_char {
    true
  } else {
    let mut offset = SIMILAR_START;
    let mut contains_a = false;
    let mut contains_b = false;

    loop {
      let cur = unsafe { *(CONFUSABLES.offset(offset as _)) };
      let sim = cur & 0x7f;

      if sim == (self_char as u8) {
        contains_a = true;
      }

      if sim == (other_char as u8) {
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
