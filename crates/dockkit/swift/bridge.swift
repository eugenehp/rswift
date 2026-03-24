import Foundation
#if canImport(DockKit)
import DockKit
@_cdecl("dockkit_swift_avail")
public func dockkit_swift_availFn() -> Bool { true }
#else
@_cdecl("dockkit_swift_avail")
public func dockkit_swift_availFn() -> Bool { false }
#endif
