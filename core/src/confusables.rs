use std::mem::size_of;

const BINARY: *const u8 = include_bytes!("../bin/confusables.bin").as_ptr();

pub(crate) struct Confusables<'a, I> {
  index: u8,
  inner: &'a I,
}

pub(crate) trait Iterable: Sized {
  type Item: Sized;

  fn iter<'a>(&'a self) -> Confusables<'a, Self> {
    Confusables { index: 0, inner: self }
  }

  fn len(&self) -> u8;
  fn nth(&self, index: u8) -> Self::Item;
}

impl<I: Iterable> Iterator for Confusables<'_, I> {
  type Item = <I as Iterable>::Item;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == self.inner.len() {
      None
    } else {
      let out = self.inner.nth(self.index);
      self.index += 1;
    
      Some(out)
    }
  }
}

impl<I: Iterable> ExactSizeIterator for Confusables<'_, I> {
  #[inline(always)]
  fn len(&self) -> usize {
    self.inner.len() as _
  }
}

#[derive(Copy, Clone)]
pub(crate) struct BinaryArray<T: Copy + PartialEq> {
  inner_len: u8,
  ptr: *const T,
}

impl<T: Copy + PartialEq> BinaryArray<T> {
  const fn new(off: *const u8) -> Self {
    unsafe {
      Self {
        inner_len: *off,
        ptr: off.offset(size_of::<u8>() as _) as _,
      }
    }
  }

  /// Like .any(), but doesn't mutably borrow the value.
  #[inline(always)]
  pub(crate) fn contains(&self, elem: T) -> bool {
    (0..self.len()).any(|x| elem == unsafe { *self.ptr.offset(x as _) })
  }

  pub(crate) const fn size(&self) -> u16 {
    (size_of::<u8>() as u16) + ((self.inner_len as u16) * (size_of::<T>() as u16))
    // size_of::<u8>() is for the u8 specifying the length before the array.
  }
}

impl<T: Copy + PartialEq> Iterable for BinaryArray<T> {
  type Item = T;

  #[inline(always)]
  fn len(&self) -> u8 {
    self.inner_len
  }
  
  #[inline(always)]
  fn nth(&self, index: u8) -> Self::Item {
    unsafe { *self.ptr.offset(index as _) }
  }
}

pub(crate) struct DynamicConfusables<'a, I> {
  index: u8,
  offset: u16,
  inner: &'a I,
}

pub(crate) trait DynamicIterable: Sized {
  type Item: Sized;

  fn iter<'a>(&'a self) -> DynamicConfusables<'a, Self> {
    DynamicConfusables { index: 0, offset: 0, inner: self }
  }

  fn len(&self) -> u8;
  fn advance(&self, offset: &mut u16) -> Self::Item;
}

impl<I: DynamicIterable> Iterator for DynamicConfusables<'_, I> {
  type Item = <I as DynamicIterable>::Item;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == self.inner.len() {
      None
    } else {
      let out = self.inner.advance(&mut self.offset);
      self.index += 1;
    
      Some(out)
    }
  }
}

impl<I: DynamicIterable> ExactSizeIterator for DynamicConfusables<'_, I> {
  #[inline(always)]
  fn len(&self) -> usize {
    self.inner.len() as _
  }
}

#[derive(Copy, Clone)]
pub(crate) struct MiscCaseSensitive {
  inner_len: u8,
  ptr: *const u8,
}

impl MiscCaseSensitive {
  const fn new(off: *const u8) -> Self {
    unsafe {
      Self {
        inner_len: *off,
        ptr: off.offset(size_of::<u8>() as _) as _,
      }
    }
  }
}

impl DynamicIterable for MiscCaseSensitive {
  type Item = (BinaryArray<u8>, BinaryArray<u32>);

  #[inline(always)]
  fn len(&self) -> u8 {
    self.inner_len
  }

  fn advance(&self, offset: &mut u16) -> Self::Item {
    unsafe {
      let a = BinaryArray::new(self.ptr.offset(*offset as _));
      *offset += a.size();

      let b = BinaryArray::new(self.ptr.offset(*offset as _));
      *offset += b.size();

      (a, b)
    }
  }
}

#[derive(Copy, Clone)]
pub(crate) struct Misc {
  inner_len: u8,
  ptr: *const u8,
}

impl Misc {
  const fn new(off: *const u8) -> Self {
    unsafe {
      Self {
        inner_len: *off,
        ptr: off.offset(size_of::<u8>() as _) as _,
      }
    }
  }
}

impl DynamicIterable for Misc {
  type Item = (u8, BinaryArray<u32>);

  #[inline(always)]
  fn len(&self) -> u8 {
    self.inner_len
  }

  fn advance(&self, offset: &mut u16) -> Self::Item {
    unsafe {
      let a = *self.ptr.offset(*offset as _);
      *offset += size_of::<u8>() as u16;

      let b = BinaryArray::new(self.ptr.offset(*offset as _));
      *offset += b.size();

      (a, b)
    }
  }
}

#[derive(Copy, Clone)]
pub(crate) struct Alphabetical {
  ptr: *const u8,
}

impl Alphabetical {
  const fn new(ptr: *const u8) -> Self {
    Self {
      ptr,
    }
  }
}

impl DynamicIterable for Alphabetical {
  type Item = BinaryArray<u32>;
  
  #[inline(always)]
  fn len(&self) -> u8 {
    26
  }

  fn advance(&self, offset: &mut u16) -> Self::Item {
    unsafe {
      let out = BinaryArray::new(self.ptr.offset(*offset as _));
      *offset += out.size();

      out
    }
  }
}

#[derive(Copy, Clone)]
pub(crate) struct Similar {
  inner_len: u8,
  ptr: *const u8,
}

impl Similar {
  const fn new(ptr: *const u8) -> Self {
    unsafe {
      Self {
        inner_len: *ptr,
        ptr: ptr.offset(size_of::<u8>() as _),
      }
    }
  }
}

impl DynamicIterable for Similar {
  type Item = BinaryArray<u8>;

  #[inline(always)]
  fn len(&self) -> u8 {
    self.inner_len
  }

  fn advance(&self, offset: &mut u16) -> Self::Item {
    unsafe {
      let out = BinaryArray::new(self.ptr.offset(*offset as _));
      *offset += out.size();

      out
    }
  }
}

pub(crate) const fn get_ptr(header_index: isize) -> *const u8 {
  unsafe { BINARY.offset(*(BINARY as *const u16).offset(header_index) as _) }
}

pub(crate) const fn numerical() -> BinaryArray<u32> {
  BinaryArray::new(get_ptr(0))
}

pub(crate) const fn misc_case_sensitive() -> MiscCaseSensitive {
  MiscCaseSensitive::new(get_ptr(1))
}

pub(crate) const fn misc() -> Misc {
  Misc::new(get_ptr(2))
}

pub(crate) const fn alphabetical_pattern() -> BinaryArray<u32> {
  BinaryArray::new(get_ptr(3))
}

pub(crate) const fn alphabetical() -> Alphabetical {
  Alphabetical::new(get_ptr(4))
}

pub(crate) const fn similar() -> Similar {
  Similar::new(get_ptr(5))
}
