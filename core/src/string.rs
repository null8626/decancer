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

  pub const fn into_str(self) -> String {
    unsafe { transmute(self) }
  }

  pub fn starts_with<S: AsRef<str> + ?Sized>(&self, other: &S) -> bool {
    let o = other.as_ref();

    if o.len() > self.len() {
      return false;
    }

    let mut other_iter = o.chars();

    for self_char in self.chars() {
      match other_iter.next() {
        Some(other_char) => {
          if !similar::is(self_char as _, other_char as _) {
            return false;
          }
        }

        None => return true,
      };
    }

    false
  }

  pub fn ends_with<S: AsRef<str> + ?Sized>(&self, other: &S) -> bool {
    let o = other.as_ref();

    if o.len() > self.len() {
      return false;
    }

    let mut other_iter = o.chars().rev();

    for self_char in self.chars().rev() {
      match other_iter.next() {
        Some(other_char) => {
          if !similar::is(self_char as _, other_char as _) {
            return false;
          }
        }

        None => return true,
      };
    }

    false
  }

  pub fn contains<S: AsRef<str> + ?Sized>(&self, other: &S) -> bool {
    let o = other.as_ref();
    if o.len() > self.len() {
      return false;
    }

    let other_chars = o.chars().collect::<Vec<_>>();
    let mut other_index = 0usize;

    for self_char in self.chars() {
      if similar::is(other_chars[other_index] as _, self_char as _) {
        other_index += 1;

        if other_index == other_chars.len() {
          return true;
        }
      } else {
        other_index = 0;
      }
    }

    false
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
      let mut other_iter = o.chars();

      for self_char in self.chars() {
        match other_iter.next() {
          Some(other_char) => {
            if !similar::is(self_char as _, other_char as _) {
              return false;
            }
          }

          None => return false,
        };
      }

      true
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
