use core::{ffi::c_void, ptr, str};

const unsafe fn str_from_ptr(input_ptr: *mut u8, input_size: usize) -> &'static str {
  str::from_utf8_unchecked(&*ptr::slice_from_raw_parts(input_ptr, input_size))
}

#[no_mangle]
pub unsafe extern "C" fn decancer_cure(input_str: *mut u8, input_size: usize) -> *mut c_void {
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