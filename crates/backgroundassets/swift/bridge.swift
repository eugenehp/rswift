import Foundation
#if canImport(BackgroundAssets)
import BackgroundAssets

@_cdecl("ba_swift_available")
public func ba_swift_availableImpl() -> Bool { true }


@_cdecl("ba_manager_available")
public func baManagerAvailable() -> Bool { true }


#else
@_cdecl("ba_swift_available")
public func ba_swift_availableImpl() -> Bool { false }
#endif
