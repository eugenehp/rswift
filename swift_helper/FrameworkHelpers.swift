import Foundation

// ═══════════════════════════════════════════════════════════════════════════
// Translation
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(Translation)
import Translation

@_cdecl("translation_available")
public func translationAvailable() -> Bool { true }
#endif

// ═══════════════════════════════════════════════════════════════════════════
// Spatial — 3D math types
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(Spatial)
import Spatial

@_cdecl("spatial_point3d_distance")
public func spatialPoint3dDistance(_ ax: Double, _ ay: Double, _ az: Double, _ bx: Double, _ by: Double, _ bz: Double) -> Double {
    Point3D(x: ax, y: ay, z: az).distance(to: Point3D(x: bx, y: by, z: bz))
}

@_cdecl("spatial_rotation3d_from_axis_angle")
public func spatialRotation3dFromAxisAngle(_ ax: Double, _ ay: Double, _ az: Double, _ angle: Double, _ out: UnsafeMutablePointer<Double>) {
    let q = Rotation3D(angle: Angle2D(radians: angle), axis: RotationAxis3D(x: ax, y: ay, z: az)).quaternion
    out[0] = q.vector.x; out[1] = q.vector.y; out[2] = q.vector.z; out[3] = q.vector.w
}

@_cdecl("spatial_rotation3d_from_euler")
public func spatialRotation3dFromEuler(_ p: Double, _ y: Double, _ r: Double, _ out: UnsafeMutablePointer<Double>) {
    let q = Rotation3D(eulerAngles: EulerAngles(angles: SIMD3(p, y, r), order: .xyz)).quaternion
    out[0] = q.vector.x; out[1] = q.vector.y; out[2] = q.vector.z; out[3] = q.vector.w
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// WidgetKit
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(WidgetKit)
import WidgetKit

@_cdecl("widgetkit_reload_all")
public func widgetkitReloadAll() { WidgetCenter.shared.reloadAllTimelines() }

@_cdecl("widgetkit_reload_kind")
public func widgetkitReloadKind(_ kindPtr: UnsafePointer<UInt8>, _ kindLen: Int) {
    let kind = String(bytes: UnsafeBufferPointer(start: kindPtr, count: kindLen), encoding: .utf8) ?? ""
    WidgetCenter.shared.reloadTimelines(ofKind: kind)
}

@_cdecl("widgetkit_get_configurations")
public func widgetkitGetConfigurations(
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    WidgetCenter.shared.getCurrentConfigurations { result in
        if let configs = try? result.get() {
            let json = configs.map { $0.kind }.joined(separator: ",")
            json.withCString { ptr in cb(UnsafePointer(OpaquePointer(ptr)), json.utf8.count, ud) }
        }
    }
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// AppIntents
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(AppIntents)
import AppIntents

@_cdecl("appintents_available")
public func appintentsAvailable() -> Bool { true }
#endif

// ═══════════════════════════════════════════════════════════════════════════
// ActivityKit
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(ActivityKit) && os(iOS)
import ActivityKit

@_cdecl("activitykit_available")
public func activitykitAvailable() -> Bool {
    ActivityAuthorizationInfo().areActivitiesEnabled
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// AVFAudio — audio playback, recording, and processing
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(AVFAudio)
import AVFAudio

@_cdecl("avfaudio_available")
public func avfaudioAvailable() -> Bool { true }

@_cdecl("avfaudio_engine_create")
public func avfaudioEngineCreate() -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(AVAudioEngine()).toOpaque()
}

@_cdecl("avfaudio_engine_start")
public func avfaudioEngineStart(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(ptr).takeUnretainedValue()
    do { try engine.start(); return true } catch { return false }
}

@_cdecl("avfaudio_engine_stop")
public func avfaudioEngineStop(_ ptr: UnsafeMutableRawPointer) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(ptr).takeUnretainedValue()
    engine.stop()
}

@_cdecl("avfaudio_engine_is_running")
public func avfaudioEngineIsRunning(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(ptr).takeUnretainedValue()
    return engine.isRunning
}

@_cdecl("avfaudio_engine_release")
public func avfaudioEngineRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AVAudioEngine>.fromOpaque(ptr).release()
}

