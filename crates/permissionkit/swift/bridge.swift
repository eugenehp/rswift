import Foundation
#if canImport(PermissionKit)
import PermissionKit
@_cdecl("permissionkit_swift_avail")
public func permissionkit_swift_availFn() -> Bool { true }
#else
@_cdecl("permissionkit_swift_avail")
public func permissionkit_swift_availFn() -> Bool { false }
#endif
