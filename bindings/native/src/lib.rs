#![allow(clippy::missing_safety_doc)]

use paste::paste;
use std::{
  borrow::Cow,
  convert::AsRef,
  mem::{size_of, transmute},
  ops::{Deref, Range},
  slice, str,
};

#[repr(C)]
pub struct Error {
  message: usize,
  message_size: u8,
}

#[repr(C)]
pub struct Translation {
  kind: u8,
  slot_a: usize,
  slot_b: usize,
  slot_c: usize,
}

const INVALID_UTF8_MESSAGE: &str = "Invalid UTF-8 bytes.";
const INVALID_UTF16_MESSAGE: &str = "Invalid UTF-16 bytes.";

struct NullTerminatedPointer<T> {
  ptr: *mut T,
  size: usize,
}

impl<T> NullTerminatedPointer<T> {
  const fn new(ptr: *mut T) -> Self {
    Self { ptr, size: 0 }
  }
}

impl<T> Iterator for NullTerminatedPointer<T>
where
  T: PartialEq<T> + Default + Copy,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let value = unsafe { *self.ptr };

    self.ptr = unsafe { self.ptr.offset(1) };

    if value == Default::default() {
      None
    } else {
      self.size += size_of::<T>();

      Some(value)
    }
  }
}

struct SizedPointer<T> {
  ptr: *mut T,
  size: usize,
}

impl<T> SizedPointer<T> {
  const fn new(ptr: *mut T, size: usize) -> Self {
    Self { ptr, size }
  }
}

impl<T> Iterator for SizedPointer<T>
where
  T: Copy,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.size == 0 {
      return None;
    }

    let value = unsafe { *self.ptr };

    self.ptr = unsafe { self.ptr.offset(1) };
    self.size -= size_of::<T>();

    Some(value)
  }
}

fn str_from_ptr(input_ptr: *mut u8, mut input_size: usize) -> Option<&'static str> {
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

