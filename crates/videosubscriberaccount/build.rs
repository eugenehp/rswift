fn main() {
    swift_helper_build::SwiftBridge::new("videosubscriberaccount_bridge")
        .file("swift/bridge.swift")
        .framework("VideoSubscriberAccount")
        .compile();
}
