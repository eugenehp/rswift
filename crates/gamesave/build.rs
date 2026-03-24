fn main() {
    swift_helper_build::SwiftBridge::new("gamesave_bridge")
        .file("swift/bridge.swift")
        .framework("GameSave")
        .compile();
}
