//! AVFoundation media player — play a local or remote media file.
//!
//! ```sh
//! cargo run -p rswift-avfoundation --example player -- /path/to/video.mp4
//! cargo run -p rswift-avfoundation --example player -- https://example.com/audio.mp3
//! ```

fn main() {
    if !avfoundation::is_available() {
        eprintln!("❌ AVFoundation not available.");
        std::process::exit(1);
    }

    let url = match std::env::args().nth(1) {
        Some(u) => u,
        None => {
            eprintln!("Usage: player <file-path-or-url>");
            eprintln!("  Examples:");
            eprintln!("    player /path/to/movie.mp4");
            eprintln!("    player https://example.com/stream.m3u8");
            std::process::exit(1);
        }
    };

    println!("=== AVFoundation Player ===\n");

    let player = avfoundation::Player::new(&url);
    println!("URL:      {url}");
    println!("Status:   {:?}", player.status());
    println!("Volume:   {:.0}%", player.volume() * 100.0);
    println!("Muted:    {}", player.is_muted());
    println!("Rate:     {}", player.rate());
    println!("Duration: {:.2}s", player.duration());
    println!();

    // Start playback
    player.play();
    println!("▶ Playing (rate: {})", player.rate());

    // Let it play for a moment
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("  Position: {:.2}s", player.current_time());

    // Pause
    player.pause();
    println!("⏸ Paused (rate: {})", player.rate());

    // Volume control
    player.set_volume(0.5);
    println!("  Volume set to 50%");

    // Seek
    player.seek(0.0);
    println!("  Seeked to 0s");

    println!("\n⏹ Done.");
}
