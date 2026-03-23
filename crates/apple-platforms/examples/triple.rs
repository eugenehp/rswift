use apple_platforms::triple::Triple;

fn main() {
    println!("=== Rust → Clang Target Conversion ===\n");

    let targets = [
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "aarch64-apple-ios",
        "aarch64-apple-ios-sim",
        "aarch64-apple-tvos",
        "aarch64-apple-watchos",
        "aarch64-apple-visionos",
        "aarch64-apple-ios-macabi",
        "aarch64-apple-driverkit",
    ];

    for target in targets {
        let clang = Triple::target_to_clang_target(target);
        println!("  {target:<40} → {clang}");
    }
}
