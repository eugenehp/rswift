import Foundation
#if canImport(HomeKit)
import HomeKit
@_cdecl("homekit_swift_avail")
public func homekit_swift_availFn() -> Bool { true }
#else
@_cdecl("homekit_swift_avail")
public func homekit_swift_availFn() -> Bool { false }
#endif
