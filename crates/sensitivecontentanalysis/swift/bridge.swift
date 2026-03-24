import Foundation
#if canImport(SensitiveContentAnalysis)
import SensitiveContentAnalysis

@_cdecl("sca_swift_available")
public func sca_swift_availableImpl() -> Bool { true }


@available(macOS 14.0, iOS 17.0, *)
@_cdecl("sca_policy_available")
public func scaPolicyAvailable() -> Bool { true }


#else
@_cdecl("sca_swift_available")
public func sca_swift_availableImpl() -> Bool { false }
#endif
