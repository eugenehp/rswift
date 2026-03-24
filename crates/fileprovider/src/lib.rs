//! Apple FileProvider — cloud file provider extension from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
use apple_objc_sys::*;
pub fn is_available() -> bool { true }

/// File provider manager.
pub fn domain_count() -> usize {
    // NSFileProviderManager requires entitlements; just check class availability
    unsafe {
        if class!(b"NSFileProviderManager\0").is_null() { 0 } else { 1 }
    }
}

