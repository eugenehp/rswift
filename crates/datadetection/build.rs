fn main() {
    swift_helper_build::SwiftBridge::new("datadetection_bridge")
        .file("swift/bridge.swift")
        .framework("DataDetection")
        .compile();
}
