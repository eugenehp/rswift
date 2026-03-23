fn main() {
    swift_helper_build::SwiftBridge::new("avfaudio_bridge")
        .file("swift/bridge.swift")
        .framework("AVFAudio")
        .compile();
}