@_cdecl("avfaudio_engine_output_format")
public func avfaudioEngineOutputFormat(_ ptr: UnsafeMutableRawPointer, _ sampleRate: UnsafeMutablePointer<Double>, _ channels: UnsafeMutablePointer<UInt32>) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(ptr).takeUnretainedValue()
    let fmt = engine.outputNode.outputFormat(forBus: 0)
    sampleRate.pointee = fmt.sampleRate
    channels.pointee = fmt.channelCount
}

@_cdecl("avfaudio_player_create")
public func avfaudioPlayerCreate(_ pathPtr: UnsafePointer<UInt8>, _ pathLen: Int) -> UnsafeMutableRawPointer? {
    let path = String(bytes: UnsafeBufferPointer(start: pathPtr, count: pathLen), encoding: .utf8) ?? ""
    let url = URL(fileURLWithPath: path)
    guard let player = try? AVAudioPlayer(contentsOf: url) else { return nil }
    return Unmanaged.passRetained(player).toOpaque()
}

@_cdecl("avfaudio_player_play")
public func avfaudioPlayerPlay(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return player.play()
}

@_cdecl("avfaudio_player_pause")
public func avfaudioPlayerPause(_ ptr: UnsafeMutableRawPointer) {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.pause()
}

@_cdecl("avfaudio_player_stop")
public func avfaudioPlayerStop(_ ptr: UnsafeMutableRawPointer) {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.stop()
}

@_cdecl("avfaudio_player_is_playing")
public func avfaudioPlayerIsPlaying(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return player.isPlaying
}

@_cdecl("avfaudio_player_duration")
public func avfaudioPlayerDuration(_ ptr: UnsafeMutableRawPointer) -> Double {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return player.duration
}

@_cdecl("avfaudio_player_current_time")
public func avfaudioPlayerCurrentTime(_ ptr: UnsafeMutableRawPointer) -> Double {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return player.currentTime
}

@_cdecl("avfaudio_player_set_current_time")
public func avfaudioPlayerSetCurrentTime(_ ptr: UnsafeMutableRawPointer, _ time: Double) {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.currentTime = time
}

@_cdecl("avfaudio_player_set_volume")
public func avfaudioPlayerSetVolume(_ ptr: UnsafeMutableRawPointer, _ volume: Float) {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.volume = volume
}

@_cdecl("avfaudio_player_volume")
public func avfaudioPlayerVolume(_ ptr: UnsafeMutableRawPointer) -> Float {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    return player.volume
}

@_cdecl("avfaudio_player_set_loops")
public func avfaudioPlayerSetLoops(_ ptr: UnsafeMutableRawPointer, _ loops: Int) {
    let player = Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue()
    player.numberOfLoops = loops
}

@_cdecl("avfaudio_player_release")
public func avfaudioPlayerRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).release()
}

#if os(macOS) || os(iOS) || os(tvOS) || os(watchOS) || os(visionOS)
@_cdecl("avfaudio_session_category")
public func avfaudioSessionCategory(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    #if os(macOS)
    let cat = "default"
    #else
    let cat = AVAudioSession.sharedInstance().category.rawValue
    #endif
    let data = Array(cat.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return len
}
#endif

#endif

// ═══════════════════════════════════════════════════════════════════════════
// AVFoundation — media capture, playback, and editing
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(AVFoundation) && (os(macOS) || os(iOS) || os(tvOS) || os(visionOS))
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

#endif

// ═══════════════════════════════════════════════════════════════════════════
// VideoSubscriberAccount — TV provider authentication
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(VideoSubscriberAccount) && (os(macOS) || os(iOS) || os(tvOS))
import VideoSubscriberAccount

@_cdecl("videosubscriberaccount_available")
public func videosubscriberaccountAvailable() -> Bool { true }

#if os(iOS) || os(tvOS)
@_cdecl("videosubscriberaccount_check_status")
public func videosubscriberaccountCheckStatus(
    _ cb: @convention(c) (Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let mgr = VSAccountManager()
    mgr.checkAccessStatus(options: [:]) { status, error in
        // 0 = notDetermined, 1 = restricted, 2 = denied, 3 = granted
        cb(status.rawValue, ud)
    }
}
#endif

#endif
