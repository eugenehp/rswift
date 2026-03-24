fn main() {
    swift_helper_build::SwiftBridge::new("browserenginekit_bridge")
        .file("swift/bridge.swift")
        .framework("BrowserEngineKit")
        .compile();
}
