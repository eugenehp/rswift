//! Apple MediaPlayer — music and media playback from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 3+, tvOS 14+, watchOS 5+.
//!
//! Wraps MediaPlayer for system music player and Now Playing info.
//!
//! ```ignore
//! assert!(mediaplayer::is_available());
//! ```

//!
//! ## Citation
//!
//! ```bibtex
//! @software{rswift,
//!   author       = {Eugene Hauptmann},
//!   title        = {rswift},
//!   year         = {2025},
//!   url          = {https://github.com/eugenehp/rswift},
//!   note         = {Build native Apple apps from Rust}
//! }
//! ```
//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"mediaplayer_available"; "macos", "ios", "tvos", "watchos");
