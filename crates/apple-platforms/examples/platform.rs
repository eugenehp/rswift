use apple_platforms::platform::{ApplePlatform, Platform};

fn main() {
    println!("=== Apple Platforms (from LLVM MachO.def) ===\n");
    println!("{:<20} {:>2}  {:<18} {:<18} {}", "Platform", "ID", "Target", "TAPI Target", "Marketing");
    println!("{}", "-".repeat(85));

    for ap in ApplePlatform::iter() {
        let p = Platform::from(ap);
        println!(
            "{:<20} {:>2}  {:<18} {:<18} {}",
            p.platform, p.id, p.target, p.tapi_target, p.marketing
        );
    }

    println!("\nSimulators:");
    for ap in ApplePlatform::iter().filter(|p| p.is_simulator()) {
        println!("  {} → device: {}", ap, ap.device_platform());
    }
}
