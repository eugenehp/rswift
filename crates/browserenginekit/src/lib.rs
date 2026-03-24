//! Apple BrowserEngineKit from Rust (Swift bridge).
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
unsafe extern "C" { fn browserenginekit_swift_avail() -> bool; }
pub fn is_available() -> bool { unsafe { browserenginekit_swift_avail() } }
