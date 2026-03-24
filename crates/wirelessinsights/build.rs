fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // Framework only exists on iOS/visionOS (not macOS) — compile stub on macOS
    // but skip framework linking; the Swift source uses #if canImport guards.
    let needs_framework = matches!(os.as_str(), "tvos" | "ios");
    let mut bridge = swift_helper_build::SwiftBridge::new("wirelessinsights_bridge")
        .file("swift/bridge.swift");
    if needs_framework {
        bridge = bridge.framework("");
    }
    bridge.compile();
}
