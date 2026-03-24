import Foundation
#if canImport(AccessoryTransportExtension)
import AccessoryTransportExtension
@_cdecl("accessorytransportextension_swift_avail")
public func accessorytransportextension_swift_availFn() -> Bool { true }
#else
@_cdecl("accessorytransportextension_swift_avail")
public func accessorytransportextension_swift_availFn() -> Bool { false }
#endif
