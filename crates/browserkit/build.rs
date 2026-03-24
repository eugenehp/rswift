fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // BrowserKit is Catalyst/iOS-only — cannot link into a native macOS dylib.
    let mut bridge = swift_helper_build::SwiftBridge::new("browserkit_bridge")
        .file("swift/bridge.swift");
    if os != "macos" {
        bridge = bridge.framework("BrowserKit");
    }
    bridge.compile();
}
