import Foundation
#if canImport(MusicKit)
import MusicKit

@_cdecl("musickit_available")
public func musickitAvailable() -> Bool { true }

@available(macOS 12.0, iOS 15.0, *)
@_cdecl("musickit_authorization_status")
public func musickitAuthorizationStatus() -> Int {
    // MusicAuthorization.Status has no Int rawValue; map manually
    switch MusicAuthorization.currentStatus {
    case .authorized:  return 1
    case .denied:      return 2
    case .restricted:  return 3
    default:           return 0   // .notDetermined
    }
}

@available(macOS 12.0, iOS 15.0, *)
@_cdecl("musickit_search_catalog")
public func musickitSearchCatalog(
    _ queryPtr: UnsafePointer<UInt8>, _ queryLen: Int,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let query = String(bytes: UnsafeBufferPointer(start: queryPtr, count: queryLen), encoding: .utf8) ?? ""
    Task {
        do {
            var request = MusicCatalogSearchRequest(term: query, types: [Song.self])
            request.limit = 10
            let response = try await request.response()
            var lines: [String] = []
            for song in response.songs {
                let title = esc(song.title)
                let artist = esc(song.artistName)
                let dur = song.duration ?? 0
                lines.append("{\"title\":\"\(title)\",\"artist\":\"\(artist)\",\"duration\":\(dur)}")
            }
            let result = "[" + lines.joined(separator: ",") + "]"
            result.withCString { ptr in
                cb(UnsafePointer(OpaquePointer(ptr)), result.utf8.count, true, ud)
            }
        } catch {
            cb(UnsafePointer(OpaquePointer(bitPattern: 1))!, 0, false, ud)
        }
    }
}

private func esc(_ s: String) -> String {
    s.replacingOccurrences(of: "\\", with: "\\\\")
     .replacingOccurrences(of: "\"", with: "\\\"")
}

#else
@_cdecl("musickit_available")
public func musickitAvailable() -> Bool { false }
#endif
