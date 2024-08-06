//! Apple AVKit — media playback UI from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+.
//!
//! Wraps AVKit for AVPlayerViewController and picture-in-picture.
//!
//! ```ignore
//! assert!(avkit::is_available());
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

apple_sys_helpers::apple_framework!(c"avkit_available"; "macos", "ios", "tvos", "xros");
