fn main() {
    swift_helper_build::SwiftBridge::new("wifiaware_bridge")
        .file("swift/bridge.swift")
        .framework("WiFiAware")
        .compile();
}
