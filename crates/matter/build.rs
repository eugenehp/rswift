fn main() {
    swift_helper_build::SwiftBridge::new("matter_bridge")
        .file("swift/bridge.swift")
        .framework("Matter")
        .compile();
}
