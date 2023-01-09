extern crate decancer;

use core::ffi::c_void;

#[cfg(feature = "utf8")]
mod utf8;

#[cfg(feature = "utf8")]
pub use utf8::*;

#[cfg(feature = "utf16")]
mod utf16;

#[cfg(feature = "utf16")]
pub use utf16::*;

#[no_mangle]
pub unsafe extern "C" fn decancer_free(cured: *mut c_void) {
  let _ = Box::from_raw(cured as *mut decancer::CuredString);
}
