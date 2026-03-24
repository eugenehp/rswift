fn main() {
    swift_helper_build::SwiftBridge::new("translation_bridge")
        .file("swift/bridge.swift")
        .compile();
}
