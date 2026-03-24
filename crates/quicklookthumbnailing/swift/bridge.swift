import Foundation
#if canImport(QuickLookThumbnailing)
import QuickLookThumbnailing

@_cdecl("qlt_swift_available")
public func qlt_swift_availableImpl() -> Bool { true }


@_cdecl("qlt_generator_available")
public func qltGeneratorAvailable() -> Bool { true }


#else
@_cdecl("qlt_swift_available")
public func qlt_swift_availableImpl() -> Bool { false }
#endif
