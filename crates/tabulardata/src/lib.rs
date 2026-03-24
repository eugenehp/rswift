//! Apple TabularData — data tables and CSV from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
use apple_objc_sys::*;
pub fn is_available() -> bool { true }

/// Check if TabularData is available.
pub fn data_frame_available() -> bool {
    unsafe { !class!(b"MLDataTable\0").is_null() }
}

