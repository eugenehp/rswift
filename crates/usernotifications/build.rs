fn main() {
    swift_helper_build::SwiftBridge::new("usernotifications_bridge")
        .file("swift/bridge.swift")
        .framework("UserNotifications")
        .compile();
}
