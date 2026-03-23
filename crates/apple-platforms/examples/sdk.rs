use apple_platforms::triple::SDK;

pub fn main() {
    let rust_target = "aarch64-apple-visionos";
    let sdk = SDK::target_to_sdk(rust_target).unwrap();

    println!("Converted from {rust_target} to {sdk}");
    // Converted from aarch64-apple-visionos to arm64-apple-xros
}
