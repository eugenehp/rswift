fn main() {
    swift_helper_build::SwiftBridge::new("quicklook_bridge")
        .file("swift/bridge.swift")
        .framework("QuickLook")
        .compile();
}
