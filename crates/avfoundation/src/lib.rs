//! Apple AVFoundation — media capture, playback, and editing from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+.
//!
//! Wraps AVFoundation for camera capture, video playback, and media composition.
//!
//! ```ignore
//! assert!(avfoundation::is_available());
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

apple_sys_helpers::apple_framework!(c"avfoundation_available"; "macos", "ios", "tvos", "xros");
