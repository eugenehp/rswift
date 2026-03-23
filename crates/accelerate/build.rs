fn main() {
    swift_helper_build::SwiftBridge::new("accelerate_bridge")
        .file("swift/bridge.swift")
        .framework("Accelerate")
        .compile();
}
