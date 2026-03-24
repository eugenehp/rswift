import Foundation
#if canImport(AdAttributionKit)
import AdAttributionKit
@_cdecl("adattributionkit_swift_avail")
public func adattributionkit_swift_availFn() -> Bool { true }
#else
@_cdecl("adattributionkit_swift_avail")
public func adattributionkit_swift_availFn() -> Bool { false }
#endif
