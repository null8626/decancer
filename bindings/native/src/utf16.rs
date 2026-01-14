// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use super::util::{Element, null_terminated, sized};

fn get_inner(iter: impl Iterator<Item = u16>) -> Option<String> {
  let mut output = String::with_capacity(iter.size_hint().0);

  for c in char::decode_utf16(iter) {
    output.push(c.ok()?);
  }

  Some(output)
}

pub(super) fn get(input_ptr: *const u16, input_size: usize) -> Option<String> {
  if input_size == 0 {
    get_inner(null_terminated(input_ptr))
  } else {
    get_inner(sized(input_ptr, input_size))
  }
}

pub(super) unsafe fn get_array(
  input_ptr: *const Element<u16>,
  input_length: usize,
) -> Option<Vec<String>> {
  let mut output = Vec::with_capacity(input_length);

  for i in 0..input_length {
    output.push(unsafe {
      let s = input_ptr.add(i);
      get((*s).string, (*s).size)?
    });
  }

  Some(output)
}
