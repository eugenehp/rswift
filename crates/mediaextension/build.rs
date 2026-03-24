fn main() {
    swift_helper_build::SwiftBridge::new("mediaextension_bridge")
        .file("swift/bridge.swift")
        .framework("MediaExtension")
        .compile();
}
