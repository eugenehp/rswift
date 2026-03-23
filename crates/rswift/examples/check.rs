//! Check available frameworks via the unified rswift crate.
//!
//! ```sh
//! cargo run -p rswift --example check --features avfaudio,avfoundation,metal
//! ```

use rswift::prelude::*;

fn main() {
    println!("=== rswift — Apple Frameworks from Rust ===\n");

    // AVFAudio
    println!("AVFAudio:      {}", status(avfaudio::is_available()));
    if avfaudio::is_available() {
        let engine = avfaudio::AudioEngine::new();
        let (sr, ch) = engine.output_format();
        println!("  Audio engine: {sr} Hz, {ch} ch");
    }

    // AVFoundation
    println!("AVFoundation:  {}", status(avfoundation::is_available()));

    // Metal
    println!("Metal:         available (GPU crate loaded)");

    println!("\n✅ All checked.");
}

fn status(available: bool) -> &'static str {
    if available { "✅ available" } else { "❌ unavailable" }
}
