//! Check contacts framework.
//!
//! cargo run -p contacts --example check

fn main() {
    println!("=== contacts ===");
    println!("Available: {}", contacts::is_available());
}
