fn main() {
    swift_helper_build::SwiftBridge::new("coremedia_bridge")
        .file("swift/bridge.swift")
        .framework("CoreMedia")
        .compile();
}
