import Foundation
#if canImport(StickerKit)
import StickerKit
@_cdecl("stickerkit_swift_avail")
public func stickerkit_swift_availFn() -> Bool { true }
#else
@_cdecl("stickerkit_swift_avail")
public func stickerkit_swift_availFn() -> Bool { false }
#endif
