fn main() {
    swift_helper_build::SwiftBridge::new("groupactivities_bridge")
        .file("swift/bridge.swift")
        .framework("GroupActivities")
        .compile();
}
