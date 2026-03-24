fn main() {
    swift_helper_build::SwiftBridge::new("visionkit_bridge")
        .file("swift/bridge.swift")
        .framework("VisionKit")
        .compile();
}
