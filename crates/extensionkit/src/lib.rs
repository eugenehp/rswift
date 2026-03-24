//! Apple ExtensionKit — app extension hosting from Rust.
//!
//! **Platform:** macOS 13+, iOS 16+.

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {
    unsafe { !class!(b"EXAppExtensionBrowserViewController\0").is_null() }
}

/// Check if extension browser is available.
pub fn browser_available() -> bool { is_available() }

/// Check if extension host view controller is available.
pub fn host_view_controller_available() -> bool {
    unsafe { !class!(b"EXHostViewController\0").is_null() }
}
