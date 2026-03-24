fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // AdAttributionKit requires iOS/Catalyst — cannot link into a native macOS binary.
    let mut bridge = swift_helper_build::SwiftBridge::new("adattributionkit_bridge")
        .file("swift/bridge.swift");
    if os != "macos" {
        bridge = bridge.framework("AdAttributionKit");
    }
    bridge.compile();
}
