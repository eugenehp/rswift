//! Play an audio file using AVFAudio.
//!
//! ```sh
//! cargo run -p avfaudio --example play_audio -- /path/to/sound.wav
//! ```

fn main() {
    if !avfaudio::is_available() {
        eprintln!("❌ AVFAudio not available.");
        std::process::exit(1);
    }

    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: play_audio <path-to-audio-file>");
            eprintln!("  Supported: WAV, MP3, AAC, AIFF, CAF, M4A");
            std::process::exit(1);
        }
    };

    let player = match avfaudio::AudioPlayer::new(&path) {
        Some(p) => p,
        None => {
            eprintln!("❌ Failed to open: {path}");
            std::process::exit(1);
        }
    };

    println!("=== AVFAudio Player ===\n");
    println!("File:     {path}");
    println!("Duration: {:.2}s", player.duration());
    println!("Volume:   {:.0}%", player.volume() * 100.0);
    println!();

    player.play();
    println!("▶ Playing...");

    // Wait for playback to finish
    while player.is_playing() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("⏹ Done.");
}
