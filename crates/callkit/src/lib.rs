//! Apple CallKit — VoIP call integration from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }



/// Check if call directory extension is supported.
pub fn is_call_directory_supported() -> bool {
    unsafe { !class!(b"CXCallDirectoryManager\0").is_null() }
}

