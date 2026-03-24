fn main() {
    swift_helper_build::SwiftBridge::new("declaredagerange_bridge")
        .file("swift/bridge.swift")
        .framework("DeclaredAgeRange")
        .compile();
}
