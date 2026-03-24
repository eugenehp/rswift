//! Showcase the fluent builder pattern for AudioSession configuration.
//!
//! ```sh
//! cargo run -p avfaudio --example session_builder
//! ```

use avfaudio::prelude::*;

fn main() -> AudioResult {
    println!("=== AVFAudio Session Builder ===\n");

    // Get the shared session
    let session = AudioSession::shared();
    println!("📍 Current session:\n{}\n", session);

    // Use the fluent builder to configure
    println!("🔧 Configuring session...");
    session
        .configure()
        .category(Category::Playback)
        .mode(Mode::Default)
        .options(CategoryOptions::DUCK_OTHERS | CategoryOptions::DEFAULT_TO_SPEAKER)
        .preferred_sample_rate(48_000.0)
        .activate()?;

    println!("✅ Session configured!\n");
    println!("📍 Updated session:\n{}\n", session);

    // Display detailed information
    println!("📊 Details:");
    println!("  Sample rate:        {:.0} Hz", session.sample_rate());
    println!("  Input channels:     {}", session.input_number_of_channels());
    println!("  Output channels:    {}", session.output_number_of_channels());
    println!("  Output volume:      {:.0}%", session.output_volume() * 100.0);
    println!("  Input available:    {}", session.is_input_available());
    println!("  Other audio:        {}", session.is_other_audio_playing());

    Ok(())
}
