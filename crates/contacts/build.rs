fn main() {
    swift_helper_build::SwiftBridge::new("contacts_bridge")
        .file("swift/bridge.swift")
        .framework("Contacts")
        .compile();
}
