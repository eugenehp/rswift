fn main() {
    swift_helper_build::SwiftBridge::new("ituneslibrary_bridge")
        .file("swift/bridge.swift")
        .framework("iTunesLibrary")
        .compile();
}
