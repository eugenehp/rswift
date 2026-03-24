fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // HomeKit is Catalyst/iOS-only — cannot link into a native macOS binary.
    let mut bridge = swift_helper_build::SwiftBridge::new("homekit_bridge")
        .file("swift/bridge.swift");
    if os != "macos" {
        bridge = bridge.framework("HomeKit");
    }
    bridge.compile();
}
