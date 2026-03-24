import Foundation
#if canImport(Matter)
import Matter
@_cdecl("matter_swift_avail")
public func matter_swift_availFn() -> Bool { true }
#else
@_cdecl("matter_swift_avail")
public func matter_swift_availFn() -> Bool { false }
#endif
