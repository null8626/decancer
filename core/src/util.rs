use std::ops::{Index, IndexMut, Range};

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
