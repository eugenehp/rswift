fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match os.as_str() {
        "macos" => {
            // JournalingSuggestions is not available on macOS — compile availability stub only.
            swift_helper_build::SwiftBridge::new("journalingsuggestions_bridge")
                .file("swift/bridge.swift")
                .compile();
        }
        _ => {
            swift_helper_build::SwiftBridge::new("journalingsuggestions_bridge")
                .file("swift/bridge.swift")
                .framework("JournalingSuggestions")
                .compile();
        }
    }
}
