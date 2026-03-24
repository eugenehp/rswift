import Foundation
#if canImport(ARKit)
import ARKit
@_cdecl("arkit_swift_avail")
public func arkit_swift_availFn() -> Bool { true }
#else
@_cdecl("arkit_swift_avail")
public func arkit_swift_availFn() -> Bool { false }
#endif
