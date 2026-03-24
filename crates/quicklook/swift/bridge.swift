import Foundation
#if canImport(QuickLook)
import QuickLook

@_cdecl("quicklook_swift_available")
public func quicklook_swift_availableImpl() -> Bool { true }


@_cdecl("quicklook_preview_available")
public func quicklookPreviewAvailable() -> Bool { true }


#else
@_cdecl("quicklook_swift_available")
public func quicklook_swift_availableImpl() -> Bool { false }
#endif
