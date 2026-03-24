fn main() {
    swift_helper_build::SwiftBridge::new("fskit_bridge")
        .file("swift/bridge.swift")
        .framework("FSKit")
        .compile();
}
