#![allow(clippy::missing_safety_doc)]

mod ptr;
mod utf16;
mod utf8;

use paste::paste;
use std::{
  borrow::Cow,
  convert::AsRef,
  mem::{size_of, transmute},
  ops::{Deref, Range},
  ptr::copy_nonoverlapping,
  str,
};

#[repr(C)]
pub struct Error {
  message: usize,
  message_length: u8,
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

#[no_mangle]
pub unsafe extern "C" fn decancer_cure(
  input_str: *const u8,
  input_length: usize,
  options: u32,
  error: *mut Error,
) -> *mut decancer::CuredString {
  let Some(input) = utf8::get(input_str, input_length) else {
    (*error).message = INVALID_UTF8_MESSAGE.as_ptr() as _;
    (*error).message_length = INVALID_UTF8_MESSAGE.len() as _;

    return 0 as _;
  };

  match decancer::cure(input, transmute(options)) {
    Ok(res) => Box::into_raw(Box::new(res)),
    Err(err) => {
      let message = <decancer::Error as AsRef<str>>::as_ref(&err);

      (*error).message = message.as_ptr() as _;
      (*error).message_length = message.len() as _;

      0 as _
    },
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure_wide(
  input_str: *const u16,
  input_length: usize,
  options: u32,
  error: *mut Error,
) -> *mut decancer::CuredString {
  let Some(input) = utf16::get(input_str, input_length) else {
    (*error).message = INVALID_UTF16_MESSAGE.as_ptr() as _;
    (*error).message_length = INVALID_UTF16_MESSAGE.len() as _;

    return 0 as _;
  };

  let input_str = str::from_utf8_unchecked(&input);

  match decancer::cure(input_str, transmute(options)) {
    Ok(res) => Box::into_raw(Box::new(res)),
    Err(err) => {
      let message = <decancer::Error as AsRef<str>>::as_ref(&err);

      (*error).message = message.as_ptr() as _;
      (*error).message_length = message.len() as _;

      0 as _
    },
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure_char(input: u32, options: u32, output: *mut Translation) {
  match decancer::cure_char(input, transmute(options)) {
    decancer::Translation::Character(c) => {
      (*output).kind = 0;
      (*output).slot_a = c as _;
    },

    decancer::Translation::String(s) => {
      (*output).kind = 1;
      (*output).slot_b = s.len();

      match s {
        Cow::Borrowed(_) => {
          (*output).slot_a = s.as_ptr() as _;
          (*output).slot_c = 0 as _;
        },

        Cow::Owned(s) => {
          let s = Box::new(s);

          (*output).slot_a = s.deref().as_ptr() as _;
          (*output).slot_c = Box::into_raw(Box::new(s)).cast::<u8>() as _;
        },
      }
    },

    decancer::Translation::None => {
      (*output).kind = 2;
    },
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_find(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> *mut decancer::Matcher<'static, 'static> {
  match utf8::get(other_str, other_length) {
    Some(result) => Box::into_raw(Box::new(transmute((*cured).find(result)))),
    None => 0 as _,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_find_wide(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
) -> *mut decancer::Matcher<'static, 'static> {
  match utf16::get(other_str, other_length) {
    Some(result) => Box::into_raw(Box::new(transmute(
      (*cured).find(str::from_utf8_unchecked(&result)),
    ))),
    None => 0 as _,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_find_multiple(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> *mut Vec<Range<usize>> {
  match utf8::get_array(other_str.cast(), other_length) {
    Some(result) => Box::into_raw(Box::new((*cured).find_multiple(result))),
    None => 0 as _,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_find_multiple_wide(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> *mut Vec<Range<usize>> {
  match utf16::get_array(other_str.cast(), other_length) {
    Some(result) => Box::into_raw(Box::new((*cured).find_multiple(result))),
    None => 0 as _,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matches_raw(
  matches: *mut Vec<Range<usize>>,
  output_length: *mut usize,
) -> *const Range<usize> {
  *output_length = (*matches).len();
  (*matches).as_ptr()
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
    },

    None => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_censor(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
  with_char: u32,
) -> bool {
  match (utf8::get(other_str, other_length), char::from_u32(with_char)) {
    (Some(other_str), Some(with_char)) => {
      (*cured).censor(other_str, with_char);
      true
    },

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_censor_wide(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
  with_char: u32,
) -> bool {
  match (utf16::get(other_str, other_length), char::from_u32(with_char)) {
    (Some(other_str), Some(with_char)) => {
      (*cured).censor(str::from_utf8_unchecked(&other_str), with_char);
      true
    },

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_censor_multiple(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
  with_char: u32,
) -> bool {
  match (
    utf8::get_array(other_str.cast(), other_length),
    char::from_u32(with_char),
  ) {
    (Some(result), Some(with_char)) => {
      (*cured).censor_multiple(result, with_char);
      true
    },

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_censor_multiple_wide(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
  with_char: u32,
) -> bool {
  match (
    utf16::get_array(other_str.cast(), other_length),
    char::from_u32(with_char),
  ) {
    (Some(result), Some(with_char)) => {
      (*cured).censor_multiple(result, with_char);
      true
    },

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_replace(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
  with_str: *const u8,
  with_length: usize,
) -> bool {
  match (
    utf8::get(other_str, other_length),
    utf8::get(with_str, with_length),
  ) {
    (Some(other_str), Some(with_str)) => {
      (*cured).replace(other_str, with_str);
      true
    },

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_replace_wide(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
  with_str: *const u16,
  with_length: usize,
) -> bool {
  match (
    utf16::get(other_str, other_length),
    utf16::get(with_str, with_length),
  ) {
    (Some(other_str), Some(with_str)) => {
      (*cured).replace(
        str::from_utf8_unchecked(&other_str),
        str::from_utf8_unchecked(&with_str),
      );
      true
    },

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_replace_multiple(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
  with_str: *const u8,
  with_length: usize,
) -> bool {
  match (
    utf8::get_array(other_str.cast(), other_length),
    utf8::get(with_str, with_length),
  ) {
    (Some(result), Some(with_str)) => {
      (*cured).replace_multiple(result, with_str);
      true
    },

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_replace_multiple_wide(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
  with_str: *const u16,
  with_length: usize,
) -> bool {
  match (
    utf16::get_array(other_str.cast(), other_length),
    utf16::get(with_str, with_length),
  ) {
    (Some(result), Some(with_str)) => {
      (*cured).replace_multiple(result, str::from_utf8_unchecked(&with_str));
      true
    },

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_equals(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> bool {
  utf8::get(other_str, other_length).is_some_and(|s| (*cured) == s)
}

#[no_mangle]
pub unsafe extern "C" fn decancer_equals_wide(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
) -> bool {
  utf16::get(other_str, other_length)
    .is_some_and(|vec| unsafe { (*cured) == str::from_utf8(&vec).unwrap() })
}

macro_rules! comparison_fn {
  ($($name:ident,)*) => {$(
    paste! {
      #[no_mangle]
      pub unsafe extern "C" fn [<decancer_ $name>](
        cured: *mut decancer::CuredString,
        other_str: *const u8,
        other_length: usize,
      ) -> bool {
        utf8::get(other_str, other_length)
          .map(|s| (*cured).$name(s))
          .unwrap_or_default()
      }

      #[no_mangle]
      pub unsafe extern "C" fn [<decancer_ $name _wide>](
        cured: *mut decancer::CuredString,
        other_str: *const u16,
        other_length: usize,
      ) -> bool {
        utf16::get(other_str, other_length)
          .map(|vec| unsafe { (*cured).$name(str::from_utf8(&vec).unwrap()) })
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
pub unsafe extern "C" fn decancer_cured_raw(
  cured: *mut decancer::CuredString,
  output_length: *mut usize,
) -> *const u8 {
  *output_length = (*cured).len();

  (*cured).as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cured_raw_wide(
  cured: *mut decancer::CuredString,
  output_ptr: *mut usize,
  output_length: *mut usize,
) -> *mut Vec<u16> {
  let vec = Box::new((*cured).encode_utf16().collect::<Vec<_>>());

  *output_ptr = vec.as_ptr() as _;
  *output_length = vec.len();

  Box::into_raw(vec)
}

#[no_mangle]
pub unsafe extern "C" fn __decancer_cured_clone(
  cured: *mut decancer::CuredString,
) -> *mut decancer::CuredString {
  Box::into_raw(Box::new((*cured).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn __decancer_translation_clone(
  translation_in: *const Translation,
  translation_out: *mut Translation,
) {
  copy_nonoverlapping(translation_in, translation_out, size_of::<Translation>());

  if (*translation_in).slot_c != 0 {
    (*translation_out).slot_c = Box::into_raw(Box::new(
      (*((*translation_in).slot_c as *mut String)).clone(),
    ))
    .cast::<u8>() as _;
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cured_raw_wide_free(wide: *mut Vec<u16>) {
  let _ = Box::from_raw(wide);
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matcher_free(matcher: *mut decancer::Matcher<'static, 'static>) {
  let _ = Box::from_raw(matcher);
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matches_free(matches: *mut Vec<Range<usize>>) {
  let _ = Box::from_raw(matches);
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
