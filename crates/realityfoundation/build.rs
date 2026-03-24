fn main() {
    swift_helper_build::SwiftBridge::new("realityfoundation_bridge")
        .file("swift/bridge.swift")
        .framework("RealityKit")
        .compile();
}
