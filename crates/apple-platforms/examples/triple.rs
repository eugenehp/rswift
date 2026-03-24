use apple_platforms::triple;

fn main() {
    println!("=== Rust → Clang Target Conversion ===\n");

    let targets = [
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "aarch64-apple-ios",
        "aarch64-apple-ios-sim",
        "x86_64-apple-ios",
        "aarch64-apple-ios-macabi",
        "aarch64-apple-tvos",
        "aarch64-apple-tvos-sim",
        "aarch64-apple-watchos",
        "aarch64-apple-watchos-sim",
        "armv7k-apple-watchos",
        "arm64_32-apple-watchos",
        "aarch64-apple-visionos",
        "aarch64-apple-visionos-sim",
        "aarch64-apple-driverkit",
        "x86_64-unknown-linux-gnu",   // not Apple — should be None
    ];

    println!("{:<42}  {}", "Rust triple", "Clang triple");
    println!("{}", "-".repeat(75));
    for target in targets {
        match triple::to_clang(target) {
            Some(clang) => println!("  {target:<40}  {clang}"),
            None        => println!("  {target:<40}  (not an Apple target)"),
        }
    }
}
