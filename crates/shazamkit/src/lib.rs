//! Apple ShazamKit — music recognition from Rust.
//!
//! **Platform:** macOS 12+, iOS 15+, tvOS 15+.
//!
//! ```ignore
//! if shazamkit::is_available() && shazamkit::is_catalog_available() {
//!     println!("ShazamKit ready for music matching");
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

unsafe extern "C" {
    fn shazamkit_swift_available() -> bool;
}

pub fn is_available() -> bool { unsafe { shazamkit_swift_available() } }

/// Whether the Shazam catalog is available for matching.
pub fn is_catalog_available() -> bool {
    unsafe {
        extern "C" { fn shazamkit_is_catalog_available() -> bool; }
        shazamkit_is_catalog_available()
    }
}

/// A matched media item from Shazam.
#[derive(Debug, Clone)]
pub struct MatchedItem {
    pub title: String,
    pub artist: String,
    pub apple_music_id: Option<String>,
    pub artwork_url: Option<String>,
}
