use apple_platforms::triple::SDK;

fn main() {
    println!("=== Rust Target → SDK Name ===\n");

    let targets = [
        "aarch64-apple-darwin",
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
    ];

    for target in targets {
        match SDK::target_to_sdk(target) {
            Ok(sdk) => println!("  {target:<40} → {sdk}"),
            Err(e) => println!("  {target:<40} → ERROR: {e}"),
        }
    }
}
