fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // Messages is Catalyst/iOS-only — cannot link into a native macOS binary.
    let mut bridge = swift_helper_build::SwiftBridge::new("messages_bridge")
        .file("swift/bridge.swift");
    if os != "macos" {
        bridge = bridge.framework("Messages");
    }
    bridge.compile();
}
