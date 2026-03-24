import Foundation
#if canImport(SecureElementCredential)
import SecureElementCredential
@_cdecl("secureelementcredential_swift_avail")
public func secureelementcredential_swift_availFn() -> Bool { true }
#else
@_cdecl("secureelementcredential_swift_avail")
public func secureelementcredential_swift_availFn() -> Bool { false }
#endif
