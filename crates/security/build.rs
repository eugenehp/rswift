fn main() {
    swift_helper_build::SwiftBridge::new("security_bridge")
        .file("swift/bridge.swift")
        .framework("Security")
        .compile();
}
