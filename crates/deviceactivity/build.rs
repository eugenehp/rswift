fn main() {
    swift_helper_build::SwiftBridge::new("deviceactivity_bridge")
        .file("swift/bridge.swift")
        .framework("DeviceActivity")
        .compile();
}
