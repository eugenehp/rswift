fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match os.as_str() {
        "macos" | "ios" | "xros" => {}
        other => {
            println!("cargo:warning=RealityKit not available on {other}; bridge will not be compiled");
            return;
        }
    }

    swift_helper_build::SwiftBridge::new("realitykit_bridge")
        .file("swift/bridge.swift")
        .framework("RealityKit")
        .compile();
}
