// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use super::util::{Element, null_terminated};
use std::{borrow::Cow, slice, str};

pub(super) fn get(input_ptr: *const u8, mut input_size: usize) -> Option<Cow<'static, str>> {
  if input_size == 0 {
    input_size = null_terminated(input_ptr).count();
  }

  str::from_utf8(unsafe { slice::from_raw_parts(input_ptr, input_size) })
    .ok()
    .map(Cow::Borrowed)
}

pub(super) unsafe fn get_array(
  input_ptr: *const Element<u8>,
  input_length: usize,
) -> Option<Vec<Cow<'static, str>>> {
  let mut output = Vec::with_capacity(input_length);

  for i in 0..input_length {
    output.push(unsafe {
      let s = input_ptr.add(i);

      get((*s).string, (*s).size)?
    });
  }

  Some(output)
}
