use super::{matcher::Translation, similar};
use std::{cmp::PartialEq, fmt, mem::transmute, ops::Deref};

pub struct CuredString(String);

fn compute_reversed_utf8(ptr: &mut *const u8) {
  unsafe {
    while **ptr >= 0x80 && **ptr < 0xC0 {
      *ptr = ptr.offset(-1);
    }
  }
}

impl CuredString {
  #[inline(always)]
  pub(crate) fn with_capacity(n: usize) -> Self {
    Self(String::with_capacity(n))
  }

  #[inline(always)]
  pub(crate) fn push_translation(&mut self, t: Translation) {
    match t {
      Translation::Character(c) => self.0.push(c),
      Translation::String(s) => self.0.push_str(s),
    }
  }

  #[inline(always)]
  pub(crate) fn push_code(&mut self, code: u32) {
    self.0.push(unsafe { transmute(code) })
  }

  #[inline(always)]
  pub fn into_str(self) -> String {
    self.0
  }

  pub fn starts_with<S: AsRef<str> + ?Sized>(&self, other: &S) -> bool {
    let o = other.as_ref();

    if self.len() < o.len() || self.is_empty() {
      false
    } else {
      unsafe {
        let mut s_ptr = self.as_ptr();
        let mut o_ptr = o.as_ptr();
        let end = o_ptr.offset(o.len() as _);

        while o_ptr < end {
          let offset = similar::is(s_ptr, o_ptr);

          if offset == 0 {
            return false;
          } else {
            s_ptr = s_ptr.offset(offset as _);
            o_ptr = o_ptr.offset(offset as _);
          }
        }

        true
      }
    }
  }

  pub fn ends_with<S: AsRef<str> + ?Sized>(&self, other: &S) -> bool {
    let o = other.as_ref();

    if self.len() < o.len() || self.is_empty() {
      false
    } else {
      unsafe {
        let mut s_ptr = self.as_ptr().offset((self.len() - 1) as _);
        let mut o_ptr = o.as_ptr().offset((o.len() - 1) as _);

        while o_ptr >= o.as_ptr() {
          compute_reversed_utf8(&mut s_ptr);
          compute_reversed_utf8(&mut o_ptr);

          if similar::is(s_ptr, o_ptr) == 0 {
            return false;
          } else {
            s_ptr = s_ptr.offset(-1);
            o_ptr = o_ptr.offset(-1);
          }
        }

        true
      }
    }
  }
}

impl Into<String> for CuredString {
  #[inline(always)]
  fn into(self) -> String {
    self.into_str()
  }
}

impl AsRef<str> for CuredString {
  #[inline(always)]
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl<S> PartialEq<S> for CuredString
where
  S: AsRef<str> + ?Sized,
{
  fn eq(&self, other: &S) -> bool {
    let o = other.as_ref();

    if self.len() != o.len() {
      false
    } else {
      unsafe {
        let mut s_ptr = self.as_ptr();
        let mut o_ptr = o.as_ptr();
        let end = s_ptr.offset(self.len() as _);

        while s_ptr < end {
          let offset = similar::is(s_ptr, o_ptr);

          if offset == 0 {
            return false;
          } else {
            s_ptr = s_ptr.offset(offset as _);
            o_ptr = o_ptr.offset(offset as _);
          }
        }

        true
      }
    }
  }
}

impl fmt::Debug for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\"{}\"", self.0)
  }
}

impl fmt::Display for CuredString {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Deref for CuredString {
  type Target = String;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
