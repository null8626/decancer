extern crate napi;

use napi::JsStringUtf16;

pub const fn charcodes(c: u32) -> (u16, Option<u16>) {
  if c <= 0xFFFF {
    (c as u16, None)
  } else {
    let n = (c - 0x10000) as u16;
    (0xD800 + (n >> 10), Some(0xDC00 + (n & 0x3FF)))
  }
}

pub struct Codepoints<'a> {
  iter: &'a [u16],
  inner_index: usize,
}

impl Codepoints<'_> {
  pub const fn index(&self) -> usize {
    self.inner_index
  }

  pub const fn min_len(&self) -> usize {
    self.iter.len() - 1
  }
}

impl<'a> From<&'a JsStringUtf16> for Codepoints<'a> {
  #[inline(always)]
  fn from(s: &'a JsStringUtf16) -> Self {
    Self {
      iter: s.as_slice(),
      inner_index: 0,
    }
  }
}

impl Iterator for Codepoints<'_> {
  type Item = u32;
  
  fn next(&mut self) -> Option<Self::Item> {
    if self.inner_index >= self.min_len() {
      return None;
    }

    let c = unsafe { *self.iter.get_unchecked(self.inner_index) as u32 };

    if c >= 0xD800 && c < 0xDC00 && ((self.inner_index + 1) < self.min_len()) {
      let n = unsafe { *self.iter.get(self.inner_index + 1).unwrap_unchecked() as u32 };
    
      if n >= 0xDC00 && n < 0xE000 {
        self.inner_index += 2;
        return Some(0x10000 + ((c - 0xD800) << 10) + (n - 0xDC00));
      }
    }

    self.inner_index += 1;
    Some(c)
  }

  #[inline(always)]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, Some(self.min_len()))
  }
}
