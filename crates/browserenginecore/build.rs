fn main() {
    swift_helper_build::SwiftBridge::new("browserenginecore_bridge")
        .file("swift/bridge.swift")
        .framework("BrowserEngineCore")
        .compile();
}
