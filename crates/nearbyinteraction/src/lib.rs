//! Apple NearbyInteraction — Ultra-Wideband ranging from Rust.
//!
//! **Platform:** iOS 14+, watchOS 7+.
//!
//! ```ignore
//! if nearbyinteraction::is_supported() {
//!     println!("UWB ranging supported");
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {
    unsafe { !class!(b"NISession\0").is_null() }
}

/// Whether device-to-device ranging is supported.
pub fn is_supported() -> bool {
    unsafe { msg_send_t![bool; class!(b"NISession\0"), isSupported] }
}

/// Wraps `NISession`.
pub struct Session { inner: Id }

impl Session {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"NISession\0"), new] } }
    }

    /// Invalidate the session.
    pub fn invalidate(&self) {
        unsafe { msg_send_void![self.inner, invalidate]; }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for Session { fn default() -> Self { Self::new() } }
impl Drop for Session { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Direction result from a peer.
#[derive(Debug, Clone, Copy)]
pub struct NearbyObject {
    pub distance: f32,
    pub direction_x: f32,
    pub direction_y: f32,
    pub direction_z: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported() {
        let _ = is_supported();
    }
}