fn utf8_from_wide_ptr_inner(iter: &mut impl Iterator<Item = u16>) -> Option<Vec<u8>> {
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

unsafe fn utf8_from_wide_ptr(input_ptr: *mut u16, input_size: usize) -> Option<Vec<u8>> {
  if input_size == 0 {
    let mut input_ptr = NullTerminatedPointer::new(input_ptr);

    utf8_from_wide_ptr_inner(&mut input_ptr)
  } else {
    let mut input_ptr = SizedPointer::new(input_ptr, input_size);

    utf8_from_wide_ptr_inner(&mut input_ptr)
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure(
  input_str: *mut u8,
  input_size: usize,
  options: u32,
  error: *mut Error,
) -> *mut decancer::CuredString {
  let input = match str_from_ptr(input_str, input_size) {
    Some(result) => result,
    None => {
      (*error).message = INVALID_UTF8_MESSAGE.as_ptr() as _;
      (*error).message_size = INVALID_UTF8_MESSAGE.len() as _;

      return 0 as _;
    }
  };

  match decancer::cure(input, transmute(options)) {
    Ok(res) => Box::into_raw(Box::new(res)),
    Err(err) => {
      let message = <decancer::Error as AsRef<str>>::as_ref(&err);

      (*error).message = message.as_ptr() as _;
      (*error).message_size = message.len() as _;

      0 as _
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure_wide(
  input_str: *mut u16,
  input_size: usize,
  options: u32,
  error: *mut Error,
) -> *mut decancer::CuredString {
  let input = match utf8_from_wide_ptr(input_str, input_size) {
    Some(result) => result,
    None => {
      (*error).message = INVALID_UTF16_MESSAGE.as_ptr() as _;
      (*error).message_size = INVALID_UTF16_MESSAGE.len() as _;

      return 0 as _;
    }
  };

  let input_str = str::from_utf8_unchecked(&input);

  match decancer::cure(input_str, transmute(options)) {
    Ok(res) => Box::into_raw(Box::new(res)),
    Err(err) => {
      let message = <decancer::Error as AsRef<str>>::as_ref(&err);

      (*error).message = message.as_ptr() as _;
      (*error).message_size = message.len() as _;

      0 as _
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure_char(input: u32, options: u32, output: *mut Translation) {
  match decancer::cure_char(input, transmute(options)) {
    decancer::Translation::Character(c) => {
      (*output).kind = 0;
      (*output).slot_a = c as _;
    }

    decancer::Translation::String(s) => {
      (*output).kind = 1;
      (*output).slot_b = s.len();

      match s {
        Cow::Borrowed(_) => {
          (*output).slot_a = s.as_ptr() as _;
          (*output).slot_c = 0 as _;
        }

        Cow::Owned(s) => {
          let s = Box::new(s);

          (*output).slot_a = s.deref().as_ptr() as _;
          (*output).slot_c = Box::into_raw(Box::new(s)) as *mut u8 as _;
        }
      }
    }

    decancer::Translation::None => {
      (*output).kind = 2;
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_find(
  cured: *mut decancer::CuredString,
  other_str: *mut u8,
  other_size: usize,
) -> *mut decancer::Matcher<'static, 'static> {
  match str_from_ptr(other_str, other_size) {
    Some(result) => Box::into_raw(Box::new(transmute((*cured).find(result)))),
    None => 0 as _,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matcher_next(
  matcher: *mut decancer::Matcher<'static, 'static>,
  output: *mut Range<usize>,
) -> bool {
  match (*matcher).next() {
    Some(mat) => {
      *output = mat;
      true
    }

    None => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_censor(
  cured: *mut decancer::CuredString,
  other_str: *mut u8,
  other_size: usize,
  with_char: u32,
) -> bool {
  match (
    str_from_ptr(other_str, other_size),
    char::from_u32(with_char),
  ) {
    (Some(other_str), Some(with_char)) => {
      (*cured).censor(other_str, with_char);
      true
    }

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_censor_wide(
  cured: *mut decancer::CuredString,
  other_str: *mut u16,
  other_size: usize,
  with_char: u32,
) -> bool {
  match (
    utf8_from_wide_ptr(other_str, other_size),
    char::from_u32(with_char),
  ) {
    (Some(other_str), Some(with_char)) => {
      (*cured).censor(str::from_utf8_unchecked(&other_str), with_char);
      true
    }

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_replace(
  cured: *mut decancer::CuredString,
  other_str: *mut u8,
  other_size: usize,
  with_str: *mut u8,
  with_size: usize,
) -> bool {
  match (
    str_from_ptr(other_str, other_size),
    str_from_ptr(with_str, with_size),
  ) {
    (Some(other_str), Some(with_str)) => {
      (*cured).replace(other_str, with_str);
      true
    }

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_replace_wide(
  cured: *mut decancer::CuredString,
  other_str: *mut u16,
  other_size: usize,
  with_str: *mut u16,
  with_size: usize,
) -> bool {
  match (
    utf8_from_wide_ptr(other_str, other_size),
    utf8_from_wide_ptr(with_str, with_size),
  ) {
    (Some(other_str), Some(with_str)) => {
      (*cured).replace(
        str::from_utf8_unchecked(&other_str),
        str::from_utf8_unchecked(&with_str),
      );
      true
    }

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_equals(
  cured: *mut decancer::CuredString,
  other_str: *mut u8,
  other_size: usize,
) -> bool {
  str_from_ptr(other_str, other_size)
    .map(|s| (*cured) == s)
    .unwrap_or_default()
}

#[no_mangle]
pub unsafe extern "C" fn decancer_equals_wide(
  cured: *mut decancer::CuredString,
  other_str: *mut u16,
  other_size: usize,
) -> bool {
  utf8_from_wide_ptr(other_str, other_size)
    .map(|vec| unsafe { (*cured) == str::from_utf8_unchecked(&vec) })
    .unwrap_or_default()
}

macro_rules! comparison_fn {
  ($($name:ident,)*) => {$(
    paste! {
      #[no_mangle]
      pub unsafe extern "C" fn [<decancer_ $name>](
        cured: *mut decancer::CuredString,
        other_str: *mut u8,
        other_size: usize,
      ) -> bool {
        str_from_ptr(other_str, other_size)
          .map(|s| (*cured).$name(s))
          .unwrap_or_default()
      }

      #[no_mangle]
      pub unsafe extern "C" fn [<decancer_ $name _wide>](
        cured: *mut decancer::CuredString,
        other_str: *mut u16,
        other_size: usize,
      ) -> bool {
        utf8_from_wide_ptr(other_str, other_size)
          .map(|vec| unsafe { (*cured).$name(str::from_utf8_unchecked(&vec)) })
          .unwrap_or_default()
      }
    }
  )*};
}

comparison_fn! {
  starts_with,
  ends_with,
  contains,
}

#[no_mangle]
pub unsafe extern "C" fn decancer_raw(
  cured: *mut decancer::CuredString,
  output_size: *mut usize,
) -> usize {
  *output_size = (*cured).len();

  (*cured).as_ptr() as _
}

#[no_mangle]
pub unsafe extern "C" fn decancer_raw_wide(
  cured: *mut decancer::CuredString,
  output_ptr: *mut usize,
  output_size: *mut usize,
) -> *mut Vec<u16> {
  let vec = Box::new((*cured).encode_utf16().collect::<Vec<_>>());

  *output_ptr = vec.as_ptr() as _;
  *output_size = vec.len() * size_of::<u16>();

  Box::into_raw(vec)
}

#[no_mangle]
pub unsafe extern "C" fn decancer_raw_wide_free(wide: *mut Vec<u16>) {
  let _ = Box::from_raw(wide);
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matcher_free(matcher: *mut decancer::Matcher<'static, 'static>) {
  let _ = Box::from_raw(matcher);
}

#[no_mangle]
pub unsafe extern "C" fn decancer_translation_free(translation: *mut Translation) {
  if (*translation).kind == 1 && (*translation).slot_c != 0 {
    let _ = Box::from_raw((*translation).slot_c as *mut String);
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cured_free(cured: *mut decancer::CuredString) {
  let _ = Box::from_raw(cured);
}
