// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use super::{super::util::CODEPOINT_MASK, BIDI, BIDI_BRACKETS_COUNT};

pub(in super::super) struct BracketPair {
  pub(in super::super) start: usize,
  pub(in super::super) end: usize,
  pub(in super::super) start_run: usize,
  pub(in super::super) end_run: usize,
}

pub(in super::super) struct OpeningBracket {
  pub(in super::super) opening: u32,
  pub(in super::super) is_open: bool,
}

// std::cmp::{max, min}; functions are not const because they have generics that prevent it from doing so

const fn min_max(a: u32, b: u32) -> (u32, u32) {
  if a > b { (b, a) } else { (a, b) }
}

impl OpeningBracket {
  pub(in super::super) const fn new(code: u32) -> Option<Self> {
    let mut start = 0i32;
    let mut end = BIDI_BRACKETS_COUNT as i32;

    while start <= end {
      let mid = start.midpoint(end);
      let offset = (4 + (mid * 5)) as _;

      let first = BIDI.u32_at(offset);
      let opening = ((BIDI.u16_at(offset + 4) as u32) << 8) | ((first >> 20) & 0xff);

      let diff = (first >> 28) & 7;

      let closing = if first >= 0x80000000 {
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
