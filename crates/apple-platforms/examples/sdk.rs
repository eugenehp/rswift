use apple_platforms::{triple, ApplePlatform};

fn main() {
    println!("=== Rust Target → SDK Name ===\n");

    let targets = [
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "aarch64-apple-ios",
        "aarch64-apple-ios-sim",
        "aarch64-apple-ios-macabi",
        "aarch64-apple-tvos",
        "aarch64-apple-tvos-sim",
        "aarch64-apple-watchos",
        "aarch64-apple-watchos-sim",
        "aarch64-apple-visionos",
        "aarch64-apple-visionos-sim",
        "aarch64-apple-driverkit",
        "x86_64-unknown-linux-gnu",   // not Apple — should be None
    ];

    println!("{:<42}  {:<18}  {}", "Rust triple", "SDK", "Platform");
    println!("{}", "-".repeat(85));
    for target in targets {
        let sdk = triple::to_sdk(target).unwrap_or("(unknown)");
        let platform = ApplePlatform::from_rust_triple(target)
            .map(|p| p.to_string())
            .unwrap_or_else(|_| "(not Apple)".to_string());
        println!("  {target:<40}  {sdk:<18}  {platform}");
    }
}
