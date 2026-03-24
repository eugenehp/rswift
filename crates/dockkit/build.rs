fn main() {
    swift_helper_build::SwiftBridge::new("dockkit_bridge")
        .file("swift/bridge.swift")
        .framework("DockKit")
        .compile();
}
