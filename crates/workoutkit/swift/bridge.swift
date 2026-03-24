import Foundation
#if canImport(WorkoutKit)
import WorkoutKit
@_cdecl("workoutkit_swift_avail")
public func workoutkit_swift_availFn() -> Bool { true }
#else
@_cdecl("workoutkit_swift_avail")
public func workoutkit_swift_availFn() -> Bool { false }
#endif
