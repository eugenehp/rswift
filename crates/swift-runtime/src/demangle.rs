//! Swift symbol demangling.

use core::ffi::c_void;
use std::ffi::CStr;

/// Demangle a Swift symbol name.
///
/// Accepts mangled names like `$sSiN` or `_$sSiN`.
/// Returns `None` if demangling fails.
pub fn demangle(mangled: &CStr) -> Option<String> {
    let result = unsafe {
        swift_runtime_sys::DebugHooks::swift_demangle(
            mangled.as_ptr(),
            mangled.to_bytes().len(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
        )
    };
    if result.is_null() {
        return None;
    }
    let s = unsafe { CStr::from_ptr(result) }
        .to_str()
        .ok()?
        .to_string();
    unsafe { libc::free(result as *mut c_void) };
    Some(s)
}
