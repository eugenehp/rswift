fn main() {
    swift_helper_build::SwiftBridge::new("mediaaccessibility_bridge")
        .file("swift/bridge.swift")
        .framework("MediaAccessibility")
        .compile();
}
