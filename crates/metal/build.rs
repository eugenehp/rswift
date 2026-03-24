fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match os.as_str() {
        "macos" | "ios" | "xros" => {}
        other => {
            println!("cargo:warning=Metal not available on {other}");
            return;
        }
    }
    swift_helper_build::SwiftBridge::new("metal_bridge")
        .file("swift/bridge.swift")
        .framework("Metal")
        .compile();
}
