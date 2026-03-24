fn main() {
    swift_helper_build::SwiftBridge::new("backgroundassets_bridge")
        .file("swift/bridge.swift")
        .framework("BackgroundAssets")
        .compile();
}
