fn main() {
    swift_helper_build::SwiftBridge::new("relevancekit_bridge")
        .file("swift/bridge.swift")
        .framework("RelevanceKit")
        .compile();
}
