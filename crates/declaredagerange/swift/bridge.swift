import Foundation
#if canImport(DeclaredAgeRange)
import DeclaredAgeRange
@_cdecl("declaredagerange_swift_avail")
public func declaredagerange_swift_availFn() -> Bool { true }
#else
@_cdecl("declaredagerange_swift_avail")
public func declaredagerange_swift_availFn() -> Bool { false }
#endif
