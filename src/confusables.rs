use std::{slice, mem::size_of};

const BINARY: *const u8 = include_bytes!("../bin/confusables.bin").as_ptr();

pub struct BinaryArray<T: Copy + PartialEq + Sized> {
  index: u8,
  inner_len: u8,
  ptr: *const T,
}

impl<T: Copy + PartialEq + Sized> BinaryArray<T> {
  const fn new(off: *const u8) -> Self {
    unsafe {
      Self {
        index: 0, inner_len: *off, ptr: off.offset(size_of::<u8>() as _) as _
      }
    }
  }

  /// Like .any(), but doesn't mutably borrow the value.
  #[inline(always)]
  pub fn contains(&self, elem: T) -> bool {
    (0..self.len()).any(|x| elem == unsafe { self.get_unchecked(x) })
  }
  
  pub const fn len(&self) -> u8 {
    self.inner_len
  }

  pub const fn size(&self) -> u16 {
    (size_of::<u8>() as u16) + ((self.len() as u16) * (size_of::<T>() as u16)) // size_of::<u8>() is for the u8 specifying the length before the array.
  }

  pub const fn as_ptr(&self) -> *const T {
    self.ptr
  }

  #[inline(always)]
  pub fn as_slice(&self) -> &[T] {
    self.as_ref()
  }

  pub const fn get(&self, index: u8) -> Option<T> {
    if index >= self.len() {
      None
    } else {
      Some(unsafe { self.get_unchecked(index) })
    }
  }

  pub const unsafe fn get_unchecked(&self, index: u8) -> T {
    *self.ptr.offset(index as _)
  }
}

impl<T: Copy + PartialEq + Sized> Iterator for BinaryArray<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == self.len() {
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
    (self.len() as _, Some(self.len() as _))
  }
}

impl<T: Copy + PartialEq + Sized> AsRef<[T]> for BinaryArray<T> {
  #[inline(always)]
  fn as_ref(&self) -> &[T] {
    unsafe { slice::from_raw_parts(self.ptr, self.inner_len as _) }
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
        index: 0, inner_len: *off, offset: 0, ptr: off.offset(size_of::<u8>() as _) as _
      }
    }
  }

  pub const fn len(&self) -> u8 {
    self.inner_len
  }
}

impl Iterator for MiscCaseSensitive {
  type Item = (BinaryArray<u8>, BinaryArray<u32>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == self.len() {
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
    (self.len() as _, Some(self.len() as _))
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
        index: 0, inner_len: *off, offset: 0, ptr: off.offset(size_of::<u8>() as _) as _
      }
    }
  }

  pub const fn len(&self) -> u8 {
    self.inner_len
  }
}

impl Iterator for Misc {
  type Item = (u8, BinaryArray<u32>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == self.len() {
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
    (self.len() as _, Some(self.len() as _))
  }
}

pub struct Alphabetical {
  index: u8,
  offset: u16,
  ptr: *const u8,
}

impl Alphabetical {
  const fn new(ptr: *const u8) -> Self {
    Self { index: 0, offset: 0, ptr }
  }

  pub const fn len(&self) -> u8 {
    26
  }
}

impl Iterator for Alphabetical {
  type Item = BinaryArray<u32>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == self.len() {
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
      Self { index: 0, inner_len: *ptr, offset: 0, ptr: ptr.offset(size_of::<u8>() as _) }
    }
  }

  pub const fn len(&self) -> u8 {
    self.inner_len
  }
}

impl Iterator for Similar {
  type Item = BinaryArray<u8>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == self.len() {
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
}

const fn get_ptr(header_index: isize) -> *const u8 {
  unsafe {
    BINARY.offset(*(BINARY as *const u16).offset(header_index) as _)
  }
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