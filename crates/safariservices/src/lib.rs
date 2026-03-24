//! Apple SafariServices — in-app Safari browser from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
use apple_objc_sys::*;
pub fn is_available() -> bool { true }

/// Check if SFSafariViewController is available.
pub fn safari_view_available() -> bool {
    unsafe { !class!(b"SFSafariViewController\0").is_null() }
}

