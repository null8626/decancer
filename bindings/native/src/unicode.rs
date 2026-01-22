// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use super::util::null_terminated;
use std::borrow::Cow;

#[cfg(feature = "utf8")]
use std::{slice, str};

#[cfg(feature = "utf16")]
use super::util::sized;

#[repr(C)]
struct Element<T: ?Sized> {
  string: *const T,
  size: usize,
}

pub(super) trait UnicodeUnit {
  fn parse(input_ptr: *const Self, input_size: usize) -> Option<Cow<'static, str>>;

  fn parse_array(input_ptr: *const u8, input_length: usize) -> Option<Vec<Cow<'static, str>>> {
    let input_ptr = input_ptr.cast::<Element<Self>>();
    let mut output = Vec::with_capacity(input_length);

    for i in 0..input_length {
      output.push(unsafe {
        let s = input_ptr.add(i);

        Self::parse((*s).string, (*s).size)?
      });
    }

    Some(output)
  }
}

#[cfg(feature = "utf8")]
impl UnicodeUnit for u8 {
  fn parse(input_ptr: *const Self, mut input_size: usize) -> Option<Cow<'static, str>> {
    if input_size == 0 {
      input_size = null_terminated(input_ptr).count();
    }

    str::from_utf8(unsafe { slice::from_raw_parts(input_ptr, input_size) })
      .ok()
      .map(Cow::Borrowed)
  }
}

#[cfg(feature = "utf16")]
fn u16_parse_inner(iter: impl Iterator<Item = u16>) -> Option<Cow<'static, str>> {
  let mut output = String::with_capacity(iter.size_hint().0);

  for c in char::decode_utf16(iter) {
    output.push(c.ok()?);
  }

  Some(Cow::Owned(output))
}

#[cfg(feature = "utf16")]
impl UnicodeUnit for u16 {
  fn parse(input_ptr: *const Self, input_size: usize) -> Option<Cow<'static, str>> {
    if input_size == 0 {
      u16_parse_inner(null_terminated(input_ptr))
    } else {
      u16_parse_inner(sized(input_ptr, input_size))
    }
  }
}
