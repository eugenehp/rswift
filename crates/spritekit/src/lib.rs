//! Apple SpriteKit — 2D game engine from Rust.
//!
//! **Platform support:** macOS 10.9+, iOS 7+, tvOS 9+, visionOS 1+, watchOS 3+.
//!
//! Wraps SpriteKit for 2D sprites, physics, and particle systems.
//!
//! ```ignore
//! assert!(spritekit::is_available());
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

apple_sys_helpers::apple_framework!(c"spritekit_available");
