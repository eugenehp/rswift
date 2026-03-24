fn main() {
    swift_helper_build::SwiftBridge::new("arkit_bridge")
        .file("swift/bridge.swift")
        .framework("ARKit")
        .compile();
}
