fn main() {
    swift_helper_build::SwiftBridge::new("quicklookthumbnailing_bridge")
        .file("swift/bridge.swift")
        .framework("QuickLookThumbnailing")
        .compile();
}
