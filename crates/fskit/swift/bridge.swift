import Foundation
#if canImport(FSKit)
import FSKit
@_cdecl("fskit_swift_avail")
public func fskit_swift_availFn() -> Bool { true }
#else
@_cdecl("fskit_swift_avail")
public func fskit_swift_availFn() -> Bool { false }
#endif
