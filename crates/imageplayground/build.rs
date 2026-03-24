fn main() {
    swift_helper_build::SwiftBridge::new("imageplayground_bridge")
        .file("swift/bridge.swift")
        .framework("ImagePlayground")
        .compile();
}
