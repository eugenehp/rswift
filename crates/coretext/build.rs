fn main() {
    println!("cargo:rustc-link-lib=framework=CoreText");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
}
