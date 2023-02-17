use super::{
  translation::Translation,
  util::{read_u16_le, read_u32_le},
};
use core::cmp::Ordering;

pub(crate) const CONFUSABLES: *const u8 = include_bytes!("../bin/confusables.bin").as_ptr();

const CASE_SENSITIVE_CONFUSABLES_OFFSET: u16 = read_u16_le(CONFUSABLES);
pub(crate) const CONFUSABLES_COUNT: u16 = ((CASE_SENSITIVE_CONFUSABLES_OFFSET - 6) / 5) - 1;
pub(crate) const CASE_SENSITIVE_CONFUSABLES_COUNT: u16 =
  ((read_u16_le(unsafe { CONFUSABLES.offset(2) }) - CASE_SENSITIVE_CONFUSABLES_OFFSET) / 5) - 1;

pub(crate) struct Confusable(u32, u8);

impl Confusable {
  pub(crate) const fn at(index: u16) -> Self {
    unsafe {
      Self(
        read_u32_le(CONFUSABLES.offset(6 + (index * 5) as isize)),
        *CONFUSABLES.offset(10 + (index * 5) as isize),
      )
    }
  }

  pub(crate) const fn case_sensitive_at(index: u16) -> Self {
    unsafe {
      Self(
        read_u32_le(CONFUSABLES.offset((CASE_SENSITIVE_CONFUSABLES_OFFSET + (index * 5)) as _)),
        *CONFUSABLES.offset((CASE_SENSITIVE_CONFUSABLES_OFFSET + 4 + (index * 5)) as _),
      )
    }
  }

  pub(crate) const fn matches(&self, other: u32) -> Ordering {
    let conf: u32 = self.0 & 0x1fffff;

    if (self.0 & 0x20000000) != 0 {
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

  pub(crate) const fn translation(&self, other: u32) -> Translation {
    if (self.0 & 0x40000000) != 0 {
      Translation::string(self.0, self.1)
    } else {
      let mut code = (self.0 >> 21) & 0xff;

      if self.1 >= 0x80 {
        code += other - (self.0 & 0x1fffff);
      }

      Translation::character(code)
    }
  }
}
