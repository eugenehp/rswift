import Foundation
#if canImport(RoomPlan)
import RoomPlan
@_cdecl("roomplan_swift_avail")
public func roomplan_swift_availFn() -> Bool { true }
#else
@_cdecl("roomplan_swift_avail")
public func roomplan_swift_availFn() -> Bool { false }
#endif
