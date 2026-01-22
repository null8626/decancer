// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use std::{borrow::Cow, mem::transmute, ops::Range};

#[repr(C)]
pub(super) struct Element<T> {
  pub(super) string: *const T,
  pub(super) size: usize,
}

pub(super) fn null_terminated<T>(ptr: *const T) -> impl Iterator<Item = T>
where
  T: Copy + Default + PartialEq<T>,
{
  (0..)
    .map(move |i| unsafe { *ptr.add(i) })
    .take_while(|&x| x != Default::default())
}

#[cfg(feature = "utf16")]
pub(super) fn sized<T>(ptr: *const T, size: usize) -> impl Iterator<Item = T>
where
  T: Copy,
{
  (0..size).map(move |i| unsafe { *ptr.add(i) })
}

macro_rules! native_comparison_methods {
  ($(($method_name_utf8:ident | $method_name_utf16:ident) => |$cured:ident, $string:ident| $process:expr),*) => {
    $(
      #[unsafe(no_mangle)]
      #[cfg(feature = "utf8")]
      pub unsafe extern "C" fn $method_name_utf8($cured: *mut decancer::CuredString, other_str: *const u8, other_length: usize) -> bool {
        utf8::get(other_str, other_length).is_some_and(|$string| {
          let $string = &$string;

          unsafe { $process }
        })
      }

      #[unsafe(no_mangle)]
      #[cfg(feature = "utf16")]
      pub unsafe extern "C" fn $method_name_utf16($cured: *mut decancer::CuredString, other_str: *const u16, other_length: usize) -> bool {
        utf16::get(other_str, other_length).is_some_and(|$string| {
          let $string = &$string;

          unsafe { $process }
        })
      }
    )*
  }
}

pub(super) use native_comparison_methods;

macro_rules! native_free_box_functions {
  ($(
    $(#[$additional_meta:meta])*
    $method_name:ident: $type:ty
  ),*) => {
    $(
      #[unsafe(no_mangle)]
      $(#[$additional_meta])*
      pub unsafe extern "C" fn $method_name(x: *mut $type) {
        let _ = unsafe { Box::from_raw(x) };
      }
    )*
  }
}

pub(super) use native_free_box_functions;

pub(super) fn native_cure(
  input: Option<Cow<'static, str>>,
  options: u32,
  invalid_input_error_message: &'static str,
  error: *mut crate::Error,
) -> *mut decancer::CuredString {
  let Some(input) = input else {
    unsafe {
      (*error).message = invalid_input_error_message.as_ptr() as _;
      (*error).message_length = invalid_input_error_message.len() as _;
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

macro_rules! native_cure_functions {
  ($(
    $(#[$additional_meta:meta])*
    $method_name:ident => ($type:ty, $namespace:ident, $invalid_input_error_message:ident)
  ),*) => {
    $(
      #[unsafe(no_mangle)]
      $(#[$additional_meta])*
      pub unsafe extern "C" fn $method_name(
        input_str: *const $type,
        input_length: usize,
        options: u32,
        error: *mut $crate::Error,
      ) -> *mut decancer::CuredString {
        $crate::util::native_cure($namespace::get(input_str, input_length), options, $invalid_input_error_message, error)
      }
    )*
  }
}

pub(super) use native_cure_functions;

pub(super) fn native_find_multiple(
  cured: *mut decancer::CuredString,
  other: Option<Vec<Cow<'static, str>>>,
) -> *mut Vec<Range<usize>> {
  other.map_or(0 as _, |result| {
    Box::into_raw(Box::new(unsafe { (*cured).find_multiple(result) }))
  })
}

macro_rules! native_find_multiple_functions {
  ($(
    $(#[$additional_meta:meta])*
    $method_name:ident => ($type:ty, $namespace:ident)
  ),*) => {
    $(
      #[unsafe(no_mangle)]
      $(#[$additional_meta])*
      pub unsafe extern "C" fn $method_name(
        cured: *mut decancer::CuredString,
        other_str: *const $type,
        other_length: usize,
      ) -> *mut Vec<Range<usize>> {
        $crate::util::native_find_multiple(cured, unsafe { $namespace::get_array(other_str.cast(), other_length) })
      }
    )*
  }
}

pub(super) use native_find_multiple_functions;

pub(super) fn native_censor(
  cured: *mut decancer::CuredString,
  other: Option<Cow<'static, str>>,
  with_char: u32,
) -> bool {
  match (other, char::from_u32(with_char)) {
    (Some(other_str), Some(with_char)) => {
      unsafe {
        (*cured).censor(&other_str, with_char);
      }

      true
    },

    _ => false,
  }
}

macro_rules! native_censor_functions {
  ($(
    $(#[$additional_meta:meta])*
    $method_name:ident => ($type:ty, $namespace:ident)
  ),*) => {
    $(
      #[unsafe(no_mangle)]
      $(#[$additional_meta])*
      pub unsafe extern "C" fn $method_name(
        cured: *mut decancer::CuredString,
        other_str: *const $type,
        other_length: usize,
        with_char: u32,
      ) -> bool {
        $crate::util::native_censor(cured, $namespace::get(other_str, other_length), with_char)
      }
    )*
  }
}

pub(super) use native_censor_functions;

pub(super) fn native_censor_multiple(
  cured: *mut decancer::CuredString,
  other: Option<Vec<Cow<'static, str>>>,
  with_char: u32,
) -> bool {
  match (other, char::from_u32(with_char)) {
    (Some(result), Some(with_char)) => {
      unsafe {
        (*cured).censor_multiple(result, with_char);
      }

      true
    },

    _ => false,
  }
}

macro_rules! native_censor_multiple_functions {
  ($(
    $(#[$additional_meta:meta])*
    $method_name:ident => $namespace:ident
  ),*) => {
    $(
      #[unsafe(no_mangle)]
      $(#[$additional_meta])*
      pub unsafe extern "C" fn $method_name(
        cured: *mut decancer::CuredString,
        other_str: *const u8,
        other_length: usize,
        with_char: u32,
      ) -> bool {
        $crate::util::native_censor_multiple(cured, unsafe { $namespace::get_array(other_str.cast(), other_length) }, with_char)
      }
    )*
  }
}

pub(super) use native_censor_multiple_functions;
