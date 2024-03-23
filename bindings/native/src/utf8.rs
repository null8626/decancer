use crate::ptr::NullTerminatedPointer;
use std::{ffi::c_void, slice, str};

pub(crate) fn get(input_ptr: *mut u8, mut input_size: usize) -> Option<&'static str> {
  if input_size == 0 {
    let mut input_ptr = NullTerminatedPointer::new(input_ptr);

    while let Some(value) = input_ptr.next() {
      if (value >= 0xA0 && value <= 0xBF)
        || value >= 0xF8
        || (value >= 0xC0
          && ((input_ptr.next()? >> 6) != 0x02
            || (value >= 0xE0
              && ((input_ptr.next()? >> 6) != 0x02
                || (value >= 0xF0 && (input_ptr.next()? >> 6) != 0x02)))))
      {
        return None;
      }
    }

    input_size = input_ptr.size;
  }

  unsafe {
    Some(str::from_utf8_unchecked(slice::from_raw_parts(
      input_ptr, input_size,
    )))
  }
}

#[repr(C)]
struct ArrayElement {
  string: *mut u8,
  size: usize,
}

pub(crate) unsafe fn get_array(
  input_ptr: *mut c_void,
  input_length: usize,
) -> Option<Vec<&'static str>> {
  let mut output = Vec::with_capacity(input_length);
  let input_ptr = input_ptr as *mut ArrayElement;

  for i in 0..input_length {
    output.push(unsafe {
      let s = input_ptr.offset(i as _);

      get((*s).string, (*s).size)?
    });
  }

  Some(output)
}
