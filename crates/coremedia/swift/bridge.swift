import Foundation
import CoreMedia

// ═══════════════════════════════════════════════════════════════════════════
// CoreMedia — CMTime, media timing
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("coremedia_available")
public func coremediaAvailable() -> Bool { true }

// ── CMTime ──────────────────────────────────────────────────────────────────

@_cdecl("coremedia_time_make")
public func coremediaTimeMake(_ value: Int64, _ timescale: Int32,
                                _ outValue: UnsafeMutablePointer<Int64>,
                                _ outTimescale: UnsafeMutablePointer<Int32>) {
    let t = CMTime(value: value, timescale: timescale)
    outValue.pointee = t.value
    outTimescale.pointee = t.timescale
}

@_cdecl("coremedia_time_make_with_seconds")
public func coremediaTimeMakeWithSeconds(_ seconds: Double, _ timescale: Int32,
                                          _ outValue: UnsafeMutablePointer<Int64>,
                                          _ outTimescale: UnsafeMutablePointer<Int32>) {
    let t = CMTimeMakeWithSeconds(seconds, preferredTimescale: timescale)
    outValue.pointee = t.value
    outTimescale.pointee = t.timescale
}

@_cdecl("coremedia_time_get_seconds")
public func coremediaTimeGetSeconds(_ value: Int64, _ timescale: Int32) -> Double {
    CMTimeGetSeconds(CMTime(value: value, timescale: timescale))
}

@_cdecl("coremedia_time_add")
public func coremediaTimeAdd(_ v1: Int64, _ ts1: Int32, _ v2: Int64, _ ts2: Int32,
                               _ outValue: UnsafeMutablePointer<Int64>,
                               _ outTimescale: UnsafeMutablePointer<Int32>) {
    let t = CMTimeAdd(CMTime(value: v1, timescale: ts1), CMTime(value: v2, timescale: ts2))
    outValue.pointee = t.value
    outTimescale.pointee = t.timescale
}

@_cdecl("coremedia_time_subtract")
public func coremediaTimeSubtract(_ v1: Int64, _ ts1: Int32, _ v2: Int64, _ ts2: Int32,
                                    _ outValue: UnsafeMutablePointer<Int64>,
                                    _ outTimescale: UnsafeMutablePointer<Int32>) {
    let t = CMTimeSubtract(CMTime(value: v1, timescale: ts1), CMTime(value: v2, timescale: ts2))
    outValue.pointee = t.value
    outTimescale.pointee = t.timescale
}

@_cdecl("coremedia_time_compare")
public func coremediaTimeCompare(_ v1: Int64, _ ts1: Int32, _ v2: Int64, _ ts2: Int32) -> Int32 {
    CMTimeCompare(CMTime(value: v1, timescale: ts1), CMTime(value: v2, timescale: ts2))
}

@_cdecl("coremedia_time_is_valid")
public func coremediaTimeIsValid(_ value: Int64, _ timescale: Int32) -> Bool {
    CMTime(value: value, timescale: timescale).isValid
}

@_cdecl("coremedia_time_is_indefinite")
public func coremediaTimeIsIndefinite(_ value: Int64, _ timescale: Int32) -> Bool {
    CMTime(value: value, timescale: timescale).isIndefinite
}
