// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use std::{
  borrow::Cow,
  convert::AsRef,
  mem::transmute,
  ops::{Deref, Range},
  ptr::copy_nonoverlapping,
  str,
};

#[cfg(feature = "utf16")]
mod utf16;
#[cfg(feature = "utf8")]
mod utf8;
mod util;

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
#[cfg(feature = "utf16")]
pub struct MatcherUtf16 {
  other: String,
  matcher: Option<decancer::Matcher<'static, 'static>>,
}

#[cfg(feature = "utf8")]
const INVALID_UTF8_MESSAGE: &str = "Invalid UTF-8 bytes.";

#[cfg(feature = "utf16")]
const INVALID_UTF16_MESSAGE: &str = "Invalid UTF-16 bytes.";

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
pub unsafe extern "C" fn decancer_cure(
  input_str: *const u8,
  input_length: usize,
  options: u32,
  error: *mut Error,
) -> *mut decancer::CuredString {
  let Some(input) = utf8::get(input_str, input_length) else {
    unsafe {
      (*error).message = INVALID_UTF8_MESSAGE.as_ptr() as _;
      (*error).message_length = INVALID_UTF8_MESSAGE.len() as _;
    }

    return 0 as _;
  };

  match decancer::cure(input, unsafe { transmute(options) }) {
    Ok(res) => Box::into_raw(Box::new(res)),

    Err(err) => {
      if !error.is_null() {
        let message = <decancer::Error as AsRef<str>>::as_ref(&err);

        unsafe {
          (*error).message = message.as_ptr() as _;
          (*error).message_length = message.len() as _;
        }
      }

      0 as _
    },
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_cure_utf16(
  input_str: *const u16,
  input_length: usize,
  options: u32,
  error: *mut Error,
) -> *mut decancer::CuredString {
  let Some(input) = utf16::get(input_str, input_length) else {
    unsafe {
      (*error).message = INVALID_UTF16_MESSAGE.as_ptr() as _;
      (*error).message_length = INVALID_UTF16_MESSAGE.len() as _;
    }

    return 0 as _;
  };

  match decancer::cure(&input, unsafe { transmute(options) }) {
    Ok(res) => Box::into_raw(Box::new(res)),

    Err(err) => {
      if !error.is_null() {
        let message = <decancer::Error as AsRef<str>>::as_ref(&err);

        unsafe {
          (*error).message = message.as_ptr() as _;
          (*error).message_length = message.len() as _;
        }
      }

      0 as _
    },
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decancer_translation_init(output: *mut Translation) {
  unsafe {
    (*output).slot_c = 0;
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decancer_cure_char(input: u32, options: u32, output: *mut Translation) {
  unsafe {
    decancer_translation_free(output);
  }

  match decancer::cure_char(input, unsafe { transmute(options) }) {
    decancer::Translation::Character(c) => unsafe {
      (*output).kind = 0;
      (*output).slot_a = c as _;
    },

    decancer::Translation::String(s) => unsafe {
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

    decancer::Translation::None => unsafe {
      (*output).kind = 2;
    },
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
pub unsafe extern "C" fn decancer_find(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> *mut decancer::Matcher<'static, 'static> {
  utf8::get(other_str, other_length).map_or(0 as _, |result| {
    Box::into_raw(Box::new(unsafe { transmute((*cured).find(result)) }))
  })
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_find_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
) -> *mut MatcherUtf16 {
  utf16::get(other_str, other_length).map_or(0 as _, |result| {
    let mut output = Box::new(MatcherUtf16 {
      other: result,
      matcher: None,
    });

    output
      .matcher
      .replace(unsafe { transmute((*cured).find(&output.other)) });

    Box::into_raw(output)
  })
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
pub unsafe extern "C" fn decancer_find_multiple(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> *mut Vec<Range<usize>> {
  unsafe { utf8::get_array(other_str.cast(), other_length) }.map_or(0 as _, |result| {
    Box::into_raw(Box::new(unsafe { (*cured).find_multiple(result) }))
  })
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_find_multiple_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
) -> *mut Vec<Range<usize>> {
  unsafe { utf16::get_array(other_str.cast(), other_length) }.map_or(0 as _, |result| {
    Box::into_raw(Box::new(unsafe { (*cured).find_multiple(result) }))
  })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decancer_matches_raw(
  matches: *mut Vec<Range<usize>>,
  output_length: *mut usize,
) -> *const Range<usize> {
  unsafe {
    *output_length = (*matches).len();
    (*matches).as_ptr()
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
pub unsafe extern "C" fn decancer_matcher_next(
  matcher: *mut decancer::Matcher<'static, 'static>,
  output: *mut Range<usize>,
) -> bool {
  unsafe { (*matcher).next() }
    .inspect(|mat| unsafe { *output = mat.clone() })
    .is_some()
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_matcher_utf16_next(
  matcher: *mut MatcherUtf16,
  output: *mut Range<usize>,
) -> bool {
  unsafe { (*matcher).matcher.as_mut().unwrap_unchecked() }
    .next()
    .map(|mat| unsafe { *output = mat.clone() })
    .is_some()
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
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
      unsafe {
        (*cured).censor(other_str, with_char);
      }

      true
    },

    _ => false,
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
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
      unsafe {
        (*cured).censor(&other_str, with_char);
      }

      true
    },

    _ => false,
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
pub unsafe extern "C" fn decancer_censor_multiple(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
  with_char: u32,
) -> bool {
  match (
    unsafe { utf8::get_array(other_str.cast(), other_length) },
    char::from_u32(with_char),
  ) {
    (Some(result), Some(with_char)) => {
      unsafe {
        (*cured).censor_multiple(result, with_char);
      }

      true
    },

    _ => false,
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_censor_multiple_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
  with_char: u32,
) -> bool {
  match (
    unsafe { utf16::get_array(other_str.cast(), other_length) },
    char::from_u32(with_char),
  ) {
    (Some(result), Some(with_char)) => {
      unsafe {
        (*cured).censor_multiple(result, with_char);
      }

      true
    },

    _ => false,
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
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
      unsafe {
        (*cured).replace(other_str, with_str);
      }

      true
    },

    _ => false,
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
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
      unsafe {
        (*cured).replace(&other_str, &with_str);
      }

      true
    },

    _ => false,
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
pub unsafe extern "C" fn decancer_replace_multiple(
  cured: *mut decancer::CuredString,
  other_str: *const u8,
  other_length: usize,
  with_str: *const u8,
  with_length: usize,
) -> bool {
  match (
    unsafe { utf8::get_array(other_str.cast(), other_length) },
    utf8::get(with_str, with_length),
  ) {
    (Some(result), Some(with_str)) => {
      unsafe {
        (*cured).replace_multiple(result, with_str);
      }

      true
    },

    _ => false,
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_replace_multiple_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
  with_str: *const u16,
  with_length: usize,
) -> bool {
  match unsafe {
    (
      utf16::get_array(other_str.cast(), other_length),
      utf16::get(with_str, with_length),
    )
  } {
    (Some(result), Some(with_str)) => {
      unsafe {
        (*cured).replace_multiple(result, &with_str);
      }

      true
    },

    _ => false,
  }
}

util::native_comparison_methods! {
  (decancer_equals | decancer_equals_utf16) => |cured, s| (*cured) == s,

  (decancer_starts_with | decancer_starts_with_utf16) => |cured, s| (*cured).starts_with(s),

  (decancer_ends_with | decancer_ends_with_utf16) => |cured, s| (*cured).ends_with(s),

  (decancer_contains | decancer_contains_utf16) => |cured, s| (*cured).contains(s)
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
pub unsafe extern "C" fn decancer_cured_raw(
  cured: *mut decancer::CuredString,
  mat: *const Range<usize>,
  output_size: *mut usize,
) -> *const u8 {
  let cured_ref = unsafe { &(*cured) };
  let ptr = cured_ref.as_ptr();

  unsafe {
    if mat.is_null() {
      *output_size = cured_ref.len();

      ptr
    } else {
      *output_size = (*mat).len();

      ptr.add((*mat).start)
    }
  }
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_cured_raw_utf16(
  cured: *mut decancer::CuredString,
  mat: *const Range<usize>,
  output_ptr: *mut usize,
  output_length: *mut usize,
) -> *mut Vec<u16> {
  let s = unsafe {
    if mat.is_null() {
      (*cured).as_str()
    } else {
      (*cured).get_unchecked((*mat).clone())
    }
  };

  let vec = Box::new(s.encode_utf16().collect::<Vec<_>>());

  unsafe {
    *output_ptr = vec.as_ptr() as _;
    *output_length = vec.len();
  }

  Box::into_raw(vec)
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_cured_raw_utf16_clone(
  raw_utf16_handle: *mut Vec<u16>,
) -> *mut Vec<u16> {
  Box::into_raw(Box::new(unsafe { (*raw_utf16_handle).clone() }))
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf8")]
pub unsafe extern "C" fn decancer_matcher_consume(
  matcher: *mut decancer::Matcher<'static, 'static>,
) -> *mut Vec<Range<usize>> {
  let output = unsafe { (*matcher).by_ref() }.collect::<Vec<_>>();

  let _ = unsafe { Box::from_raw(matcher) };
  Box::into_raw(Box::new(output))
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_matcher_utf16_consume(
  matcher: *mut MatcherUtf16,
) -> *mut Vec<Range<usize>> {
  let output = unsafe { (*matcher).matcher.as_mut().unwrap_unchecked() }.collect::<Vec<_>>();

  let _ = unsafe { Box::from_raw(matcher) };
  Box::into_raw(Box::new(output))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decancer_matches_clone(
  matches: *mut Vec<Range<usize>>,
) -> *mut Vec<Range<usize>> {
  Box::into_raw(Box::new(unsafe { (*matches).clone() }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decancer_translation_clone(
  translation_in: *const Translation,
  translation_out: *mut Translation,
) {
  unsafe {
    copy_nonoverlapping(translation_in, translation_out, 1);

    if (*translation_in).slot_c != 0 {
      (*translation_out).slot_c = Box::into_raw(Box::new(
        (*((*translation_in).slot_c as *mut String)).clone(),
      ))
      .cast::<u8>() as _;
    }
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decancer_translation_free(translation: *mut Translation) {
  unsafe {
    if (*translation).kind == 1 && (*translation).slot_c != 0 {
      let _ = Box::from_raw((*translation).slot_c as *mut String);
    }
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decancer_cured_clone(
  cured: *mut decancer::CuredString,
) -> *mut decancer::CuredString {
  Box::into_raw(Box::new(unsafe { (*cured).clone() }))
}

util::native_free_box_functions! {
  decancer_matcher_free: decancer::Matcher<'static, 'static>,

  decancer_cured_raw_utf16_free: Vec<u16>,

  decancer_matcher_utf16_free: MatcherUtf16,

  decancer_matches_free: Vec<Range<usize>>,

  decancer_cured_free: decancer::CuredString
}
