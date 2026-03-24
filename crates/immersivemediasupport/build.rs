fn main() {
    swift_helper_build::SwiftBridge::new("immersivemediasupport_bridge")
        .file("swift/bridge.swift")
        .framework("ImmersiveMediaSupport")
        .compile();
}
