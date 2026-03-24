//! Apple MLCompute — ML training on GPU from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;
pub fn is_available() -> bool {
    unsafe { !class!(b"MLCDevice\0").is_null() }
}
