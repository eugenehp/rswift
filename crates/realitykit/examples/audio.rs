//! Audio demo — spatial and non-spatial audio playback.
//!
//! cargo run -p realitykit --example audio -- path/to/sound.wav

use realitykit::prelude::*;

fn main() {
    println!("=== RealityKit audio demo ===\n");

    let audio_path = std::env::args().nth(1)
        .unwrap_or_else(|| "sound.wav".to_string());

    // Spatial audio source
    match AudioResource::load(&audio_path, AudioInputMode::Spatial) {
        Ok(resource) => {
            let emitter = Entity::new().at(2.0, 1.0, 0.0);
            emitter.set_spatial_audio(-3.0, -6.0); // direct -3 dB, reverb -6 dB

            let ctrl = emitter.play_audio(&resource);
            println!("Playing spatial audio: '{audio_path}'");
            println!("  is_playing: {}", ctrl.is_playing());
            println!("  gain:  {} dB", ctrl.gain());
            println!("  speed: {}x",  ctrl.speed());

            ctrl.set_gain(-6.0);
            ctrl.set_speed(1.0);
            ctrl.pause();
            ctrl.resume();

            // Non-spatial (background music)
            if let Ok(bg) = AudioResource::load(&audio_path, AudioInputMode::NonSpatial) {
                let bg_entity = Entity::new();
                bg_entity.set_ambient_audio(-12.0);
                let bg_ctrl = bg_entity.play_audio(&bg);
                bg_ctrl.set_gain(-12.0);
                println!("Background music started at -12 dB");
            }

            println!("\n✓ Audio scene assembled.");
        }
        Err(e) => {
            println!("Note: could not load '{audio_path}': {e}");
            println!("Pass an audio file path as the first argument.");
        }
    }
}
