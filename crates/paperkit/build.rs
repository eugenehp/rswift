fn main() {
    swift_helper_build::SwiftBridge::new("paperkit_bridge")
        .file("swift/bridge.swift")
        .framework("PaperKit")
        .compile();
}
