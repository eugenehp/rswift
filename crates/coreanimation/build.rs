fn main() {
    // CACurrentMediaTime is pure C. CATransaction uses ObjC msg dispatch
    // via libobjc which is always linked on Apple platforms.
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=dylib=objc");
}
