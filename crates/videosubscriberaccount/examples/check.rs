//! Check VideoSubscriberAccount framework availability.
//!
//! ```sh
//! cargo run -p videosubscriberaccount --example check
//! ```

fn main() {
    println!("=== VideoSubscriberAccount Framework ===\n");
    println!("Available: {}", videosubscriberaccount::is_available());

    if !videosubscriberaccount::is_available() {
        println!("❌ VideoSubscriberAccount is not available on this platform.");
        println!("   Supported: macOS 10.14+, iOS 10+, tvOS 10+.");
        return;
    }

    println!("✅ VideoSubscriberAccount is ready\n");
    println!("Use cases:");
    println!("  • Single sign-on with TV providers");
    println!("  • Subscriber verification for premium content");
    println!("  • TV provider picker UI integration");
}
