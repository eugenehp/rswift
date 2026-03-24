//! Apple Symbols — SF Symbols from Rust.
//! Uses Swift bridge for Swift-only APIs.

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

unsafe extern "C" { fn symbols_swift_available() -> bool; }

pub fn is_available() -> bool { unsafe { symbols_swift_available() } }


