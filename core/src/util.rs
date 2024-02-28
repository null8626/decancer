use std::ops::{Deref, DerefMut, Index, IndexMut, Range};

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

pub(crate) trait RestartableOpt<T>: Iterator<Item = T> {
  fn restart_callback(&mut self) {}
}

pub(crate) struct Restartable<I, E> {
  iterator: I,
  members: Vec<E>,
  index: usize,
}

impl<I, E> Restartable<I, E>
where
  I: RestartableOpt<E>,
{
  #[inline(always)]
  pub(crate) fn new(iterator: I) -> Self {
    let (size_hint, _) = iterator.size_hint();

    Self {
      iterator,
      members: Vec::with_capacity(size_hint),
      index: 0,
    }
  }

  #[inline(always)]
  pub(crate) fn restart(&mut self) {
    self.iterator.restart_callback();
    self.index = 0;
  }
}

impl<I, E> Iterator for Restartable<I, E>
where
  I: RestartableOpt<E>,
  E: Clone,
{
  type Item = E;

  fn next(&mut self) -> Option<Self::Item> {
    self.index += 1;

    match self.members.get(self.index - 1) {
      Some(value) => Some(value.clone()),

      None => {
        let value = self.iterator.next()?;
        self.members.push(value.clone());

        Some(value)
      }
    }
  }
}

impl<I, E> Deref for Restartable<I, E> {
  type Target = I;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.iterator
  }
}

impl<I, E> DerefMut for Restartable<I, E> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.iterator
  }
}

macro_rules! unwrap_or_ret {
  ($value:expr,$fallback:expr) => {
    match $value {
      Some(output) => output,
      None => return $fallback,
    }
  };
}

pub(crate) use unwrap_or_ret;
