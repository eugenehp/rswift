//! Apple RealityFoundation (via RealityKit) from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
unsafe extern "C" { fn realityfoundation_swift_avail() -> bool; }
pub fn is_available() -> bool { unsafe { realityfoundation_swift_avail() } }
