//! Apple RelevanceKit from Rust (Swift bridge).
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
unsafe extern "C" { fn relevancekit_swift_avail() -> bool; }
pub fn is_available() -> bool { unsafe { relevancekit_swift_avail() } }
