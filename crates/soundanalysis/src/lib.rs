//! Apple SoundAnalysis — audio classification from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
use apple_objc_sys::*;
pub fn is_available() -> bool { true }

/// Check if sound analysis is available.
pub fn analyzer_available() -> bool {
    unsafe { !class!(b"SNAudioStreamAnalyzer\0").is_null() }
}

