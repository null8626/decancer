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
  E: Copy,
{
  type Item = E;

  fn next(&mut self) -> Option<Self::Item> {
    self.index += 1;

    match self.members.get(self.index - 1) {
      Some(value) => Some(*value),

      None => {
        let value = self.iterator.next()?;
        self.members.push(value);

        Some(value)
      }
    }
  }
}

impl<I, E> Deref for Restartable<I, E>
where
  I: RestartableOpt<E>,
{
  type Target = I;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.iterator
  }
}

impl<I, E> DerefMut for Restartable<I, E>
where
  I: RestartableOpt<E>,
{
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.iterator
  }
}

pub(crate) struct Peek<I, E> {
  iterator: I,
  current: E,
  ended: bool,
}

impl<I, E> Peek<I, E>
where
  I: Iterator<Item = E>,
  E: Copy,
{
  #[inline(always)]
  pub(crate) fn new(mut iterator: I) -> Option<Self> {
    iterator.next().map(|current| Self {
      iterator,
      current,
      ended: false,
    })
  }

  pub(crate) const fn has_ended(&self) -> bool {
    self.ended
  }
}

impl<I, E> Iterator for Peek<I, E>
where
  I: Iterator<Item = E>,
  E: Copy,
{
  type Item = (E, Option<E>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.has_ended() {
      return None;
    }

    let current = self.current;
    let next_element = self.iterator.next();

    match next_element {
      Some(next_element_inner) => self.current = next_element_inner,
      None => self.ended = true,
    };

    Some((current, next_element))
  }
}

impl<I, E> RestartableOpt<(E, Option<E>)> for Peek<I, E>
where
  I: Iterator<Item = E>,
  E: Copy,
{
  #[inline(always)]
  fn restart_callback(&mut self) {
    self.ended = false;
  }
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
