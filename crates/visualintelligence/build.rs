fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match os.as_str() {
        "macos" => {
            // VisualIntelligence is not available on macOS — compile availability stub only.
            swift_helper_build::SwiftBridge::new("visualintelligence_bridge")
                .file("swift/bridge.swift")
                .compile();
        }
        _ => {
            swift_helper_build::SwiftBridge::new("visualintelligence_bridge")
                .file("swift/bridge.swift")
                .framework("VisualIntelligence")
                .compile();
        }
    }
}
