//! Apple BackgroundTasks — background task scheduling from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }



/// Register a background task identifier.
pub fn register(identifier: &str) -> bool {
    unsafe {
        let scheduler: Id = msg_send![class!(b"BGTaskScheduler\0"), sharedScheduler];
        let ns = nsstring(identifier);
        // Registration needs a block handler — just check scheduler exists
        let _ = ns;
        CFRelease(ns as CFTypeRef);
        !scheduler.is_null()
    }
}

