import Foundation
#if canImport(ClockKit)
import ClockKit
@_cdecl("clockkit_swift_avail")
public func clockkit_swift_availFn() -> Bool { true }
#else
@_cdecl("clockkit_swift_avail")
public func clockkit_swift_availFn() -> Bool { false }
#endif
