fn main() {
    swift_helper_build::SwiftBridge::new("financekit_bridge")
        .file("swift/bridge.swift")
        .framework("FinanceKit")
        .compile();
}
