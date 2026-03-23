//! Check AVFAudio via the rswift-avfaudio alias crate.
//!
//! ```sh
//! cargo run -p rswift-avfaudio --example check
//! ```

fn main() {
    println!("=== rswift-avfaudio (alias for avfaudio) ===\n");
    println!("Available: {}", rswift_avfaudio::is_available());

    if !rswift_avfaudio::is_available() {
        println!("❌ AVFAudio not available.");
        return;
    }

    println!("✅ AVFAudio is ready via rswift namespace\n");

    // Demo: use the re-exported AudioEngine
    let engine = rswift_avfaudio::AudioEngine::new();
    let (sr, ch) = engine.output_format();
    println!("Audio output: {sr} Hz, {ch} channels");
}
