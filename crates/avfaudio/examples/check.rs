//! Check AVFAudio framework availability and query the audio engine.
//!
//! ```sh
//! cargo run -p avfaudio --example check
//! ```

fn main() {
    println!("=== AVFAudio Framework ===\n");
    println!("Available: {}", avfaudio::is_available());

    if !avfaudio::is_available() {
        println!("❌ AVFAudio is not available on this platform.");
        return;
    }

    println!("✅ AVFAudio is ready\n");

    // Create an audio engine and inspect the output format
    let engine = avfaudio::AudioEngine::new();
    let (sample_rate, channels) = engine.output_format();
    println!("Audio Engine:");
    println!("  Output sample rate: {sample_rate} Hz");
    println!("  Output channels:    {channels}");
    println!("  Running:            {}", engine.is_running());

    // Start/stop the engine
    if engine.start() {
        println!("  Started:            ✅");
        println!("  Running:            {}", engine.is_running());
        engine.stop();
        println!("  Stopped:            ✅");
    } else {
        println!("  Start failed (no audio device?)");
    }
}
