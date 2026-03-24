import Foundation
#if canImport(BrowserKit)
import BrowserKit
@_cdecl("browserkit_swift_avail")
public func browserkit_swift_availFn() -> Bool { true }
#else
@_cdecl("browserkit_swift_avail")
public func browserkit_swift_availFn() -> Bool { false }
#endif
