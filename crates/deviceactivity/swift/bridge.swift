import Foundation
#if canImport(DeviceActivity)
import DeviceActivity

@_cdecl("da_swift_available")
public func da_swift_availableImpl() -> Bool { true }


@available(macOS 14.0, iOS 15.0, *)
@_cdecl("da_monitor_available")
public func daMonitorAvailable() -> Bool { true }


#else
@_cdecl("da_swift_available")
public func da_swift_availableImpl() -> Bool { false }
#endif
