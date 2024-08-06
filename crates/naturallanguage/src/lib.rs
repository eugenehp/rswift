//! Apple NaturalLanguage — text processing and NLP from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 12+, tvOS 12+, visionOS 1+, watchOS 5+.
//!
//! Wraps NaturalLanguage for language detection, tokenization, and sentiment analysis.
//!
//! ```ignore
//! assert!(naturallanguage::is_available());
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

apple_sys_helpers::apple_framework!(c"naturallanguage_available");
