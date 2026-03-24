//! Apple ClockKit from Rust (Swift bridge).
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
unsafe extern "C" { fn clockkit_swift_avail() -> bool; }
pub fn is_available() -> bool { unsafe { clockkit_swift_avail() } }
