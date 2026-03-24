//! Apple AppIntents — app intents and shortcuts from Rust.
//! Uses Swift bridge for Swift-only APIs.

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

unsafe extern "C" { fn appintents_swift_available() -> bool; }

pub fn is_available() -> bool { unsafe { appintents_swift_available() } }


