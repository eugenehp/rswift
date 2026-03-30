//! Everything you need for a native SwiftUI app.
//!
//! ```ignore
//! use swiftui_native::prelude::*;
//!
//! fn main() {
//!     run(|| {
//!         vstack(&[
//!             text("Hello").padding(20.0),
//!             color(Color::BLUE).frame(100.0, 50.0),
//!             spacer(),
//!         ]).bg(Color::DARK)
//!     });
//! }
//! ```

// View type + chainable modifiers
pub use crate::view::{IntoView, View};

// Free functions (DSL)
pub use crate::view::{button, color, divider, empty, hstack, image, spacer, text, vstack, zstack};

// Color type + constants
pub use crate::color::Color;

// App lifecycle
pub use crate::app::run;

// Window (for manual hosting without App lifecycle)
pub use crate::views::show_window;
