// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

#[repr(C)]
pub(crate) struct Element<T> {
  pub(crate) string: *const T,
  pub(crate) size: usize,
}

pub(crate) struct NullTerminatedPointer<T> {
  ptr: *const T,
  pub(crate) size: usize,
}

impl<T> From<*const T> for NullTerminatedPointer<T> {
  #[inline(always)]
  fn from(ptr: *const T) -> Self {
    Self { ptr, size: 0 }
  }
}

impl<T> Iterator for NullTerminatedPointer<T>
where
  T: PartialEq<T> + Default + Copy,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let value = unsafe { *self.ptr };

    self.ptr = unsafe { self.ptr.offset(1) };

    if value == Default::default() {
      None
    } else {
      self.size += 1;

      Some(value)
    }
  }
}

#[cfg(feature = "utf16")]
pub(crate) struct SizedPointer<T> {
  ptr: *const T,
  size: usize,
}

#[cfg(feature = "utf16")]
impl<T> SizedPointer<T> {
  pub(crate) const fn new(ptr: *const T, size: usize) -> Self {
    Self { ptr, size }
  }
}

#[cfg(feature = "utf16")]
impl<T> Iterator for SizedPointer<T>
where
  T: Copy,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.size == 0 {
      return None;
    }

    let value = unsafe { *self.ptr };

    self.ptr = unsafe { self.ptr.offset(1) };
    self.size -= 1;

    Some(value)
  }
}
