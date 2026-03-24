fn main() {
    swift_helper_build::SwiftBridge::new("devicediscoveryextension_bridge")
        .file("swift/bridge.swift")
        .framework("DeviceDiscoveryExtension")
        .compile();
}
