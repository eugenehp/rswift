fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // CoreNFC is iOS-only
    let mut bridge = swift_helper_build::SwiftBridge::new("corenfc_bridge")
        .file("swift/bridge.swift");
    if os == "ios" {
        bridge = bridge.framework("CoreNFC");
    }
    bridge.compile();
}
