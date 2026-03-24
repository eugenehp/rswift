import Foundation
#if canImport(ImmersiveMediaSupport)
import ImmersiveMediaSupport
@_cdecl("immersivemediasupport_swift_avail")
public func immersivemediasupport_swift_availFn() -> Bool { true }
#else
@_cdecl("immersivemediasupport_swift_avail")
public func immersivemediasupport_swift_availFn() -> Bool { false }
#endif
