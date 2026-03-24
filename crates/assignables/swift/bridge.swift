import Foundation
#if canImport(Assignables)
import Assignables
@_cdecl("assignables_swift_avail")
public func assignables_swift_availFn() -> Bool { true }
#else
@_cdecl("assignables_swift_avail")
public func assignables_swift_availFn() -> Bool { false }
#endif
