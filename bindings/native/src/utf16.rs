use crate::ptr::{Element, NullTerminatedPointer, SizedPointer};

fn get_inner(iter: &mut impl Iterator<Item = u16>) -> Option<Vec<u8>> {
  let mut output: Vec<u8> = Vec::new();
  let mut next: Option<u16> = None;

  loop {
    let c = match next.take() {
      Some(res) => res,
      None => match iter.next() {
        Some(res) => res,
        None => return Some(output),
      },
    };

    if c <= 0x7f {
      output.push(c as _);
    } else if c <= 0x7ff {
      output.extend([((c >> 6) as u8) | 0xc0, ((c & 0x3f) as u8) | 0x80]);
    } else if c < 0xd800 || c >= 0xe000 {
      output.extend([
        ((c >> 12) as u8) | 0xe0,
        (((c >> 6) & 0x3f) as u8) | 0x80,
        ((c & 0x3f) as u8) | 0x80,
      ]);
    } else {
      let n = iter.next()?;

      if n >= 0xdc00 && n < 0xe000 {
        let c = 0x10000 + (((c - 0xd800) as u32) << 10) + ((n as u32) - 0xdc00);

        output.extend([
          ((c >> 18) as u8) | 0xf0,
          (((c >> 12) & 0x3f) as u8) | 0x80,
          (((c >> 6) & 0x3f) as u8) | 0x80,
          ((c & 0x3f) as u8) | 0x80,
        ]);
      } else {
        next.replace(n);
      }
    }
  }
}

pub(crate) unsafe fn get(input_ptr: *mut u16, input_size: usize) -> Option<Vec<u8>> {
  if input_size == 0 {
    let mut input_ptr = NullTerminatedPointer::new(input_ptr);

    get_inner(&mut input_ptr)
  } else {
    let mut input_ptr = SizedPointer::new(input_ptr, input_size);

    get_inner(&mut input_ptr)
  }
}

pub(crate) unsafe fn get_array(
  input_ptr: *mut Element<u16>,
  input_length: usize,
) -> Option<Vec<String>> {
  let mut output = Vec::with_capacity(input_length);

  for i in 0..input_length {
    output.push(unsafe {
      let s = input_ptr.offset(i as _);

      String::from_utf8_unchecked(get((*s).string, (*s).size)?)
    });
  }

  Some(output)
}
