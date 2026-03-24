import Foundation
#if canImport(ContactProvider)
import ContactProvider
@_cdecl("contactprovider_swift_avail")
public func contactprovider_swift_availFn() -> Bool { true }
#else
@_cdecl("contactprovider_swift_avail")
public func contactprovider_swift_availFn() -> Bool { false }
#endif
