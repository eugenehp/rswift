fn main() {
    println!("cargo:rustc-link-lib=framework=StoreKit");

    // StoreKit 2 async API requires a Swift bridge.
    // Only compile it when the feature is enabled.
    #[cfg(feature = "storekit2")]
    {
        swift_helper_build::SwiftBridge::new("storekit_bridge")
            .file("swift/bridge.swift")
            .framework("StoreKit")
            .compile();
    }
}
