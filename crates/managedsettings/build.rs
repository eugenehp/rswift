fn main() {
    swift_helper_build::SwiftBridge::new("managedsettings_bridge")
        .file("swift/bridge.swift")
        .framework("ManagedSettings")
        .compile();
}
