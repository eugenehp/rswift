import Foundation
#if canImport(VisionKit)
import VisionKit

@_cdecl("visionkit_swift_available")
public func visionkit_swift_availableImpl() -> Bool { true }


@available(macOS 14.0, iOS 16.0, *)
@_cdecl("visionkit_image_analyzer_available")
public func visionkitImageAnalyzerAvailable() -> Bool { true }


#else
@_cdecl("visionkit_swift_available")
public func visionkit_swift_availableImpl() -> Bool { false }
#endif
