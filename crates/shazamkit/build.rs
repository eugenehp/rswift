fn main() {
    swift_helper_build::SwiftBridge::new("shazamkit_bridge")
        .file("swift/bridge.swift")
        .framework("ShazamKit")
        .compile();
}
