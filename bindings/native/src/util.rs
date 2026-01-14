// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

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
        utf8::get(other_str, other_length).is_some_and(|$string| unsafe { $process })
      }

      #[unsafe(no_mangle)]
      #[cfg(feature = "utf16")]
      pub unsafe extern "C" fn $method_name_utf16($cured: *mut decancer::CuredString, other_str: *const u16, other_length: usize) -> bool {
        utf16::get(other_str, other_length).is_some_and(|$string| {
          let $string = $string.as_str();

          unsafe { $process }
        })
      }
    )*
  }
}

pub(super) use native_comparison_methods;

macro_rules! native_free_box_functions {
  ($($method_name:ident: $type:ty),*) => {
    $(
      #[unsafe(no_mangle)]
      pub unsafe extern "C" fn $method_name(x: *mut $type) {
        let _ = unsafe { Box::from_raw(x) };
      }
    )*
  }
}

pub(super) use native_free_box_functions;
