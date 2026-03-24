import Foundation
#if canImport(ActivityKit)
import ActivityKit

@_cdecl("activitykit_swift_available")
public func activitykit_swift_availableImpl() -> Bool { true }


@available(macOS 14.0, iOS 16.1, *)
@_cdecl("activitykit_activities_available")
public func activitykitActivitiesAvailable() -> Bool { true }


#else
@_cdecl("activitykit_swift_available")
public func activitykit_swift_availableImpl() -> Bool { false }
#endif
