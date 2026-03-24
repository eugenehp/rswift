import Foundation
#if canImport(MediaAccessibility)
import MediaAccessibility
@_cdecl("mediaaccessibility_swift_avail")
public func mediaaccessibility_swift_availFn() -> Bool { true }
#else
@_cdecl("mediaaccessibility_swift_avail")
public func mediaaccessibility_swift_availFn() -> Bool { false }
#endif
