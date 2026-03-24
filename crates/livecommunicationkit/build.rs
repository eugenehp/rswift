fn main() {
    swift_helper_build::SwiftBridge::new("livecommunicationkit_bridge")
        .file("swift/bridge.swift")
        .framework("LiveCommunicationKit")
        .compile();
}
