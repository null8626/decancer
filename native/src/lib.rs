extern crate decancer;

use core::{slice, str};
use std::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn decancer_cure(utf8_string: *const u8, utf8_size: usize) -> *mut c_void {
  Box::into_raw(Box::new(decancer::cure(str::from_utf8_unchecked(
    slice::from_raw_parts(utf8_string, utf8_size),
  )))) as _
}

#[no_mangle]
pub unsafe extern "C" fn decancer_starts_with(
  handle: *mut c_void,
  utf8_string: *const u8,
  utf8_size: usize,
) -> bool {
  (*(handle as *mut decancer::CuredString)).starts_with(str::from_utf8_unchecked(
    slice::from_raw_parts(utf8_string, utf8_size),
  ))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_ends_with(
  handle: *mut c_void,
  utf8_string: *const u8,
  utf8_size: usize,
) -> bool {
  (*(handle as *mut decancer::CuredString)).ends_with(str::from_utf8_unchecked(
    slice::from_raw_parts(utf8_string, utf8_size),
  ))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_contains(
  handle: *mut c_void,
  utf8_string: *const u8,
  utf8_size: usize,
) -> bool {
  (*(handle as *mut decancer::CuredString)).contains(str::from_utf8_unchecked(
    slice::from_raw_parts(utf8_string, utf8_size),
  ))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_equals(
  handle: *mut c_void,
  utf8_string: *const u8,
  utf8_size: usize,
) -> bool {
  (*(handle as *mut decancer::CuredString))
    == str::from_utf8_unchecked(slice::from_raw_parts(utf8_string, utf8_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_retrieve_string(
  handle: *mut c_void,
  utf8_string: *mut *const u8,
  utf8_size: *mut usize,
) {
  *utf8_string = (*(handle as *mut String)).as_ptr();
  *utf8_size = (*(handle as *mut String)).len();
}

#[no_mangle]
pub unsafe extern "C" fn decancer_free(handle: *mut c_void) {
  let _ = Box::from_raw(handle as *mut decancer::CuredString);
}
