fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // AccessorySetupKit is iOS 18+ only (not available on macOS even via Catalyst)
    if os == "ios" || os == "xros" {
        swift_helper_build::SwiftBridge::new("accessorysetupkit_bridge")
            .file("swift/bridge.swift")
            .framework("AccessorySetupKit")
            .compile();
    } else {
        swift_helper_build::SwiftBridge::new("accessorysetupkit_bridge")
            .file("swift/bridge.swift")
            .compile();
    }
}
