fn main() {
    swift_helper_build::SwiftBridge::new("foundation_bridge")
        .file("swift/bridge.swift")
        .compile();
}
