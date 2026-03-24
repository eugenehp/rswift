fn main() {
    swift_helper_build::SwiftBridge::new("permissionkit_bridge")
        .file("swift/bridge.swift")
        .framework("PermissionKit")
        .compile();
}
