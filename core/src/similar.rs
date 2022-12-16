use super::{matcher::CONFUSABLES, util::read_u16_le};

const SIMILAR_START: u16 = read_u16_le(CONFUSABLES);
const SIMILAR_END: u16 = read_u16_le(unsafe { CONFUSABLES.offset(2) });

pub(crate) fn is(a: *const u8, b: *const u8) -> u8 {
  unsafe {
    if *a == *b {
      if *a <= 0x7F {
        1
      } else {
        let count: u8 = {
          if *a >= 0xF0 {
            4
          } else if *a >= 0xE0 {
            3
          } else {
            2
          }
        };

        for i in 1isize..count as isize {
          if *a.offset(i) != *b.offset(i) {
            return 0;
          }
        }

        count
      }
    } else if *b > 0x7F || *a > 0x7F {
      0 // nah fuck it we're giving up this time
    } else {
      let mut offset = SIMILAR_START;

      while offset < SIMILAR_END {
        let mut contains_a = false;
        let mut contains_b = false;

        let len = *CONFUSABLES.offset(offset as _);
        offset += 1;

        for part in (offset..offset + (len as u16)).map(|x| *CONFUSABLES.offset(x as _)) {
          if part == *a {
            contains_a = true;
          } else if part == *b {
            contains_b = true;
          }

          if contains_a && contains_b {
            return 1;
          }
        }

        offset += len as u16;
      }

      0
    }
  }
}
