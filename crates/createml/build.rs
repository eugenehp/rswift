fn main() {
    swift_helper_build::SwiftBridge::new("createml_bridge")
        .file("swift/bridge.swift")
        .framework("CreateML")
        .compile();
}
