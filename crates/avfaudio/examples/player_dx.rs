//! Demonstrate AudioPlayer with improved DX — error handling, Display, and builder pattern.
//!
//! ```sh
//! cargo run -p avfaudio --example player_dx
//! ```

use avfaudio::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> AudioResult {
    println!("=== AVFAudio Player DX ===\n");

    // Set up the session using the fluent builder
    let session = AudioSession::shared();
    session
        .configure()
        .category(Category::Playback)
        .options(CategoryOptions::MIX_WITH_OTHERS)
        .activate()?;

    println!("✅ Session ready\n");

    // Try to open a file (simulating with a nonexistent file for demo)
    // In a real scenario, you'd have an actual audio file
    match AudioPlayer::open("/tmp/nonexistent.mp3") {
        Ok(player) => {
            println!("🎵 Player opened");
            println!("{}", player);
            println!("Debug: {:#?}", player);

            // Play the audio
            player.play();
            println!("\nNow playing...");

            // Display metadata while playing
            for i in 0..5 {
                thread::sleep(Duration::from_millis(200));
                println!("[{}s] {}", i, player);
            }

            player.pause();
            println!("\n⏸ Paused");
        }
        Err(e) => {
            println!("❌ Failed to open file");
            println!("Error: {}", e);
            println!("\n(This is expected — the file doesn't exist)");
        }
    }

    // Demonstrate using the older `new()` API as fallback
    println!("\n--- Using legacy API ---");
    if let Some(player) = AudioPlayer::new("test.m4a") {
        println!("Opened with new(): {}", player);
    } else {
        println!("Could not open test.m4a (expected)");
    }

    Ok(())
}
