use apple_platforms::triple::Triple;

pub fn main() {
    let rust_target = "aarch64-apple-visionos";
    let clang_target = Triple::target_to_clang_target(rust_target);

    println!("Converted from {rust_target} to {clang_target}");
    // Converted from aarch64-apple-visionos to arm64-apple-xros
}
