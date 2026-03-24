import Foundation
#if canImport(FamilyControls)
import FamilyControls

@_cdecl("fc_swift_available")
public func fc_swift_availableImpl() -> Bool { true }


@available(macOS 14.0, iOS 15.0, *)
@_cdecl("fc_authorization_center_available")
public func fcAuthorizationCenterAvailable() -> Bool { true }


#else
@_cdecl("fc_swift_available")
public func fc_swift_availableImpl() -> Bool { false }
#endif
