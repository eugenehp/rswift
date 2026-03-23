//! Check usernotifications framework.
//!
//! cargo run -p usernotifications --example check

fn main() {
    println!("=== usernotifications ===");
    println!("Available: {}", usernotifications::is_available());
}
