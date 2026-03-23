fn main() {
    swift_helper_build::SwiftBridge::new("audiotoolbox_bridge")
        .file("swift/bridge.swift")
        .framework("AudioToolbox")
        .compile();
}
