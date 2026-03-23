import Foundation
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
    Unmanaged<AVAudioEngine>.fromOpaque(ptr).takeUnretainedValue().stop()
}

@_cdecl("avfaudio_engine_is_running")
public func avfaudioEngineIsRunning(_ ptr: UnsafeMutableRawPointer) -> Bool {
    Unmanaged<AVAudioEngine>.fromOpaque(ptr).takeUnretainedValue().isRunning
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
    guard let player = try? AVAudioPlayer(contentsOf: URL(fileURLWithPath: path)) else { return nil }
    return Unmanaged.passRetained(player).toOpaque()
}

@_cdecl("avfaudio_player_play")
public func avfaudioPlayerPlay(_ ptr: UnsafeMutableRawPointer) -> Bool {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().play()
}

@_cdecl("avfaudio_player_pause")
public func avfaudioPlayerPause(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().pause()
}

@_cdecl("avfaudio_player_stop")
public func avfaudioPlayerStop(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().stop()
}

@_cdecl("avfaudio_player_is_playing")
public func avfaudioPlayerIsPlaying(_ ptr: UnsafeMutableRawPointer) -> Bool {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().isPlaying
}

@_cdecl("avfaudio_player_duration")
public func avfaudioPlayerDuration(_ ptr: UnsafeMutableRawPointer) -> Double {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().duration
}

@_cdecl("avfaudio_player_current_time")
public func avfaudioPlayerCurrentTime(_ ptr: UnsafeMutableRawPointer) -> Double {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().currentTime
}

@_cdecl("avfaudio_player_set_current_time")
public func avfaudioPlayerSetCurrentTime(_ ptr: UnsafeMutableRawPointer, _ time: Double) {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().currentTime = time
}

@_cdecl("avfaudio_player_set_volume")
public func avfaudioPlayerSetVolume(_ ptr: UnsafeMutableRawPointer, _ volume: Float) {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().volume = volume
}

@_cdecl("avfaudio_player_volume")
public func avfaudioPlayerVolume(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().volume
}

@_cdecl("avfaudio_player_set_loops")
public func avfaudioPlayerSetLoops(_ ptr: UnsafeMutableRawPointer, _ loops: Int) {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).takeUnretainedValue().numberOfLoops = loops
}

@_cdecl("avfaudio_player_release")
public func avfaudioPlayerRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AVAudioPlayer>.fromOpaque(ptr).release()
}
