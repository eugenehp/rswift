//! Apple AdSupport — advertising identifier from Rust.
//!
//! **Platform:** macOS 10.14+, iOS 6+, tvOS 6+.
//!
//! ```ignore
//! let idfa = adsupport::advertising_identifier();
//! println!("IDFA: {idfa}");
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

pub fn is_available() -> bool { true }

/// The advertising identifier (IDFA) as a UUID string.
///
/// Returns `"00000000-0000-0000-0000-000000000000"` if tracking is denied.
pub fn advertising_identifier() -> String {
    unsafe {
        let mgr: Id = msg_send![class!(b"ASIdentifierManager\0"), sharedManager];
        let uuid: Id = msg_send![mgr, advertisingIdentifier];
        nsstring_to_string(msg_send![uuid, UUIDString]).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_idfa() {
        let idfa = advertising_identifier();
        assert_eq!(idfa.len(), 36); // UUID format
    }
}
