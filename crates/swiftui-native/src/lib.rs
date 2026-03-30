//! SwiftUI Native — pure Rust, no Swift code, no bridge.
//!
//! Constructs SwiftUI views entirely from Rust by calling Swift runtime
//! functions via dlsym and arm64 inline assembly. Zero Swift source files.
//!
//! # Architecture
//!
//! ```text
//! Rust code
//!   ↓ dlsym (SwiftUI.framework symbols)
//!   ↓ arm64 asm (Swift calling convention)
//! Swift Runtime (libswiftCore + SwiftUI.framework)
//! ```
//!
//! # Available views (symbols exported from SwiftUI.framework)
//!
//! | View | Status |
//! |------|--------|
//! | `text()` | ✅ via LocalizedStringKey.init + Text.init |
//! | `empty_view()` | ✅ via EmptyView.init() |
//! | `divider()` | ✅ via Divider.init() |
//! | `system_image()` | ✅ via Image.init(systemName:) |
//! | `show_window()` | ✅ via NSHostingController + ObjC runtime |
//!
//! # Unavailable (symbols inlined by Swift compiler, not exported)
//!
//! Color, Spacer, VStack/HStack/ZStack, all modifiers (.padding, .frame, etc.),
//! Button, AnyView. These require a Swift bridge — see the `swiftui` crate.
//!
//! # Platform
//!
//! macOS aarch64 only. Requires Swift 6.3 runtime.

pub mod existential;
pub mod resolve;
pub mod views;
pub mod window;

pub use existential::ViewExistential;
