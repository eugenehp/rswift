fn main() {
    // Pure framework link — no cc, no .m, no swiftc.
    // ObjC dispatch happens in Rust via apple-objc-sys.
    println!("cargo:rustc-link-lib=framework=Foundation");
}
