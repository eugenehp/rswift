fn main() {
    swift_helper_build::SwiftBridge::new("familycontrols_bridge")
        .file("swift/bridge.swift")
        .framework("FamilyControls")
        .compile();
}
