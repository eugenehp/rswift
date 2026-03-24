import Foundation
#if canImport(CarKey)
import CarKey
@_cdecl("carkey_swift_avail")
public func carkey_swift_availFn() -> Bool { true }
#else
@_cdecl("carkey_swift_avail")
public func carkey_swift_availFn() -> Bool { false }
#endif
