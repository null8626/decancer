// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use std::{
  borrow::Cow,
  mem::transmute,
  ops::{Deref, Range},
  ptr::copy_nonoverlapping,
  str,
};

mod unicode;
mod util;

use unicode::UnicodeUnit;

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

util::native_cure_functions!(
  decancer_cure(INVALID_UTF8_MESSAGE),
  decancer_cure_utf16(INVALID_UTF16_MESSAGE)
);

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
  u8::parse(other_str, other_length).map_or(0 as _, |result| {
    Box::into_raw(Box::new(unsafe { transmute((*cured).find(&result)) }))
  })
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_find_utf16(
  cured: *mut decancer::CuredString,
  other_str: *const u16,
  other_length: usize,
) -> *mut MatcherUtf16 {
  u16::parse(other_str, other_length).map_or(0 as _, |result| {
    let mut output = Box::new(MatcherUtf16 {
      other: result.into_owned(),
      matcher: None,
    });

    output
      .matcher
      .replace(unsafe { transmute((*cured).find(&output.other)) });

    Box::into_raw(output)
  })
}

util::native_find_multiple_methods!(decancer_find_multiple, decancer_find_multiple_utf16);

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

util::native_methods! {
  dual(decancer_censor | decancer_censor_utf16) => censor(string(other), char(with)),

  dual(decancer_censor_multiple | decancer_censor_multiple_utf16) => censor_multiple(array(other), char(with)),

  dual(decancer_replace | decancer_replace_utf16) => replace(string(other), string(with)),

  dual(decancer_replace_multiple | decancer_replace_multiple_utf16) => replace_multiple(array(other), string(with)),

  dual(decancer_equals | decancer_equals_utf16) => eq(string(other)) -> compare(true),

  dual(decancer_starts_with | decancer_starts_with_utf16) => starts_with(string(other)) -> compare(true),

  dual(decancer_ends_with | decancer_ends_with_utf16) => ends_with(string(other)) -> compare(true),

  dual(decancer_contains | decancer_contains_utf16) => contains(string(other)) -> compare(true)
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
  util::consume_iterator!(|matcher| (*matcher).by_ref())
}

#[unsafe(no_mangle)]
#[cfg(feature = "utf16")]
pub unsafe extern "C" fn decancer_matcher_utf16_consume(
  matcher: *mut MatcherUtf16,
) -> *mut Vec<Range<usize>> {
  util::consume_iterator!(|matcher| (*matcher).matcher.as_mut().unwrap_unchecked())
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

util::native_free_box_methods! {
  #[cfg(feature = "utf8")]
  decancer_matcher_free: decancer::Matcher<'static, 'static>,

  #[cfg(feature = "utf16")]
  decancer_cured_raw_utf16_free: Vec<u16>,

  #[cfg(feature = "utf16")]
  decancer_matcher_utf16_free: MatcherUtf16,

  decancer_matches_free: Vec<Range<usize>>,

  decancer_cured_free: decancer::CuredString
}
