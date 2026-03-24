import Foundation
#if canImport(ShazamKit)
import ShazamKit

@_cdecl("shazamkit_swift_available")
public func shazamkitSwiftAvailFn() -> Bool { true }

@available(macOS 12.0, iOS 15.0, *)
@_cdecl("shazamkit_recognize_from_signature")
public func shazamkitRecognize(
    _ signatureData: UnsafePointer<UInt8>, _ signatureLen: Int,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    guard (try? SHSignature(dataRepresentation: Data(bytes: signatureData, count: signatureLen))) != nil else {
        cb(UnsafePointer(OpaquePointer(bitPattern: 1))!, 0, false, ud)
        return
    }
    Task {
        // Full recognition requires a live audio buffer — signature-only matching is not
        // supported in the public SHKit API; return empty result.
        cb(UnsafePointer(OpaquePointer(bitPattern: 1))!, 0, false, ud)
    }
}

@available(macOS 12.0, iOS 15.0, *)
@_cdecl("shazamkit_is_catalog_available")
public func shazamkitIsCatalogAvailable() -> Bool {
    return true
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("shazamkit_library_item_count")
public func shazamkitLibraryItemCount(
    _ cb: @convention(c) (Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    Task {
        let library = SHLibrary.default
        // SHLibrary.items is @MainActor-isolated — dispatch to main actor
        let count = await MainActor.run { library.items.count }
        cb(count, true, ud)
    }
}

#else
@_cdecl("shazamkit_swift_available")
public func shazamkitSwiftAvailFn() -> Bool { false }
#endif
