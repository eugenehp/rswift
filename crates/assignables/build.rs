fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // Assignables requires iOS/Catalyst — cannot link into a native macOS binary.
    let mut bridge = swift_helper_build::SwiftBridge::new("assignables_bridge")
        .file("swift/bridge.swift");
    if os != "macos" {
        bridge = bridge.framework("Assignables");
    }
    bridge.compile();
}
