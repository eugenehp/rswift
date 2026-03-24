import Foundation
#if canImport(AppIntents)
import AppIntents

@_cdecl("appintents_swift_available")
public func appintents_swift_availableImpl() -> Bool { true }


@available(macOS 13.0, iOS 16.0, *)
@_cdecl("appintents_available_check")
public func appintentsAvailableCheck() -> Bool { true }


#else
@_cdecl("appintents_swift_available")
public func appintents_swift_availableImpl() -> Bool { false }
#endif
