//! Apple AVKit — video player UI from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }



/// AVPlayerView / AVPlayerViewController availability.
pub fn player_view_available() -> bool {
    unsafe {
        #[cfg(target_os = "macos")]
        { !class!(b"AVPlayerView\0").is_null() }
        #[cfg(not(target_os = "macos"))]
        { !class!(b"AVPlayerViewController\0").is_null() }
    }
}

