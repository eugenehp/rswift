//! Apple UserNotifications — local notifications from Rust.
//!
//! **Platform:** macOS 10.14+, iOS 10+, tvOS 10+, watchOS 3+.
//!
//! ```ignore
//! usernotifications::cancel_all();
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


fn center() -> Id {
    unsafe { msg_send![class!(b"UNUserNotificationCenter\0"), currentNotificationCenter] }
}

/// Cancel a pending notification by identifier.
pub fn cancel(identifier: &str) {
    unsafe {
        let ns = nsstring(identifier);
        let arr = msg_send![class!(b"NSArray\0"), arrayWithObject: ns];
        msg_send_void![center(), removePendingNotificationRequestsWithIdentifiers: arr];
        CFRelease(ns as CFTypeRef);
    }
}

/// Cancel all pending notifications.
pub fn cancel_all() {
    unsafe { msg_send_void![center(), removeAllPendingNotificationRequests]; }
}

/// Remove all delivered notifications from the notification center.
pub fn remove_all_delivered() {
    unsafe { msg_send_void![center(), removeAllDeliveredNotifications]; }
}

/// Schedule a local notification with a time interval trigger.
///
/// - `identifier`: unique ID for this notification
/// - `title`: notification title
/// - `body`: notification body text
/// - `delay_seconds`: seconds from now (minimum 0.1)
///
/// Note: requires notification authorization. This is fire-and-forget;
/// use the callback-based version for error handling.
pub fn schedule(identifier: &str, title: &str, body: &str, delay_seconds: f64) {
    unsafe {
        let content: Id = msg_send![class!(b"UNMutableNotificationContent\0"), new];
        let t = nsstring(title);
        let b = nsstring(body);
        msg_send_void![content, setTitle: t];
        msg_send_void![content, setBody: b];
        // Set default sound
        let snd: Id = msg_send![class!(b"UNNotificationSound\0"), defaultSound];
        msg_send_void![content, setSound: snd];
        CFRelease(t as CFTypeRef);
        CFRelease(b as CFTypeRef);

        let delay = if delay_seconds < 0.1 { 0.1 } else { delay_seconds };
        let sel_trigger = sel_registerName(
            b"triggerWithTimeInterval:repeats:\0".as_ptr());
        let f_trigger: unsafe extern "C" fn(Id, Sel, f64, bool) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let trigger = f_trigger(
            class!(b"UNTimeIntervalNotificationTrigger\0") as Id,
            sel_trigger, delay, false);

        let ns_id = nsstring(identifier);
        let sel_req = sel_registerName(
            b"requestWithIdentifier:content:trigger:\0".as_ptr());
        let f_req: unsafe extern "C" fn(Id, Sel, Id, Id, Id) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let request = f_req(
            class!(b"UNNotificationRequest\0") as Id,
            sel_req, ns_id, content, trigger);
        CFRelease(ns_id as CFTypeRef);

        msg_send_void![center(), addNotificationRequest: request, withCompletionHandler: NIL];
    }
}

/// Authorization options flags.
pub mod options {
    pub const BADGE: u64 = 1;
    pub const SOUND: u64 = 2;
    pub const ALERT: u64 = 4;
    pub const PROVISIONAL: u64 = 64;
}
