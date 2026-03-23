//! Check VideoSubscriberAccount via the rswift alias crate.
//!
//! ```sh
//! cargo run -p rswift-videosubscriberaccount --example check
//! ```

fn main() {
    println!("=== rswift-videosubscriberaccount (alias) ===\n");
    println!("Available: {}", rswift_videosubscriberaccount::is_available());

    if rswift_videosubscriberaccount::is_available() {
        println!("✅ VideoSubscriberAccount is ready via rswift namespace.");
    } else {
        println!("❌ VideoSubscriberAccount not available.");
    }
}
