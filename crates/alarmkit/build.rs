fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // AlarmKit is Catalyst/iOS-only — cannot link into a native macOS dylib.
    let mut bridge = swift_helper_build::SwiftBridge::new("alarmkit_bridge")
        .file("swift/bridge.swift");
    if os != "macos" {
        bridge = bridge.framework("AlarmKit");
    }
    bridge.compile();
}
