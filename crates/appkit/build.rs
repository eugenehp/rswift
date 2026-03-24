fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match os.as_str() {
        "macos" => println!("cargo:rustc-link-lib=framework=AppKit"),
        "ios" | "tvos" | "xros" => println!("cargo:rustc-link-lib=framework=UIKit"),
        _ => {}
    }
    println!("cargo:rustc-link-lib=framework=Foundation");
}
