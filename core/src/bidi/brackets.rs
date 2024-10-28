use super::{BIDI, BIDI_BRACKETS_COUNT};
use crate::util::CODEPOINT_MASK;

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

// std::cmp::{max, min}; functions are not const because they have generics that prevent it from doing so

const fn min_max(a: u32, b: u32) -> (u32, u32) {
  if a > b {
    (b, a)
  } else {
    (a, b)
  }
}

impl OpeningBracket {
  pub(crate) const fn new(code: u32) -> Option<Self> {
    let mut start = 0i32;
    let mut end = BIDI_BRACKETS_COUNT as i32;

    while start <= end {
      let mid = (start + end) / 2;
      let offset = (4 + (mid * 5)) as _;

      let first = BIDI.u32_at(offset);
      let opening = ((BIDI.u16_at(offset + 4) as u32) << 8) | ((first >> 20) & 0xff);

      let diff = (first >> 28) & 7;

      let closing = if (first >> 31) == 1 {
        opening - diff
      } else {
        opening + diff
      };

      let (min, max) = min_max(opening, closing);

      if code < min {
        end = mid - 1;
      } else if code > max {
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
