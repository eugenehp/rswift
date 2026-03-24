fn main() {
    swift_helper_build::SwiftBridge::new("corehid_bridge")
        .file("swift/bridge.swift")
        .framework("CoreHID")
        .compile();
}
