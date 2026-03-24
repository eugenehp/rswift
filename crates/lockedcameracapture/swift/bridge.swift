import Foundation
#if canImport(LockedCameraCapture)
import LockedCameraCapture
@_cdecl("lockedcameracapture_swift_avail")
public func lockedcameracapture_swift_availFn() -> Bool { true }
#else
@_cdecl("lockedcameracapture_swift_avail")
public func lockedcameracapture_swift_availFn() -> Bool { false }
#endif
