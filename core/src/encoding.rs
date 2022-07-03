use std::slice::Iter;

pub const fn charcodes(c: u32) -> (u16, Option<u16>) {
  if c <= 0xFFFF {
    (c as u16, None)
  } else {
    let n = (c - 0x10000) as u16;
    (0xD800 + (n >> 10), Some(0xDC00 + (n & 0x3FF)))
  }
}

pub struct Codepoints<'a> {
  iter: Iter<'a, u16>,
  next: Option<u16>,
}

impl<'a> From<&'a [u16]> for Codepoints<'a> {
  #[inline(always)]
  fn from(s: &'a [u16]) -> Self {
    Self {
      iter: s.iter(),
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

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, Some(self.iter.len()))
  }
}
