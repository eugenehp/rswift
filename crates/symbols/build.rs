fn main() {
    swift_helper_build::SwiftBridge::new("symbols_bridge")
        .file("swift/bridge.swift")
        .framework("Symbols")
        .compile();
}
