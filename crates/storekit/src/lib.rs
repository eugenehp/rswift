//! Apple StoreKit — in-app purchases from Rust.
//!
//! **Platform support:** macOS 10.7+, iOS 3+, tvOS 9+, watchOS 6.2+.
//!
//! # Quick start
//!
//! ```ignore
//! if storekit::can_make_payments() {
//!     println!("✅ In-app purchases enabled");
//! }
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"storekit_available");

unsafe extern "C" {
    fn storekit_can_make_payments() -> bool;
    fn storekit_app_store_receipt_url(buf: *mut u8, bl: usize) -> isize;
}

/// Whether the device can make payments (not restricted by parental controls).
pub fn can_make_payments() -> bool {
    unsafe { storekit_can_make_payments() }
}

/// Path to the App Store receipt file, if it exists.
pub fn app_store_receipt_url() -> Option<String> {
    let mut buf = vec![0u8; 4096];
    let len = unsafe { storekit_app_store_receipt_url(buf.as_mut_ptr(), buf.len()) };
    if len < 0 { None } else { Some(String::from_utf8_lossy(&buf[..len as usize]).into()) }
}
