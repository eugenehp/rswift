import Foundation
#if canImport(ImagePlayground)
import ImagePlayground

@_cdecl("imageplayground_swift_available")
public func imageplayground_swift_availableImpl() -> Bool { true }


@available(macOS 15.4, iOS 18.2, *)
@_cdecl("imageplayground_available_check")
public func imageplaygroundAvailableCheck() -> Bool { true }


#else
@_cdecl("imageplayground_swift_available")
public func imageplayground_swift_availableImpl() -> Bool { false }
#endif
