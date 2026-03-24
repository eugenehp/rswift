import Foundation
#if canImport(AlarmKit)
import AlarmKit
@_cdecl("alarmkit_swift_avail")
public func alarmkit_swift_availFn() -> Bool { true }
#else
@_cdecl("alarmkit_swift_avail")
public func alarmkit_swift_availFn() -> Bool { false }
#endif
