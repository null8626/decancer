use super::{BIDI, BIDI_BRACKETS_COUNT};
use crate::util::{read_u16_le, read_u32_le, CODEPOINT_MASK};
use core::cmp::{max, min};

pub(crate) struct BracketPair {
  pub(crate) start: usize,
  pub(crate) end: usize,
  pub(crate) start_run: usize,
  pub(crate) end_run: usize,
}

pub(crate) struct OpeningBracket {
  pub(crate) opening: u32,
  pub(crate) is_open: bool,
}

impl OpeningBracket {
  pub(crate) fn new(code: u32) -> Option<Self> {
    let mut start = 0i32;
    let mut end = BIDI_BRACKETS_COUNT as i32;

    while start <= end {
      let mid = (start + end) / 2;
      let offset = (4 + (mid * 5)) as isize;

      let first = read_u32_le(unsafe { BIDI.offset(offset) });
      let opening =
        ((read_u16_le(unsafe { BIDI.offset(offset + 4) }) as u32) << 8) | ((first >> 20) & 0xff);

      let diff = (first >> 28) & 7;

      let closing = if (first >> 31) == 1 {
        opening - diff
      } else {
        opening + diff
      };

      if code < min(opening, closing) {
        end = mid - 1;
      } else if code > max(opening, closing) {
        start = mid + 1;
      } else {
        let is_open = code == opening;

        if is_open || code == closing {
          let mut decomps = first & CODEPOINT_MASK;

          if decomps == 0 {
            decomps = opening;
          }

          return Some(Self {
            opening: decomps,
            is_open,
          });
        }

        break;
      }
    }

    None
  }
}
