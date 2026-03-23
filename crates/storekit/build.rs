fn main() {
    swift_helper_build::SwiftBridge::new("storekit_bridge")
        .file("swift/bridge.swift")
        .framework("StoreKit")
        .compile();
}
