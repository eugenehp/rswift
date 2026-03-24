fn main() {
    swift_helper_build::SwiftBridge::new("sensitivecontentanalysis_bridge")
        .file("swift/bridge.swift")
        .framework("SensitiveContentAnalysis")
        .compile();
}
