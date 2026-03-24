fn main() {
    swift_helper_build::SwiftBridge::new("quicklookui_bridge")
        .file("swift/bridge.swift")
        .framework("QuickLookUI")
        .compile();
}
