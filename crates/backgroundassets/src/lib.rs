//! Apple BackgroundAssets — background asset downloads from Rust.
//! Uses Swift bridge for Swift-only APIs.

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

unsafe extern "C" { fn ba_swift_available() -> bool; }

pub fn is_available() -> bool { unsafe { ba_swift_available() } }


