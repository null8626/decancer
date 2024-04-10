use std::{
  cmp::max,
  ops::{Index, IndexMut, Range},
};

pub(crate) const CODEPOINT_MASK: u32 = 0x000f_ffff;

pub(crate) const fn read_u32_le(ptr: *const u8) -> u32 {
  unsafe { u32::from_le_bytes([*ptr, *ptr.offset(1), *ptr.offset(2), *ptr.offset(3)]) }
}

pub(crate) const fn read_u16_le(ptr: *const u8) -> u16 {
  unsafe { u16::from_le_bytes([*ptr, *ptr.offset(1)]) }
}

#[inline(always)]
pub(crate) fn sliced<T: Index<Range<usize>> + ?Sized>(
  slicable: &T,
  range: Range<usize>,
) -> &<T as Index<Range<usize>>>::Output {
  slicable.index(range)
}

#[inline(always)]
pub(crate) fn sliced_mut<T: IndexMut<Range<usize>> + ?Sized>(
  slicable: &mut T,
  range: Range<usize>,
) -> &mut <T as Index<Range<usize>>>::Output {
  slicable.index_mut(range)
}

// special thanks to https://medium.com/@michealkeines/merge-overlapping-intervals-rust-117a7099f348
// except i've improved upon it :)
pub(crate) fn merge_ranges<T>(ranges: &mut Vec<Range<T>>)
where
  T: Ord + Copy,
{
  if ranges.is_empty() {
    return;
  }

  ranges.sort_by(|a, b| a.start.cmp(&b.start));

  let mut j = 0;

  for i in 1..ranges.len() {
    let current = ranges[i].clone();
    let previous = &mut ranges[j];

    if current.start >= previous.start && current.start <= previous.end {
      previous.end = max(current.end, previous.end);
    } else {
      j += 1;
      ranges[j] = current;
    }
  }

  ranges.truncate(j + 1);
}

macro_rules! unwrap_or_ret {
  ($option:expr,$fallback:expr) => {
    match $option {
      Some(inner) => inner,
      None => return $fallback,
    }
  };
}

pub(crate) use unwrap_or_ret;
