fn main() {
    swift_helper_build::SwiftBridge::new("automaticassessmentconfiguration_bridge")
        .file("swift/bridge.swift")
        .framework("AutomaticAssessmentConfiguration")
        .compile();
}
