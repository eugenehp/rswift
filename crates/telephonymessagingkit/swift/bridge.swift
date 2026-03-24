import Foundation
#if canImport(TelephonyMessagingKit)
import TelephonyMessagingKit
@_cdecl("telephonymessagingkit_swift_avail")
public func telephonymessagingkit_swift_availFn() -> Bool { true }
#else
@_cdecl("telephonymessagingkit_swift_avail")
public func telephonymessagingkit_swift_availFn() -> Bool { false }
#endif
