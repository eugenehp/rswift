fn main() {
    swift_helper_build::SwiftBridge::new("swiftdata_bridge")
        .file("swift/bridge.swift")
        .framework("SwiftData")
        .compile();
}
