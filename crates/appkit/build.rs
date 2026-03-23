fn main() {
    swift_helper_build::SwiftBridge::new("appkit_bridge")
        .file("swift/bridge.swift")
        .framework("AppKit")
        .compile();
}
