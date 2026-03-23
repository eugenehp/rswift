import Foundation
import AudioToolbox

// ═══════════════════════════════════════════════════════════════════════════
// AudioToolbox — system sounds, audio services
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("audiotoolbox_available")
public func audiotoolboxAvailable() -> Bool { true }

@_cdecl("audiotoolbox_play_system_sound")
public func audiotoolboxPlaySystemSound(_ soundID: UInt32) {
    AudioServicesPlaySystemSound(SystemSoundID(soundID))
}

@_cdecl("audiotoolbox_play_alert_sound")
public func audiotoolboxPlayAlertSound(_ soundID: UInt32) {
    AudioServicesPlayAlertSound(SystemSoundID(soundID))
}

@_cdecl("audiotoolbox_create_system_sound")
public func audiotoolboxCreateSystemSound(_ pathPtr: UnsafePointer<UInt8>, _ pathLen: Int,
                                            _ outID: UnsafeMutablePointer<UInt32>) -> Int32 {
    let path = String(bytes: UnsafeBufferPointer(start: pathPtr, count: pathLen), encoding: .utf8)!
    let url = URL(fileURLWithPath: path) as CFURL
    var soundID: SystemSoundID = 0
    let status = AudioServicesCreateSystemSoundID(url, &soundID)
    outID.pointee = soundID
    return status
}

@_cdecl("audiotoolbox_dispose_system_sound")
public func audiotoolboxDisposeSystemSound(_ soundID: UInt32) -> Int32 {
    AudioServicesDisposeSystemSoundID(SystemSoundID(soundID))
}

// Well-known system sound IDs
// 1000 = new mail, 1001 = mail sent, 1003 = sms received
// 1004 = calendar alert, 1005 = lock, 1006 = key press
// 1007 = key press (delete), 1016 = key press (modifier)
// 1057 = screen capture, 1108 = photo shutter
// 1306 = begin recording, 1307 = end recording
// 4095 = vibrate (iOS)
