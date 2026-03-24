fn main() {
    swift_helper_build::SwiftBridge::new("carkey_bridge")
        .file("swift/bridge.swift")
        .framework("CarKey")
        .compile();
}
