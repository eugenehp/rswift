fn main() {
    swift_helper_build::SwiftBridge::new("compositorservices_bridge")
        .file("swift/bridge.swift")
        .framework("CompositorServices")
        .compile();
}
