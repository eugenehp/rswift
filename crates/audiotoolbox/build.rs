fn main() {
    // Pure C framework — no Swift bridge, no shims, just link.
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}
