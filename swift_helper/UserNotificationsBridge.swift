import Foundation
import UserNotifications

// ═══════════════════════════════════════════════════════════════════════════
// UserNotifications — local notification scheduling
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("usernotifications_available")
public func usernotificationsAvailable() -> Bool { true }

private func makeString(_ ptr: UnsafePointer<UInt8>, _ len: Int) -> String {
    String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8)!
}

@_cdecl("usernotifications_request_authorization")
public func usernotificationsRequestAuthorization(
    _ options: UInt,
    _ cb: @convention(c) (Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let opts = UNAuthorizationOptions(rawValue: options)
    UNUserNotificationCenter.current().requestAuthorization(options: opts) { granted, _ in
        cb(granted, ud)
    }
}

@_cdecl("usernotifications_get_settings")
public func usernotificationsGetSettings(
    _ cb: @convention(c) (Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    UNUserNotificationCenter.current().getNotificationSettings { settings in
        // 0=notDetermined, 1=denied, 2=authorized, 3=provisional, 4=ephemeral
        cb(settings.authorizationStatus.rawValue, ud)
    }
}

@_cdecl("usernotifications_schedule")
public func usernotificationsSchedule(
    _ idPtr: UnsafePointer<UInt8>, _ idLen: Int,
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ bodyPtr: UnsafePointer<UInt8>, _ bodyLen: Int,
    _ delaySeconds: Double,
    _ cb: @convention(c) (Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let content = UNMutableNotificationContent()
    content.title = makeString(titlePtr, titleLen)
    content.body = makeString(bodyPtr, bodyLen)
    content.sound = .default

    let trigger = UNTimeIntervalNotificationTrigger(timeInterval: max(delaySeconds, 0.1), repeats: false)
    let id = makeString(idPtr, idLen)
    let request = UNNotificationRequest(identifier: id, content: content, trigger: trigger)

    UNUserNotificationCenter.current().add(request) { error in
        cb(error == nil, ud)
    }
}

@_cdecl("usernotifications_cancel")
public func usernotificationsCancel(_ idPtr: UnsafePointer<UInt8>, _ idLen: Int) {
    let id = makeString(idPtr, idLen)
    UNUserNotificationCenter.current().removePendingNotificationRequests(withIdentifiers: [id])
}

@_cdecl("usernotifications_cancel_all")
public func usernotificationsCancelAll() {
    UNUserNotificationCenter.current().removeAllPendingNotificationRequests()
}

@_cdecl("usernotifications_pending_count")
public func usernotificationsPendingCount(
    _ cb: @convention(c) (Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    UNUserNotificationCenter.current().getPendingNotificationRequests { reqs in
        cb(reqs.count, ud)
    }
}
