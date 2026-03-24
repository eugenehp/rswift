fn main() {
    swift_helper_build::SwiftBridge::new("weatherkit_bridge")
        .file("swift/bridge.swift")
        .framework("WeatherKit")
        .framework("CoreLocation")
        .compile();
}
