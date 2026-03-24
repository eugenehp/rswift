//! Apple StoreKit — in-app purchases from Rust.
//!
//! **Platform:** macOS 10.7+, iOS 3+, tvOS 9+, watchOS 6.2+.
//!
//! The core API (canMakePayments, receipt URL) uses pure ObjC — no Swift needed.
//! Enable `features = ["storekit2"]` for StoreKit 2 async product fetching
//! (requires Swift toolchain at build time).
//!
//! ```ignore
//! if storekit::can_make_payments() {
//!     println!("✅ In-app purchases enabled");
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


/// Whether the device can make payments (not restricted by parental controls).
pub fn can_make_payments() -> bool {
    unsafe { msg_send_t![bool; class!(b"SKPaymentQueue\0"), canMakePayments] }
}

/// Path to the App Store receipt file, if it exists.
pub fn app_store_receipt_url() -> Option<String> {
    unsafe {
        let bundle: Id = msg_send![class!(b"NSBundle\0"), mainBundle];
        let url: Id = msg_send![bundle, appStoreReceiptURL];
        if url.is_null() { return None; }
        nsstring_to_string(msg_send![url, path])
    }
}

// ── StoreKit 2 (requires `storekit2` feature + Swift toolchain) ─────────────

#[cfg(feature = "storekit2")]
pub mod storekit2 {
    //! StoreKit 2 async product fetching.
    //!
    //! Requires the `storekit2` feature flag (compiles a Swift bridge).

    unsafe extern "C" {
        fn storekit2_fetch_products(
            ids_ptr: *const u8, ids_len: usize,
            cb: extern "C" fn(*const u8, usize, *mut core::ffi::c_void),
            ud: *mut core::ffi::c_void,
        );
    }

    /// Fetch products by ID (comma-separated). Calls the callback with JSON results.
    pub fn fetch_products(
        ids: &str,
        callback: extern "C" fn(*const u8, usize, *mut core::ffi::c_void),
        user_data: *mut core::ffi::c_void,
    ) {
        unsafe { storekit2_fetch_products(ids.as_ptr(), ids.len(), callback, user_data) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_make_payments() {
        // Should not crash; returns true on most Macs
        let _ = can_make_payments();
    }

    #[test]
    fn test_receipt_url() {
        // May be None in test environment
        let _ = app_store_receipt_url();
    }
}
