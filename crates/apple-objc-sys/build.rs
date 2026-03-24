fn main() {
    // libobjc is always present on Apple platforms.
    println!("cargo:rustc-link-lib=dylib=objc");
    // CoreFoundation for CFRelease, CFStringCreate, etc.
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}
