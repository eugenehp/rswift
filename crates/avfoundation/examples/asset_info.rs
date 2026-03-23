//! Inspect a media asset's metadata using AVFoundation.
//!
//! ```sh
//! cargo run -p rswift-avfoundation --example asset_info -- /path/to/video.mp4
//! ```

fn main() {
    if !avfoundation::is_available() {
        eprintln!("❌ AVFoundation not available.");
        std::process::exit(1);
    }

    let url = match std::env::args().nth(1) {
        Some(u) => u,
        None => {
            eprintln!("Usage: asset_info <file-path-or-url>");
            std::process::exit(1);
        }
    };

    println!("=== AVFoundation Asset Info ===\n");

    let asset = avfoundation::Asset::new(&url);
    let duration = asset.duration();

    println!("URL:      {url}");
    println!("Duration: {:.2}s", duration);

    if duration > 0.0 {
        let mins = (duration / 60.0).floor() as u64;
        let secs = duration % 60.0;
        println!("          {}m {:.1}s", mins, secs);
    }
}
