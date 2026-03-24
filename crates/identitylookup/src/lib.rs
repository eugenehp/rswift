//! Apple IdentityLookup — call/message filtering from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
use apple_objc_sys::*;
pub fn is_available() -> bool { true }

/// Check if call classification is available.
pub fn classification_available() -> bool {
    unsafe { !class!(b"ILClassificationRequest\0").is_null() }
}

