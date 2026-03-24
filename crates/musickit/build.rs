fn main() {
    swift_helper_build::SwiftBridge::new("musickit_bridge")
        .file("swift/bridge.swift")
        .framework("MusicKit")
        .compile();
}
