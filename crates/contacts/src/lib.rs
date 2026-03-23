//! Apple Contacts — address book access from Rust.
//!
//! **Platform support:** macOS 10.11+, iOS 9+, watchOS 2+.
//!
//! # Quick start
//!
//! ```ignore
//! println!("Auth: {:?}", contacts::authorization_status());
//! println!("Count: {}", contacts::count());
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"contacts_available");

unsafe extern "C" {
    fn contacts_authorization_status() -> isize;
    fn contacts_count() -> isize;
}

/// Contact store authorization status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus {
    NotDetermined = 0,
    Restricted = 1,
    Denied = 2,
    Authorized = 3,
}

impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self {
        match v {
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::Authorized,
            _ => Self::NotDetermined,
        }
    }
}

/// Current authorization status for contacts access.
pub fn authorization_status() -> AuthorizationStatus {
    AuthorizationStatus::from(unsafe { contacts_authorization_status() })
}

/// Number of contacts in the address book (-1 if access denied).
pub fn count() -> isize {
    unsafe { contacts_count() }
}
