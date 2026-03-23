import AVFoundation
import Foundation
// AVFoundation — media capture, playback, and editing
// ═══════════════════════════════════════════════════════════════════════════

import AVFoundation

@_cdecl("avfoundation_available")
public func avfoundationAvailable() -> Bool { true }

@_cdecl("avfoundation_player_create")
public func avfoundationPlayerCreate(_ urlPtr: UnsafePointer<UInt8>, _ urlLen: Int) -> UnsafeMutableRawPointer {
    let urlStr = String(bytes: UnsafeBufferPointer(start: urlPtr, count: urlLen), encoding: .utf8) ?? ""
    let url: URL
    if urlStr.hasPrefix("http://") || urlStr.hasPrefix("https://") {
        url = URL(string: urlStr)!
    } else {
        url = URL(fileURLWithPath: urlStr)
    }
    let player = AVPlayer(url: url)
    return Unmanaged.passRetained(player).toOpaque()
}

@_cdecl("avfoundation_player_play")
public func avfoundationPlayerPlay(_ ptr: UnsafeMutableRawPointer) {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.play()
}

@_cdecl("avfoundation_player_pause")
public func avfoundationPlayerPause(_ ptr: UnsafeMutableRawPointer) {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.pause()
}

@_cdecl("avfoundation_player_rate")
public func avfoundationPlayerRate(_ ptr: UnsafeMutableRawPointer) -> Float {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return player.rate
}

@_cdecl("avfoundation_player_set_rate")
public func avfoundationPlayerSetRate(_ ptr: UnsafeMutableRawPointer, _ rate: Float) {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.rate = rate
}

@_cdecl("avfoundation_player_set_volume")
public func avfoundationPlayerSetVolume(_ ptr: UnsafeMutableRawPointer, _ volume: Float) {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.volume = volume
}

@_cdecl("avfoundation_player_volume")
public func avfoundationPlayerVolume(_ ptr: UnsafeMutableRawPointer) -> Float {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return player.volume
}

@_cdecl("avfoundation_player_is_muted")
public func avfoundationPlayerIsMuted(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return player.isMuted
}

@_cdecl("avfoundation_player_set_muted")
public func avfoundationPlayerSetMuted(_ ptr: UnsafeMutableRawPointer, _ muted: Bool) {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.isMuted = muted
}

@_cdecl("avfoundation_player_current_time")
public func avfoundationPlayerCurrentTime(_ ptr: UnsafeMutableRawPointer) -> Double {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return CMTimeGetSeconds(player.currentTime())
}

@_cdecl("avfoundation_player_duration")
public func avfoundationPlayerDuration(_ ptr: UnsafeMutableRawPointer) -> Double {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    guard let item = player.currentItem else { return 0 }
    let dur = item.duration
    if dur.flags.contains(.valid) && !dur.flags.contains(.indefinite) {
        return CMTimeGetSeconds(dur)
    }
    return 0
}

@_cdecl("avfoundation_player_seek")
public func avfoundationPlayerSeek(_ ptr: UnsafeMutableRawPointer, _ seconds: Double) {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.seek(to: CMTime(seconds: seconds, preferredTimescale: 600))
}

@_cdecl("avfoundation_player_status")
public func avfoundationPlayerStatus(_ ptr: UnsafeMutableRawPointer) -> Int {
    let player = Unmanaged<AVPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return player.status.rawValue
}

@_cdecl("avfoundation_player_release")
public func avfoundationPlayerRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AVPlayer>.fromOpaque(ptr).release()
}

@_cdecl("avfoundation_asset_create")
public func avfoundationAssetCreate(_ urlPtr: UnsafePointer<UInt8>, _ urlLen: Int) -> UnsafeMutableRawPointer {
    let urlStr = String(bytes: UnsafeBufferPointer(start: urlPtr, count: urlLen), encoding: .utf8) ?? ""
    let url: URL
    if urlStr.hasPrefix("http://") || urlStr.hasPrefix("https://") {
        url = URL(string: urlStr)!
    } else {
        url = URL(fileURLWithPath: urlStr)
    }
    let asset = AVURLAsset(url: url)
    return Unmanaged.passRetained(asset).toOpaque()
}

@_cdecl("avfoundation_asset_duration")
@available(macOS, deprecated: 13, message: "Use async load(.duration) in production")
public func avfoundationAssetDuration(_ ptr: UnsafeMutableRawPointer) -> Double {
    let asset = Unmanaged<AVURLAsset>.fromOpaque(ptr).takeUnretainedValue()
    return CMTimeGetSeconds(asset.duration)
}

@_cdecl("avfoundation_asset_release")
public func avfoundationAssetRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AVURLAsset>.fromOpaque(ptr).release()
}


