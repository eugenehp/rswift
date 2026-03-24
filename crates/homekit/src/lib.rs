//! Apple HomeKit from Rust (Swift bridge).
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
unsafe extern "C" { fn homekit_swift_avail() -> bool; }
pub fn is_available() -> bool { unsafe { homekit_swift_avail() } }
