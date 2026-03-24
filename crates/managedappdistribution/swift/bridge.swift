import Foundation
#if canImport(ManagedAppDistribution)
import ManagedAppDistribution
@_cdecl("managedappdistribution_swift_avail")
public func managedappdistribution_swift_availFn() -> Bool { true }
#else
@_cdecl("managedappdistribution_swift_avail")
public func managedappdistribution_swift_availFn() -> Bool { false }
#endif
