// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use std::{borrow::Cow, mem::transmute};

pub fn null_terminated<T>(ptr: *const T) -> impl Iterator<Item = T>
where
  T: Copy + Default + PartialEq<T>,
{
  (0..)
    .map(move |i| unsafe { *ptr.add(i) })
    .take_while(|&x| x != Default::default())
}

#[cfg(feature = "utf16")]
pub fn sized<T>(ptr: *const T, size: usize) -> impl Iterator<Item = T>
where
  T: Copy,
{
  (0..size).map(move |i| unsafe { *ptr.add(i) })
}

macro_rules! write_error {
  ($error:ident, $message:ident) => {
    #[allow(clippy::cast_possible_truncation)]
    unsafe {
      (*$error).message = $message.as_ptr() as _;
      (*$error).message_length = $message.len() as _;
    }
  };
}

pub fn native_cure(
  input: Option<Cow<'static, str>>,
  options: u32,
  invalid_input_error_message: &'static str,
  error: *mut super::Error,
) -> *mut decancer::CuredString {
  let Some(input) = input else {
    write_error!(error, invalid_input_error_message);

    return 0 as _;
  };

  match decancer::cure(&input, unsafe { transmute::<u32, decancer::Options>(options) }) {
    Ok(res) => Box::into_raw(Box::new(res)),

    Err(err) => {
      if !error.is_null() {
        let message = <decancer::Error as AsRef<str>>::as_ref(&err);

        write_error!(error, message);
      }

      0 as _
    },
  }
}

macro_rules! native_cure_functions {
  ($(
    $(#[$additional_meta:meta])*
    $method_name:ident($type:ty, $invalid_input_error_message:ident)
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
        $crate::util::native_cure(<$type>::parse(input_str, input_length), options, $invalid_input_error_message, error)
      }
    )*
  };

  ($utf8_method_name:ident($utf8_invalid_input_error_message:ident), $utf16_method_name:ident($utf16_invalid_input_error_message:ident)) => {
    $crate::util::native_cure_functions! {
      #[cfg(feature = "utf8")]
      $utf8_method_name(u8, $utf8_invalid_input_error_message),

      #[cfg(feature = "utf16")]
      $utf16_method_name(u16, $utf16_invalid_input_error_message)
    }
  }
}

pub(super) use native_cure_functions;

macro_rules! native_methods {
  ($(
    $(#[$additional_meta:meta])*
    $exported_method_name:ident => $used_method_name:ident<$type:ty>(
      $(array($other_array:ident))?
      $(string($other_str:ident))?
      $(,
        $(char($with_char:ident))?
        $(string($with_str:ident))?
      )?
    ) -> $return_type:ty
  ),*) => {
    $(
      #[unsafe(no_mangle)]
      $(#[$additional_meta])*
      pub unsafe extern "C" fn $exported_method_name(
        cured: *mut decancer::CuredString,
        $($other_array: *const u8)?
        $($other_str: *const $type)?,
        other_length: usize
        $(,
          $($with_char: u32)?
          $(
            $with_str: *const $type,
            with_length: usize
          )?
        )?
      ) -> $return_type {
        $used_method_name(
          cured,
          $(<$type>::parse($other_str, other_length))?
          $(<$type>::parse_array($other_array, other_length))?
          $(,
            $($with_char)?
            $(<$type>::parse($with_str, with_length))?
          )?
        )
      }
    )*
  };

  ($(
    dual($exported_utf8_method_name:ident | $exported_utf16_method_name:ident) => $used_method_name:ident(
      $(array($other_array:ident))?
      $(string($other_str:ident))?
      $(,
        $(char($with_char:ident))?
        $(string($with_str:ident))?
      )?
    ) $(-> compare($compare_set:literal))?
  ),*) => {
    $(
      #[allow(unused_variables)]
      fn $used_method_name(
        cured: *mut decancer::CuredString,
        $($other_array: Option<Vec<Cow<'static, str>>>)?
        $($other_str: Option<Cow<'static, str>>)?
        $(,
          $($with_char: u32)?
          $($with_str: Option<Cow<'static, str>>)?
        )?
      ) -> bool {
        match ($($other_array)?$($other_str)?, $($(char::from_u32($with_char))?$($with_str)?)?) {
          (Some($($other_array)?$($other_str)?), $(Some($($with_char)?$($with_str)?))?) => {
            let output = unsafe {
              (*cured).$used_method_name($($other_array)?$(&$other_str)?$(, $($with_char)?$(&$with_str)?)?)
            };

            $(
              if output == !$compare_set {
                return false;
              }
            )?

            true
          },

          _ => false
        }
      }

      $crate::util::native_methods! {
        #[cfg(feature = "utf8")]
        $exported_utf8_method_name => $used_method_name<u8>(
          $(array($other_array))?
          $(string($other_str))?
          $(,
            $(char($with_char))?
            $(string($with_str))?
          )?
        ) -> bool,

        #[cfg(feature = "utf16")]
        $exported_utf16_method_name => $used_method_name<u16>(
          $(array($other_array))?
          $(string($other_str))?
          $(,
            $(char($with_char))?
            $(string($with_str))?
          )?
        ) -> bool
      }
    )*
  }
}

pub(super) use native_methods;

macro_rules! native_find_multiple_methods {
  ($(
    $(#[$additional_meta:meta])*
    $method_name:ident: $type:ty
  ),*) => {
    fn find_multiple(
      cured: *mut decancer::CuredString,
      other: Option<Vec<Cow<'static, str>>>,
    ) -> *mut Vec<Range<usize>> {
      other.map_or(0 as _, |result| {
        Box::into_raw(Box::new(unsafe { (*cured).find_multiple(result) }))
      })
    }

    $crate::util::native_methods! {
      $(
        $(#[$additional_meta])*
        $method_name => find_multiple<$type>(array(other)) -> *mut Vec<Range<usize>>
      ),*
    }
  };

  ($utf8_method_name:ident, $utf16_method_name:ident) => {
    $crate::util::native_find_multiple_methods! {
      #[cfg(feature = "utf8")]
      $utf8_method_name: u8,

      #[cfg(feature = "utf16")]
      $utf16_method_name: u16
    }
  }
}

pub(super) use native_find_multiple_methods;

macro_rules! consume_iterator {
  (|$iterator_ptr:ident| $iterator_ptr_mapper:expr) => {{
    let output = unsafe { $iterator_ptr_mapper }.collect::<Vec<_>>();

    let _ = unsafe { Box::from_raw($iterator_ptr) };
    Box::into_raw(Box::new(output))
  }};
}

pub(super) use consume_iterator;

macro_rules! native_free_box_methods {
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

pub(super) use native_free_box_methods;
