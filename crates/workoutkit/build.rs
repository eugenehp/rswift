fn main() {
    swift_helper_build::SwiftBridge::new("workoutkit_bridge")
        .file("swift/bridge.swift")
        .framework("WorkoutKit")
        .compile();
}
