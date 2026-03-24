fn main() {
    swift_helper_build::SwiftBridge::new("appintents_bridge")
        .file("swift/bridge.swift")
        .framework("AppIntents")
        .compile();
}
