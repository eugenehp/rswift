use apple_platforms::platform::{ApplePlatform, Platform};

fn main() {
    println!("=== Apple Platforms (from LLVM MachO.def) ===\n");
    println!(
        "{:<20} {:>2}  {:<18} {:<18} {:<22} {}",
        "Platform", "ID", "Target", "TAPI Target", "Marketing", "SDK"
    );
    println!("{}", "-".repeat(100));

    for ap in ApplePlatform::iter() {
        let p = Platform::from(ap);
        println!(
            "{:<20} {:>2}  {:<18} {:<18} {:<22} {}",
            p.platform,
            p.id,
            p.target,
            p.tapi_target,
            p.marketing,
            p.sdk.unwrap_or("-"),
        );
    }

    println!("\nSimulators  (sim → device):");
    for ap in ApplePlatform::iter().filter(|p| p.is_simulator()) {
        println!("  {:<22} → {}", ap, ap.device());
    }

    println!("\nDevice platforms  (device → sim):");
    for ap in ApplePlatform::iter().filter(|p| p.is_device()) {
        let sim = ap.simulator()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "(none)".to_string());
        println!("  {:<22} → {}", ap, sim);
    }
}
