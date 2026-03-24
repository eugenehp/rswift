fn main() {
    swift_helper_build::SwiftBridge::new("telephonymessagingkit_bridge")
        .file("swift/bridge.swift")
        .framework("TelephonyMessagingKit")
        .compile();
}
