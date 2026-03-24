import Foundation
#if canImport(ManagedSettings)
import ManagedSettings

@_cdecl("ms_swift_available")
public func ms_swift_availableImpl() -> Bool { true }


@available(macOS 14.0, iOS 15.0, *)
@_cdecl("ms_store_available")
public func msStoreAvailable() -> Bool { true }


#else
@_cdecl("ms_swift_available")
public func ms_swift_availableImpl() -> Bool { false }
#endif
