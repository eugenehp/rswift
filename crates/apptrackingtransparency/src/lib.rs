//! Apple AppTrackingTransparency — tracking authorization from Rust.
//!
//! **Platform:** iOS 14+, tvOS 14+, visionOS 1+.
//!
//! ```ignore
//! println!("Tracking: {:?}", apptrackingtransparency::status());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

pub fn is_available() -> bool { true }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus {
    NotDetermined = 0,
    Restricted = 1,
    Denied = 2,
    Authorized = 3,
}

impl From<usize> for AuthorizationStatus {
    fn from(v: usize) -> Self {
        match v { 1=>Self::Restricted, 2=>Self::Denied, 3=>Self::Authorized, _=>Self::NotDetermined }
    }
}

/// Current tracking authorization status.
pub fn status() -> AuthorizationStatus {
    unsafe {
        AuthorizationStatus::from(msg_send_t![usize; class!(b"ATTrackingManager\0"), trackingAuthorizationStatus])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status() { let _ = status(); }
}
