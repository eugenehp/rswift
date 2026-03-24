import Foundation
#if canImport(CompositorServices)
import CompositorServices
@_cdecl("compositorservices_swift_avail")
public func compositorservices_swift_availFn() -> Bool { true }
#else
@_cdecl("compositorservices_swift_avail")
public func compositorservices_swift_availFn() -> Bool { false }
#endif
