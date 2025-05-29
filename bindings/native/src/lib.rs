mod ptr;
mod utf16;
mod utf8;

use std::{
  borrow::Cow,
  convert::AsRef,
  mem::transmute,
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

#[repr(C)]
pub struct MatcherUtf16 {
  other: Vec<u8>,
  matcher: Option<decancer::Matcher<'static, 'static>>,
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
      if !error.is_null() {
        let message = <decancer::Error as AsRef<str>>::as_ref(&err);

        (*error).message = message.as_ptr() as _;
        (*error).message_length = message.len() as _;
      }

      0 as _
    },
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure_utf16(
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
      if !error.is_null() {
        let message = <decancer::Error as AsRef<str>>::as_ref(&err);

        (*error).message = message.as_ptr() as _;
        (*error).message_length = message.len() as _;
      }

      0 as _
    },
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_translation_init(output: *mut Translation) {
  (*output).slot_c = 0;
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure_char(input: u32, options: u32, output: *mut Translation) {
  decancer_translation_free(output);

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
pub unsafe extern "C" fn decancer_find_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
) -> *mut MatcherUtf16 {
  match utf16::get(other_str, other_length) {
    Some(result) => {
      let mut output = Box::new(MatcherUtf16 {
        other: result,
        matcher: None,
      });

      output.matcher.replace(transmute(
        (*cured).find(str::from_utf8_unchecked(&output.other)),
      ));

      Box::into_raw(output)
    },
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
pub unsafe extern "C" fn decancer_find_multiple_utf16(
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
pub unsafe extern "C" fn decancer_matcher_utf16_next(
  matcher: *mut MatcherUtf16,
  output: *mut Range<usize>,
) -> bool {
  match (*matcher).matcher.as_mut().unwrap_unchecked().next() {
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
  match (
    utf8::get(other_str, other_length),
    char::from_u32(with_char),
  ) {
    (Some(other_str), Some(with_char)) => {
      (*cured).censor(other_str, with_char);
      true
    },

    _ => false,
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_censor_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
  with_char: u32,
) -> bool {
  match (
    utf16::get(other_str, other_length),
    char::from_u32(with_char),
  ) {
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
pub unsafe extern "C" fn decancer_censor_multiple_utf16(
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
pub unsafe extern "C" fn decancer_replace_utf16(
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
pub unsafe extern "C" fn decancer_replace_multiple_utf16(
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
pub unsafe extern "C" fn decancer_equals_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
) -> bool {
  utf16::get(other_str, other_length)
    .is_some_and(|vec| unsafe { (*cured) == str::from_utf8(&vec).unwrap() })
}

#[no_mangle]
pub unsafe extern "C" fn decancer_starts_with(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> bool {
  utf8::get(other_str, other_length).map_or_else(Default::default, |s| (*cured).starts_with(s))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_starts_with_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
) -> bool {
  utf16::get(other_str, other_length).map_or_else(Default::default, |vec| unsafe {
    (*cured).starts_with(str::from_utf8(&vec).unwrap())
  })
}

#[no_mangle]
pub unsafe extern "C" fn decancer_ends_with(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> bool {
  utf8::get(other_str, other_length).map_or_else(Default::default, |s| (*cured).ends_with(s))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_ends_with_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
) -> bool {
  utf16::get(other_str, other_length).map_or_else(Default::default, |vec| unsafe {
    (*cured).ends_with(str::from_utf8(&vec).unwrap())
  })
}

#[no_mangle]
pub unsafe extern "C" fn decancer_contains(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> bool {
  utf8::get(other_str, other_length).map_or_else(Default::default, |s| (*cured).contains(s))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_contains_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
) -> bool {
  utf16::get(other_str, other_length).map_or_else(Default::default, |vec| unsafe {
    (*cured).contains(str::from_utf8(&vec).unwrap())
  })
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cured_raw(
  cured: *mut decancer::CuredString,
  mat: *const Range<usize>,
  output_size: *mut usize,
) -> *const u8 {
  let ptr = (*cured).as_ptr();

  if mat.is_null() {
    *output_size = (*cured).len();

    ptr
  } else {
    *output_size = (*mat).len();

    ptr.add((*mat).start)
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cured_raw_utf16(
  cured: *mut decancer::CuredString,
  mat: *const Range<usize>,
  output_ptr: *mut usize,
  output_length: *mut usize,
) -> *mut Vec<u16> {
  let s = if mat.is_null() {
    (*cured).as_str()
  } else {
    (*cured).get_unchecked((*mat).clone())
  };

  let vec = Box::new(s.encode_utf16().collect::<Vec<_>>());

  *output_ptr = vec.as_ptr() as _;
  *output_length = vec.len();

  Box::into_raw(vec)
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cured_raw_utf16_clone(
  raw_utf16_handle: *mut Vec<u16>,
) -> *mut Vec<u16> {
  Box::into_raw(Box::new((*raw_utf16_handle).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cured_raw_utf16_free(raw_utf16_handle: *mut Vec<u16>) {
  let _ = Box::from_raw(raw_utf16_handle);
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matcher_consume(
  matcher: *mut decancer::Matcher<'static, 'static>,
) -> *mut Vec<Range<usize>> {
  let output = (*matcher).by_ref().collect::<Vec<_>>();

  let _ = Box::from_raw(matcher);
  Box::into_raw(Box::new(output))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matcher_free(matcher: *mut decancer::Matcher<'static, 'static>) {
  let _ = Box::from_raw(matcher);
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matcher_utf16_consume(
  matcher: *mut MatcherUtf16,
) -> *mut Vec<Range<usize>> {
  let output = (*matcher)
    .matcher
    .as_mut()
    .unwrap_unchecked()
    .collect::<Vec<_>>();

  let _ = Box::from_raw(matcher);
  Box::into_raw(Box::new(output))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matcher_utf16_free(matcher: *mut MatcherUtf16) {
  let _ = Box::from_raw(matcher);
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matches_clone(
  matches: *mut Vec<Range<usize>>,
) -> *mut Vec<Range<usize>> {
  Box::into_raw(Box::new((*matches).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_matches_free(matches: *mut Vec<Range<usize>>) {
  let _ = Box::from_raw(matches);
}

#[no_mangle]
pub unsafe extern "C" fn decancer_translation_clone(
  translation_in: *const Translation,
  translation_out: *mut Translation,
) {
  copy_nonoverlapping(translation_in, translation_out, 1);

  if (*translation_in).slot_c != 0 {
    (*translation_out).slot_c = Box::into_raw(Box::new(
      (*((*translation_in).slot_c as *mut String)).clone(),
    ))
    .cast::<u8>() as _;
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_translation_free(translation: *mut Translation) {
  if (*translation).kind == 1 && (*translation).slot_c != 0 {
    let _ = Box::from_raw((*translation).slot_c as *mut String);
  }
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cured_clone(
  cured: *mut decancer::CuredString,
) -> *mut decancer::CuredString {
  Box::into_raw(Box::new((*cured).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cured_free(cured: *mut decancer::CuredString) {
  let _ = Box::from_raw(cured);
}
