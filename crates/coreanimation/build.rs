fn main() {
    swift_helper_build::SwiftBridge::new("coreanimation_bridge")
        .file("swift/bridge.swift")
        .framework("QuartzCore")
        .compile();
}
