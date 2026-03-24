//! Apple DeviceCheck — device validation from Rust.
//!
//! **Platform:** macOS 10.15+, iOS 11+, tvOS 11+.
//!
//! ```ignore
//! if devicecheck::is_supported() {
//!     println!("DeviceCheck available on this device");
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

pub fn is_available() -> bool { true }

/// Whether DeviceCheck is supported on this device.
pub fn is_supported() -> bool {
    unsafe {
        let dev: Id = msg_send![class!(b"DCDevice\0"), currentDevice];
        msg_send_t![bool; dev, isSupported]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_supported() { let _ = is_supported(); }
}
