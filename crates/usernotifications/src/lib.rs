//! Apple UserNotifications — local notification scheduling from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 10+, tvOS 10+, watchOS 3+.
//!
//! # Quick start
//!
//! ```ignore
//! // Cancel all pending notifications
//! usernotifications::cancel_all();
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"usernotifications_available");

unsafe extern "C" {
    fn usernotifications_cancel(id: *const u8, len: usize);
    fn usernotifications_cancel_all();
}

/// Cancel a pending notification by identifier.
pub fn cancel(identifier: &str) {
    unsafe { usernotifications_cancel(identifier.as_ptr(), identifier.len()) }
}

/// Cancel all pending notifications.
pub fn cancel_all() {
    unsafe { usernotifications_cancel_all() }
}

/// Authorization options flags.
pub mod options {
    pub const BADGE: u64 = 1;
    pub const SOUND: u64 = 2;
    pub const ALERT: u64 = 4;
    pub const PROVISIONAL: u64 = 64;
}
