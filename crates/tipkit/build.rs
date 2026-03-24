fn main() {
    swift_helper_build::SwiftBridge::new("tipkit_bridge")
        .file("swift/bridge.swift")
        .framework("TipKit")
        .compile();
}
