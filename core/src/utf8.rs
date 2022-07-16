use std::{slice::Iter, mem::MaybeUninit};

#[doc(hidden)]
pub struct Codepoints<'a> {
  iter: Iter<'a, u8>,
}

impl<'a> Codepoints<'a> {
  #[inline(always)]
  pub fn new(arr: &'a [u8]) -> Self {
    Self {
      iter: arr.iter()
    }
  }
}

impl Iterator for Codepoints<'_> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    let current = *self.iter.next()?;

    if (current >= 0xA0 && current <= 0xBF) || current >= 0xF8 {
      return None; // invalid UTF-8
    }

    let mut bytes: u32 = 0;
    let mut mask: u8 = 0x7F;
    let mut shift: u32 = 0;

    if current >= 0xC0 {
      let mut next = *self.iter.next()?;

      if (next >> 6) != 0x02 {
        return None; // invalid UTF-8
      }

      bytes = (next & 0x3F) as u32;

      if current >= 0xE0 {
        next = *self.iter.next()?;

        if (next >> 6) != 0x02 {
          return None; // invalid UTF-8
        }

        bytes = (bytes << 6) | ((next & 0x3F) as u32);
        
        if current >= 0xF0 {
          next = *self.iter.next()?;

          if (next >> 6) != 0x02 {
            return None; // invalid UTF-8
          }

          bytes = (bytes << 6) | ((next & 0x3F) as u32);
          mask = 0x07;
          shift = 18;
        } else {
          shift = 12;
          mask = 0x0F;
        }
      } else {
        shift = 6;
        mask = 0x1F;
      }
    }

    Some((((current & mask) as u32) << shift) | bytes)
  }

  #[allow(unused_assignments)]
  fn count(mut self) -> usize
    where
      Self: Sized
  {
    let mut i = 0;
    let mut current = unsafe { MaybeUninit::uninit().assume_init() };
    let mut next = unsafe { MaybeUninit::uninit().assume_init() };

    loop {
      match self.iter.next() {
        Some(&c) => current = c,
        None => break,
      };

      if (current >= 0xA0 && current <= 0xBF) || current >= 0xF8 {
        break; // invalid UTF-8
      } else if current >= 0xC0 {
        match self.iter.next() {
          Some(&c) => next = c,
          None => break,
        };
  
        if (next >> 6) != 0x02 {
          break; // invalid UTF-8
        }

        if current >= 0xE0 {
          match self.iter.next() {
            Some(&c) => next = c,
            None => break,
          };
  
          if (next >> 6) != 0x02 {
            break; // invalid UTF-8
          }

          if current >= 0xF0 {
            match self.iter.next() {
              Some(&c) => next = c,
              None => break,
            };
  
            if (next >> 6) != 0x02 {
              break; // invalid UTF-8
            }
          }
        }
      }

      i += 1;
    }

    i
  }
}