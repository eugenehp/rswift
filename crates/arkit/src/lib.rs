//! Apple ARKit from Rust (Swift bridge).
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
unsafe extern "C" { fn arkit_swift_avail() -> bool; }
pub fn is_available() -> bool { unsafe { arkit_swift_avail() } }
