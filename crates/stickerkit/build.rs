fn main() {
    swift_helper_build::SwiftBridge::new("stickerkit_bridge")
        .file("swift/bridge.swift")
        .framework("StickerKit")
        .compile();
}
