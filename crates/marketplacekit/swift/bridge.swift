import Foundation
#if canImport(MarketplaceKit)
import MarketplaceKit
@_cdecl("marketplacekit_swift_avail")
public func marketplacekit_swift_availFn() -> Bool { true }
#else
@_cdecl("marketplacekit_swift_avail")
public func marketplacekit_swift_availFn() -> Bool { false }
#endif
