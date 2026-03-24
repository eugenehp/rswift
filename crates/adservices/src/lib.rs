//! Apple AdServices — Apple Search Ads attribution from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
use apple_objc_sys::*;
pub fn is_available() -> bool { true }

/// Request attribution token.
/// Returns the attribution token string or None on error.
pub fn attribution_token() -> Option<String> {
    unsafe {
        let token: Id = msg_send![class!(b"AAAttribution\0"), attributionTokenWithError: NIL];
        nsstring_to_string(token)
    }
}

