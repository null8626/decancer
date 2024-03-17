#![allow(clippy::missing_safety_doc)]

use std::{
  borrow::Cow,
  convert::AsRef,
  mem::transmute,
  ops::{Deref, Range},
  slice, str,
};

#[repr(C)]
pub struct Error {
  message: *const u8,
  message_size: u8,
}

#[repr(C)]
pub struct Translation {
  kind: u8,
  slot_a: usize,
  slot_b: usize,
  slot_c: usize,
}

const unsafe fn str_from_ptr(input_ptr: *mut u8, input_size: usize) -> &'static str {
  str::from_utf8_unchecked(slice::from_raw_parts(input_ptr, input_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure(
  input_str: *mut u8,
  input_size: usize,
  options: u32,
  error: *mut Error,
) -> *mut decancer::CuredString {
  match decancer::cure(str_from_ptr(input_str, input_size), transmute(options)) {
    Ok(res) => Box::into_raw(Box::new(res)),
    Err(err) => {
      let message = <decancer::Error as AsRef<str>>::as_ref(&err);

      (*error).message = message.as_ptr();
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
  Box::into_raw(Box::new(transmute(
    (*cured).find(str_from_ptr(other_str, other_size)),
  )))
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
pub unsafe extern "C" fn decancer_equals(
  cured: *mut decancer::CuredString,
  other_str: *mut u8,
  other_size: usize,
) -> bool {
  (*cured) == str_from_ptr(other_str, other_size)
}

#[no_mangle]
pub unsafe extern "C" fn decancer_contains(
  cured: *mut decancer::CuredString,
  other_str: *mut u8,
  other_size: usize,
) -> bool {
  (*cured).contains(str_from_ptr(other_str, other_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_starts_with(
  cured: *mut decancer::CuredString,
  other_str: *mut u8,
  other_size: usize,
) -> bool {
  (*cured).starts_with(str_from_ptr(other_str, other_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_ends_with(
  cured: *mut decancer::CuredString,
  other_str: *mut u8,
  other_size: usize,
) -> bool {
  (*cured).ends_with(str_from_ptr(other_str, other_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_raw(
  cured: *mut decancer::CuredString,
  output_size: *mut usize,
) -> *const u8 {
  *output_size = (*cured).len();

  (*cured).as_ptr()
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
pub unsafe extern "C" fn decancer_free(cured: *mut decancer::CuredString) {
  let _ = Box::from_raw(cured);
}
