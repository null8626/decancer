use std::{mem::size_of, slice};

const BINARY: *const u8 = include_bytes!("../bin/confusables.bin").as_ptr();

pub struct BinaryArray<T: Copy + PartialEq> {
  index: u8,
  inner_len: u8,
  ptr: *const T,
}

impl<T: Copy + PartialEq> BinaryArray<T> {
  const fn new(off: *const u8) -> Self {
    unsafe {
      Self {
        index: 0,
        inner_len: *off,
        ptr: off.offset(size_of::<u8>() as _) as _,
      }
    }
  }

  /// Like .any(), but doesn't mutably borrow the value.
  #[inline(always)]
  pub fn contains(&self, elem: T) -> bool {
    (0..self.len()).any(|x| elem == unsafe { *self.ptr.offset(x as _) })
  }

  pub const fn size(&self) -> u16 {
    (size_of::<u8>() as u16) + ((self.inner_len as u16) * (size_of::<T>() as u16))
    // size_of::<u8>() is for the u8 specifying the length before the array.
  }
}

impl<T: Copy + PartialEq> Iterator for BinaryArray<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.inner_len {
      None
    } else {
      let out = Some(unsafe { *self.ptr.offset(self.index as _) });
      self.index += size_of::<u8>() as u8;

      out
    }
  }

  fn nth(&mut self, n: usize) -> Option<Self::Item> {
    if n > 0xFF || ((n as u8) >= self.inner_len) {
      None
    } else {
      Some(unsafe { *self.ptr.offset(n as _) })
    }
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

impl<T: Copy + PartialEq> AsRef<[T]> for BinaryArray<T> {
  #[inline(always)]
  fn as_ref(&self) -> &[T] {
    unsafe { slice::from_raw_parts(self.ptr, self.inner_len as _) }
  }
}

impl<T: Copy + PartialEq> ExactSizeIterator for BinaryArray<T> {
  #[inline(always)]
  fn len(&self) -> usize {
    self.inner_len as _
  }
}

pub struct MiscCaseSensitive {
  index: u8,
  inner_len: u8,
  offset: u16,
  ptr: *const u8,
}

impl MiscCaseSensitive {
  const fn new(off: *const u8) -> Self {
    unsafe {
      Self {
        index: 0,
        inner_len: *off,
        offset: 0,
        ptr: off.offset(size_of::<u8>() as _) as _,
      }
    }
  }
}

impl Iterator for MiscCaseSensitive {
  type Item = (BinaryArray<u8>, BinaryArray<u32>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.inner_len {
      None
    } else {
      unsafe {
        let a = BinaryArray::new(self.ptr.offset(self.offset as _));
        self.offset += a.size();

        let b = BinaryArray::new(self.ptr.offset(self.offset as _));
        self.offset += b.size();
        self.index += 1;

        Some((a, b))
      }
    }
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

impl ExactSizeIterator for MiscCaseSensitive {
  #[inline(always)]
  fn len(&self) -> usize {
    self.inner_len as _
  }
}

pub struct Misc {
  index: u8,
  inner_len: u8,
  offset: u16,
  ptr: *const u8,
}

impl Misc {
  const fn new(off: *const u8) -> Self {
    unsafe {
      Self {
        index: 0,
        inner_len: *off,
        offset: 0,
        ptr: off.offset(size_of::<u8>() as _) as _,
      }
    }
  }
}

impl Iterator for Misc {
  type Item = (u8, BinaryArray<u32>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.inner_len {
      None
    } else {
      unsafe {
        let a = *self.ptr.offset(self.offset as _);
        self.offset += size_of::<u8>() as u16;

        let b = BinaryArray::new(self.ptr.offset(self.offset as _));
        self.offset += b.size();
        self.index += 1;

        Some((a, b))
      }
    }
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

impl ExactSizeIterator for Misc {
  #[inline(always)]
  fn len(&self) -> usize {
    self.inner_len as _
  }
}

pub struct Alphabetical {
  index: u8,
  offset: u16,
  ptr: *const u8,
}

impl Alphabetical {
  const fn new(ptr: *const u8) -> Self {
    Self {
      index: 0,
      offset: 0,
      ptr,
    }
  }
}

impl Iterator for Alphabetical {
  type Item = BinaryArray<u32>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= 26 {
      None
    } else {
      unsafe {
        let out = BinaryArray::new(self.ptr.offset(self.offset as _));
        self.offset += out.size();
        self.index += 1;

        Some(out)
      }
    }
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (26, Some(26))
  }
}

impl ExactSizeIterator for Alphabetical {
  #[inline(always)]
  fn len(&self) -> usize {
    26
  }
}

pub struct Similar {
  index: u8,
  inner_len: u8,
  offset: u16,
  ptr: *const u8,
}

impl Similar {
  const fn new(ptr: *const u8) -> Self {
    unsafe {
      Self {
        index: 0,
        inner_len: *ptr,
        offset: 0,
        ptr: ptr.offset(size_of::<u8>() as _),
      }
    }
  }
}

impl Iterator for Similar {
  type Item = BinaryArray<u8>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.inner_len {
      None
    } else {
      unsafe {
        let out = BinaryArray::new(self.ptr.offset(self.offset as _));
        self.offset += out.size();
        self.index += 1;

        Some(out)
      }
    }
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

impl ExactSizeIterator for Similar {
  #[inline(always)]
  fn len(&self) -> usize {
    self.inner_len as _
  }
}

const fn get_ptr(header_index: isize) -> *const u8 {
  unsafe { BINARY.offset(*(BINARY as *const u16).offset(header_index) as _) }
}

pub const fn numerical() -> BinaryArray<u32> {
  BinaryArray::new(get_ptr(0))
}

pub const fn misc_case_sensitive() -> MiscCaseSensitive {
  MiscCaseSensitive::new(get_ptr(1))
}

pub const fn misc() -> Misc {
  Misc::new(get_ptr(2))
}

pub const fn alphabetical_pattern() -> BinaryArray<u32> {
  BinaryArray::new(get_ptr(3))
}

pub const fn alphabetical() -> Alphabetical {
  Alphabetical::new(get_ptr(4))
}

pub const fn similar() -> Similar {
  Similar::new(get_ptr(5))
}
