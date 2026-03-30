//! A SwiftUI app from pure Rust — no Swift source code.
//!
//! ```bash
//! cargo run -p swiftui-native --example app
//! ```

use swiftui_native::prelude::*;

fn main() {
    run(|| {
        vstack(&[
            text("Hello from Rust! 🦀")
                .padding(12.0)
                .bg(Color::BLUE)
                .rounded(8.0)
                .opacity(0.95),
            spacer(),
            hstack(&[
                image("star.fill"),
                text("No Swift compiler needed"),
            ]),
            divider(),
            text("Pure dlsym + arm64 asm → SwiftUI")
                .padding(8.0),
        ])
        .padding(40.0)
        .into_handle()
    });
}
