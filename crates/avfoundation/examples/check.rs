//! Check AVFoundation framework availability.
//!
//! ```sh
//! cargo run -p rswift-avfoundation --example check
//! ```

fn main() {
    println!("=== AVFoundation Framework ===\n");
    println!("Available: {}", avfoundation::is_available());

    if !avfoundation::is_available() {
        println!("❌ AVFoundation is not available on this platform.");
        println!("   Supported: macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+.");
        return;
    }

    println!("✅ AVFoundation is ready\n");
    println!("Capabilities:");
    println!("  • Media playback (AVPlayer)");
    println!("  • Asset inspection (AVURLAsset)");
    println!("  • Camera capture");
    println!("  • Video/audio composition");
}
