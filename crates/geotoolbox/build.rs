fn main() {
    swift_helper_build::SwiftBridge::new("geotoolbox_bridge")
        .file("swift/bridge.swift")
        .framework("GeoToolbox")
        .compile();
}
