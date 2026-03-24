fn main() {
    swift_helper_build::SwiftBridge::new("coretransferable_bridge")
        .file("swift/bridge.swift")
        .framework("CoreTransferable")
        .compile();
}
