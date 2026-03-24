//! Apple QuickLook — file preview from Rust.
//! Uses Swift bridge for Swift-only APIs.

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

unsafe extern "C" { fn quicklook_swift_available() -> bool; }

pub fn is_available() -> bool { unsafe { quicklook_swift_available() } }


