fn main() {
    swift_helper_build::SwiftBridge::new("managedappdistribution_bridge")
        .file("swift/bridge.swift")
        .framework("ManagedAppDistribution")
        .compile();
}
