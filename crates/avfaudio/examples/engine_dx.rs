//! Demonstrate the audio engine with improved DX and Display implementations.
//!
//! ```sh
//! cargo run -p avfaudio --example engine_dx
//! ```

use avfaudio::prelude::*;

fn main() -> AudioResult {
    println!("=== AVFAudio Engine DX ===\n");

    // Configure the session first
    AudioSession::shared()
        .configure()
        .category(Category::Playback)
        .activate()?;

    // Create an engine
    let engine = AudioEngine::new();
    println!("🎛 Engine created");
    println!("{}\n", engine);

    // Show detailed info
    println!("📊 Engine Details:");
    println!("Debug: {:#?}\n", engine);

    // Inspect nodes
    let output = engine.output_node();
    let input = engine.input_node();
    let mixer = engine.main_mixer_node();

    println!("🔗 Built-in nodes:");
    println!("  Output: {}", output);
    println!("  Input:  {}", input);
    println!("  Mixer:  {}", mixer);

    // Try to start/stop the engine
    println!("\n🚀 Lifecycle:");
    engine.prepare();
    println!("  Prepared: ✅");

    match engine.try_start() {
        Ok(()) => {
            println!("  Started:  ✅");
            println!("  Running:  {}", engine.is_running());
            println!("  Status:   {}", engine);

            engine.stop();
            println!("  Stopped:  ✅");
        }
        Err(e) => {
            println!("  Start failed: {}", e);
            println!("  (This can happen if there's no audio device)");
        }
    }

    Ok(())
}
