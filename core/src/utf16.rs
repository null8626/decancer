use std::{mem::MaybeUninit, slice::Iter};

pub(crate) const fn from(c: u32) -> (u16, Option<u16>) {
  if c <= 0xFFFF {
    (c as u16, None)
  } else {
    let n = (c - 0x10000) as u16;
    (0xD800 + (n >> 10), Some(0xDC00 + (n & 0x3FF)))
  }
}

#[doc(hidden)]
pub struct Codepoints<'a> {
  iter: Iter<'a, u16>,
  next: Option<u16>,
}

impl<'a> Codepoints<'a> {
  #[inline(always)]
  pub fn new(arr: &'a [u16]) -> Self {
    Self {
      iter: arr.iter(),
      next: None,
    }
  }
}

impl Iterator for Codepoints<'_> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    let c = self.next.take().unwrap_or(*self.iter.next()?) as u32;

    if c >= 0xD800 && c < 0xDC00 {
      if let Some(&n) = self.iter.next() {
        if n >= 0xDC00 && n < 0xE000 {
          return Some(0x10000 + ((c - 0xD800) << 10) + ((n as u32) - 0xDC00));
        } else {
          self.next.replace(n);
        }
      }
    }

    Some(c)
  }

  #[allow(unused_assignments)]
  fn count(mut self) -> usize
  where
    Self: Sized,
  {
    let mut i = 0;

    // before any rustacean screams at me, this WILL be initiated in the loop
    let mut c = unsafe { MaybeUninit::uninit().assume_init() };

    loop {
      match self.next.take() {
        Some(ne) => c = ne,
        None => match self.iter.next() {
          Some(&it) => c = it,
          None => break,
        },
      };

      if c >= 0xD800 && c < 0xDC00 {
        if let Some(&n) = self.iter.next() {
          if n < 0xDC00 || n >= 0xE000 {
            self.next.replace(n);
          }
        }
      }

      i += 1;
    }

    i
  }
}
