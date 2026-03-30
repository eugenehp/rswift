//! Launch a SwiftUI app from pure Rust — no Swift source code.
//!
//! ```bash
//! cargo run -p swiftui-native --example app
//! ```

fn main() {
    println!("Launching SwiftUI app from pure Rust...");
    println!("No Swift source code. No bridge dylib. No shim.");
    println!("Just Rust → dlsym → arm64 asm → Swift runtime.");

    swiftui_native::app::run(|| {
        let title = swiftui_native::views::text("Hello from pure Rust! 🦀");
        let padded = swiftui_native::views::padding(&title, 40.0);
        padded
    });
}
