use super::{BIDI, BIDI_BRACKETS_COUNT};
use crate::util::{read_u16_le, read_u32_le, CODEPOINT_MASK};

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
    let mut start = 0;
    let mut end = BIDI_BRACKETS_COUNT;

    while start <= end {
      let mid = (start + end) / 2;
      let offset = (4 + (mid * 5)) as isize;

      let first = read_u32_le(unsafe { BIDI.offset(offset) });
      let other =
        ((read_u16_le(unsafe { BIDI.offset(offset + 4) }) as u32) << 8) | ((first >> 20) & 0xff);

      if code < other {
        end = mid - 1;
      } else if code > other {
        start = mid + 1;
      } else {
        let mut diff = (first >> 28) & 7;

        let closing = if (first >> 31) == 1 {
          other + diff
        } else {
          other - diff
        };

        let mut opening = first & CODEPOINT_MASK;

        if opening == 0 {
          opening = other;
        }

        return Some(Self {
          opening,
          is_open: code == other,
        });
      }
    }

    None
  }
}
