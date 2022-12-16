use super::util::{read_u16_le, read_u32_le};
use core::{slice, str};
use std::{cmp::Ordering, mem::transmute};

pub(crate) const CONFUSABLES: *const u8 = include_bytes!("../bin/confusables.bin").as_ptr();
pub(crate) const CONFUSABLES_COUNT: u16 = ((read_u16_le(CONFUSABLES) - 4) / 5) - 1;

pub(crate) struct Confusable(u32, u8);

pub(crate) enum Translation {
  Character(char),
  String(&'static str),
}

impl Translation {
  const fn string(index: u16) -> Self {
    unsafe {
      let ptr = CONFUSABLES.offset(read_u16_le(CONFUSABLES.offset(2)) as isize + (index as isize));

      Self::String(str::from_utf8_unchecked(slice::from_raw_parts(
        ptr.offset(1),
        (*ptr) as _,
      )))
    }
  }

  const fn character(code: u32) -> Self {
    // TODO: Self::Character(unsafe { char::from_u32_unchecked(code) }) not stable yet as a const fn

    Self::Character(unsafe { transmute(code) })
  }
}

impl Confusable {
  pub(crate) const fn at(index: u16) -> Self {
    unsafe {
      Self(
        read_u32_le(CONFUSABLES.offset(4 + (index * 5) as isize)),
        *CONFUSABLES.offset(8 + (index * 5) as isize),
      )
    }
  }

  pub(crate) const fn matches(&self, other: u32, other_lowercased: u32) -> Ordering {
    let other = if self.case_sensitive() {
      other
    } else {
      other_lowercased
    };

    let conf: u32 = self.0 & 0x1fffff;

    if (self.0 & 0x10000000) != 0 {
      if other < conf {
        Ordering::Less
      } else if other > (conf + ((self.1 & 0x7f) as u32)) {
        Ordering::Greater
      } else {
        Ordering::Equal
      }
    } else if other < conf {
      Ordering::Less
    } else if other > conf {
      Ordering::Greater
    } else {
      Ordering::Equal
    }
  }

  pub(crate) const fn translation(&self, other: u32, other_lowercased: u32) -> Translation {
    let mut code = (self.0 >> 21) & 0x7f;

    if (self.0 & 0x20000000) != 0 {
      Translation::string(code as _)
    } else {
      let other = if self.case_sensitive() {
        other
      } else {
        other_lowercased
      };

      if self.1 >= 0x80 {
        code += other - (self.0 & 0x1fffff);
      }

      Translation::character(code)
    }
  }

  pub(crate) const fn case_sensitive(&self) -> bool {
    (self.0 & 0x40000000) != 0
  }
}
