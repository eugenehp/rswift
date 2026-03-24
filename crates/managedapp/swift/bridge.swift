import Foundation
#if canImport(ManagedApp)
import ManagedApp
@_cdecl("managedapp_swift_avail")
public func managedapp_swift_availFn() -> Bool { true }
#else
@_cdecl("managedapp_swift_avail")
public func managedapp_swift_availFn() -> Bool { false }
#endif
