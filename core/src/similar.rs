use crate::{codepoints::CODEPOINTS, util::read_u16_le};

pub(crate) const SIMILAR_START: u16 = read_u16_le(unsafe { CODEPOINTS.offset(2) });
pub(crate) const SIMILAR_END: u16 = read_u16_le(unsafe { CODEPOINTS.offset(4) });

pub(crate) fn is(self_char: u32, other_char: char) -> bool {
  // SAFETY: even if there is no lowercase mapping for some codepoints, it would just return itself.
  // therefore, the first iteration and/or codepoint always exists.
  let other_char = unsafe { other_char.to_lowercase().next().unwrap_unchecked() as u32 };

  if self_char == other_char {
    return true;
  }

  if self_char <= 0x7f && other_char <= 0x7f {
    let mut offset = SIMILAR_START;
    let mut contains_a = false;
    let mut contains_b = false;

    loop {
      let cur = unsafe { *(CODEPOINTS.offset(offset as _)) };
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

  false
}

#[inline(always)]
pub(crate) fn is_iter<S, O>(s: S, o: O) -> bool
where
  S: Iterator<Item = char>,
  O: Iterator<Item = char>,
{
  s.zip(o)
    .all(|(self_char, other_char)| is(self_char as _, other_char))
}

#[inline(always)]
pub(crate) fn is_str<'a>(s: &'a str, o: &'a str) -> bool {
  is_iter(s.chars(), o.chars())
}
