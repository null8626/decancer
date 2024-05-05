use crate::ptr::{Element, NullTerminatedPointer};
use std::{slice, str};

pub(crate) fn get(input_ptr: *mut u8, mut input_size: usize) -> Option<&'static str> {
  if input_size == 0 {
    let mut input_ptr = NullTerminatedPointer::new(input_ptr);

    while let Some(value) = input_ptr.next() {
      if (0xA0..=0xBF).contains(&value)
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

  str::from_utf8(unsafe { slice::from_raw_parts(input_ptr, input_size) }).ok()
}

pub(crate) unsafe fn get_array(
  input_ptr: *mut Element<u8>,
  input_length: usize,
) -> Option<Vec<&'static str>> {
  let mut output = Vec::with_capacity(input_length);

  for i in 0..input_length {
    output.push(unsafe {
      let s = input_ptr.add(i);

      get((*s).string, (*s).size)?
    });
  }

  Some(output)
}
