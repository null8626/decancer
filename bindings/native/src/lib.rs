extern crate decancer;

use core::{ffi::c_void, ptr, str};

const unsafe fn str_from_ptr(input_ptr: *mut u8, input_size: usize) -> &'static str {
  str::from_utf8_unchecked(&*ptr::slice_from_raw_parts(input_ptr, input_size))
}

#[inline(always)]
unsafe fn wstr_from_ptr(input_ptr: *mut u16, input_size: usize) -> String {
  String::from_utf16_lossy(&*ptr::slice_from_raw_parts(input_ptr, input_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure(input_str: *mut u8, input_size: usize) -> *mut c_void {
  println!("'{}' {}", str_from_ptr(
    input_str, input_size,
  ), input_size);

  Box::into_raw(Box::new(decancer::cure(str_from_ptr(
    input_str, input_size,
  )))) as _
}

#[no_mangle]
pub unsafe extern "C" fn decancer_equals(
  cured: *mut c_void,
  other_str: *mut u8,
  other_size: usize,
) -> bool {
  println!("'{}' {}", str_from_ptr(
    other_str, other_size,
  ), other_size);

  *(cured as *mut decancer::CuredString) == str_from_ptr(other_str, other_size)
}

#[no_mangle]
pub unsafe extern "C" fn decancer_contains(
  cured: *mut c_void,
  other_str: *mut u8,
  other_size: usize,
) -> bool {
  (*(cured as *mut decancer::CuredString)).contains(str_from_ptr(other_str, other_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_starts_with(
  cured: *mut c_void,
  other_str: *mut u8,
  other_size: usize,
) -> bool {
  (*(cured as *mut decancer::CuredString)).starts_with(str_from_ptr(other_str, other_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_ends_with(
  cured: *mut c_void,
  other_str: *mut u8,
  other_size: usize,
) -> bool {
  (*(cured as *mut decancer::CuredString)).ends_with(str_from_ptr(other_str, other_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_raw(cured: *mut c_void, output_size: *mut usize) -> *const u8 {
  let cured = cured as *mut decancer::CuredString;
  *output_size = (*cured).len();

  (*cured).as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn decancer_free(cured: *mut c_void) {
  let _ = Box::from_raw(cured as *mut decancer::CuredString);
}

#[no_mangle]
pub unsafe extern "C" fn wdecancer_cure(input_str: *mut u16, input_size: usize) -> *mut c_void {
  let s = wstr_from_ptr(input_str, input_size);

  Box::into_raw(Box::new(decancer::cure(&s))) as _
}

#[no_mangle]
pub unsafe extern "C" fn wdecancer_equals(
  cured: *mut c_void,
  other_str: *mut u16,
  other_size: usize,
) -> bool {
  let s = wstr_from_ptr(other_str, other_size);

  *(cured as *mut decancer::CuredString) == s
}

#[no_mangle]
pub unsafe extern "C" fn wdecancer_contains(
  cured: *mut c_void,
  other_str: *mut u16,
  other_size: usize,
) -> bool {
  let s = wstr_from_ptr(other_str, other_size);

  (*(cured as *mut decancer::CuredString)).contains(&s)
}

#[no_mangle]
pub unsafe extern "C" fn wdecancer_starts_with(
  cured: *mut c_void,
  other_str: *mut u16,
  other_size: usize,
) -> bool {
  let s = wstr_from_ptr(other_str, other_size);

  (*(cured as *mut decancer::CuredString)).starts_with(&s)
}

#[no_mangle]
pub unsafe extern "C" fn wdecancer_ends_with(
  cured: *mut c_void,
  other_str: *mut u16,
  other_size: usize,
) -> bool {
  let s = wstr_from_ptr(other_str, other_size);

  (*(cured as *mut decancer::CuredString)).ends_with(&s)
}

#[no_mangle]
pub unsafe extern "C" fn wdecancer_raw(cured: *mut c_void, output_size: *mut usize) -> *mut c_void {
  let cured = cured as *mut decancer::CuredString;
  let vec: Vec<_> = (*cured).encode_utf16().collect();
  *output_size = vec.len();

  Box::into_raw(Box::new(vec)) as _
}

#[no_mangle]
pub unsafe extern "C" fn wdecancer_raw_ptr(cured_raw: *mut c_void) -> *const u16 {
  let cured_raw = cured_raw as *mut Vec<u16>;

  (*cured_raw).as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn wdecancer_raw_free(cured_raw: *mut c_void) {
  let _ = Box::from_raw(cured_raw as *mut Vec<u16>);
}
