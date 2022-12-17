use super::{matcher::CONFUSABLES, util::read_u16_le};

const SIMILAR_START: u16 = read_u16_le(CONFUSABLES);
const SIMILAR_END: u16 = read_u16_le(unsafe { CONFUSABLES.offset(2) });

pub(crate) fn is(a: u32, b: u32) -> bool {
  if a > 0xff || b > 0xff {
    a == b
  } else if a == b {
    true
  } else {
    let mut offset = SIMILAR_START;

    while offset < SIMILAR_END {
      let mut contains_a = false;
      let mut contains_b = false;

      let len = unsafe { *CONFUSABLES.offset(offset as _) };
      offset += 1;

      for part in (offset..offset + (len as u16)).map(|x| unsafe { *CONFUSABLES.offset(x as _) }) {
        if part == (a as u8) {
          contains_a = true;
        } else if part == (b as u8) {
          contains_b = true;
        }

        if contains_a && contains_b {
          return true;
        }
      }

      offset += len as u16;
    }

    false
  }
}
