import Foundation
#if canImport(ServicesAccountLinking)
import ServicesAccountLinking
@_cdecl("servicesaccountlinking_swift_avail")
public func servicesaccountlinking_swift_availFn() -> Bool { true }
#else
@_cdecl("servicesaccountlinking_swift_avail")
public func servicesaccountlinking_swift_availFn() -> Bool { false }
#endif
