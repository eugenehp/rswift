//! AudioToolbox system sounds demo.
//!
//! cargo run -p rswift-audiotoolbox --example check

fn main() {
    println!("=== AudioToolbox ===\n");
    println!("Available: {}", audiotoolbox::is_available());

    // Play a system sound
    println!("Playing system sound (Glass)...");
    audiotoolbox::play_system_sound(audiotoolbox::sounds::GLASS);

    std::thread::sleep(std::time::Duration::from_millis(500));

    // Load and play a custom sound
    let path = "/System/Library/Sounds/Ping.aiff";
    match audiotoolbox::SystemSound::new(path) {
        Ok(sound) => {
            println!("Loaded: {path}");
            sound.play();
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("Done.");
        }
        Err(e) => println!("Failed to load {path}: error {e}"),
    }
}
