//! `View` — chainable modifier API for ergonomic view construction.
//!
//! ```ignore
//! use swiftui_native::prelude::*;
//!
//! text("Hello").bold().padding(16.0).bg(Color::BLUE).opacity(0.9)
//! ```

use crate::color::Color;
use crate::views::{self, ViewHandle};

/// A SwiftUI view with chainable modifiers.
pub struct View {
    pub(crate) handle: ViewHandle,
}

impl View {
    pub fn new(handle: ViewHandle) -> Self {
        Self { handle }
    }

    /// Get the underlying handle.
    pub fn handle(&self) -> &ViewHandle {
        &self.handle
    }

    /// Consume and return the handle.
    pub fn into_handle(self) -> ViewHandle {
        self.handle
    }

    // ── Layout ──

    /// Apply `.padding(_:)`.
    pub fn padding(self, amount: f64) -> Self {
        Self::new(views::padding(&self.handle, amount))
    }

    /// Apply `.frame(width:height:)`.
    pub fn frame(self, w: f64, h: f64) -> Self {
        Self::new(views::frame(&self.handle, w, h))
    }

    // ── Appearance ──

    /// Apply `.background(_:)` with a color.
    pub fn bg(self, c: Color) -> Self {
        Self::new(views::background(&self.handle, c.r, c.g, c.b, c.a))
    }

    /// Apply `.opacity(_:)`.
    pub fn opacity(self, value: f64) -> Self {
        Self::new(views::opacity(&self.handle, value))
    }

    /// Apply `.cornerRadius(_:)`.
    pub fn rounded(self, radius: f64) -> Self {
        Self::new(views::corner_radius(&self.handle, radius))
    }

    /// Apply `.border(_:width:)`.
    pub fn border(self, c: Color, width: f64) -> Self {
        Self::new(views::border(&self.handle, c.r, c.g, c.b, width))
    }

    /// Shorthand: padding + background + corner radius.
    pub fn card(self, pad: f64, bg: Color, radius: f64) -> Self {
        self.padding(pad).bg(bg).rounded(radius)
    }
}

impl std::fmt::Debug for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View({:?})", self.handle)
    }
}

/// Anything that can become a `View`.
pub trait IntoView {
    fn into_view(self) -> View;
}

impl IntoView for View {
    fn into_view(self) -> View {
        self
    }
}

impl IntoView for ViewHandle {
    fn into_view(self) -> View {
        View::new(self)
    }
}

impl IntoView for &str {
    fn into_view(self) -> View {
        text(self)
    }
}

impl IntoView for String {
    fn into_view(self) -> View {
        text(&self)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Free functions — the ergonomic DSL
// ═══════════════════════════════════════════════════════════════════════════

/// Create a `Text` view.
pub fn text(s: &str) -> View {
    View::new(views::text(s))
}

/// Create a `Color` view.
pub fn color(c: Color) -> View {
    View::new(views::color(c.r, c.g, c.b, c.a))
}

/// Create a `Spacer`.
pub fn spacer() -> View {
    View::new(views::spacer())
}

/// Create a `Divider`.
pub fn divider() -> View {
    View::new(views::divider())
}

/// Create an `EmptyView`.
pub fn empty() -> View {
    View::new(views::empty_view())
}

/// Create an SF Symbol `Image`.
pub fn image(system_name: &str) -> View {
    View::new(views::system_image(system_name))
}

/// Create a `VStack`.
pub fn vstack(children: &[View]) -> View {
    let handles: Vec<ViewHandle> = children.iter().map(|v| v.handle.clone()).collect();
    View::new(views::vstack(&handles))
}

/// Create an `HStack`.
pub fn hstack(children: &[View]) -> View {
    let handles: Vec<ViewHandle> = children.iter().map(|v| v.handle.clone()).collect();
    View::new(views::hstack(&handles))
}

/// Create a `ZStack`.
pub fn zstack(children: &[View]) -> View {
    let handles: Vec<ViewHandle> = children.iter().map(|v| v.handle.clone()).collect();
    View::new(views::zstack(&handles))
}

/// Create a `Button`.
pub fn button(label: &str, action: fn()) -> View {
    View::new(views::button(label, action))
}
