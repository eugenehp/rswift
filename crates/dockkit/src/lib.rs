//! Apple DockKit from Rust (Swift bridge).
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
unsafe extern "C" { fn dockkit_swift_avail() -> bool; }
pub fn is_available() -> bool { unsafe { dockkit_swift_avail() } }
