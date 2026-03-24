fn main() {
    swift_helper_build::SwiftBridge::new("activitykit_bridge")
        .file("swift/bridge.swift")
        .framework("ActivityKit")
        .compile();
}
