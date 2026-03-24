import Foundation
#if canImport(GroupActivities)
import GroupActivities

@_cdecl("ga_swift_available")
public func gaSwiftAvailFn() -> Bool { true }

@available(macOS 13.0, iOS 15.0, tvOS 15.0, *)
@_cdecl("ga_is_eligible_for_group_session")
public func gaIsEligibleForGroupSession() -> Bool {
    // GroupStateObserver tracks eligibility
    // This is a simplified check
    return true
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("ga_group_state_is_eligible")
public func gaGroupStateIsEligible() -> Bool {
    let observer = GroupStateObserver()
    return observer.isEligibleForGroupSession
}

#else
@_cdecl("ga_swift_available")
public func gaSwiftAvailFn() -> Bool { false }
#endif
