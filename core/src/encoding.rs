pub(crate) const fn charcodes(c: u32) -> (u16, Option<u16>) {
  if c <= 0xFFFF {
    (c as u16, None)
  } else {
    let n = (c - 0x10000) as u16;
    (0xD800 + (n >> 10), Some(0xDC00 + (n & 0x3FF)))
  }
}

pub(crate) struct Codepoints<I: Iterator<Item = u16>> {
  iter: I,
  next: Option<u16>,
}

impl<I: Iterator<Item = u16>> From<I> for Codepoints<I> {
  #[inline(always)]
  fn from(iter: I) -> Self {
    Self { iter, next: None }
  }
}

impl<I: Iterator<Item = u16>> Iterator for Codepoints<I> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    let c = self.next.take().unwrap_or(self.iter.next()?) as u32;

    if c >= 0xD800 && c < 0xDC00 {
      if let Some(n) = self.iter.next() {
        if n >= 0xDC00 && n < 0xE000 {
          return Some(0x10000 + ((c - 0xD800) << 10) + ((n as u32) - 0xDC00));
        } else {
          self.next.replace(n);
        }
      }
    }

    Some(c)
  }
}
