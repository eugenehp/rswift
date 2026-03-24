import Foundation
#if canImport(CoreHID)
import CoreHID
@_cdecl("corehid_swift_avail")
public func corehid_swift_availFn() -> Bool { true }
#else
@_cdecl("corehid_swift_avail")
public func corehid_swift_availFn() -> Bool { false }
#endif
