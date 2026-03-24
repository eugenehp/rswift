//! Apple CoreSpotlight — search indexing from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

pub fn is_available() -> bool { true }


/// Check if indexing is available on this device.
pub fn is_indexing_available() -> bool {
    unsafe { msg_send_t![bool; class!(b"CSSearchableIndex\0"), isIndexingAvailable] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_indexing() { let _ = is_indexing_available(); }
}

