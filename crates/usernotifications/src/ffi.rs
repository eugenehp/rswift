//! ObjC selector constants for UserNotifications.
#![allow(dead_code)]

// ── UNUserNotificationCenter (13 methods, 2 properties) ──
pub mod u_n_user_notification_center {
    pub const CLASS: &[u8] = b"UNUserNotificationCenter\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_SUPPORTS_CONTENT_EXTENSIONS: &[u8] = b"supportsContentExtensions\0";
    pub const SEL_SET_SUPPORTS_CONTENT_EXTENSIONS: &[u8] = b"setSupportsContentExtensions:\0";
    pub const SEL_CURRENT_NOTIFICATION_CENTER: &[u8] = b"currentNotificationCenter\0";
    pub const SEL_SET_NOTIFICATION_CATEGORIES: &[u8] = b"setNotificationCategories:\0";
    pub const SEL_REMOVE_PENDING_NOTIFICATION_REQUESTS_WITH_IDENTIFIERS: &[u8] = b"removePendingNotificationRequestsWithIdentifiers:\0";
    pub const SEL_REMOVE_ALL_PENDING_NOTIFICATION_REQUESTS: &[u8] = b"removeAllPendingNotificationRequests\0";
    pub const SEL_REMOVE_DELIVERED_NOTIFICATIONS_WITH_IDENTIFIERS: &[u8] = b"removeDeliveredNotificationsWithIdentifiers:\0";
    pub const SEL_REMOVE_ALL_DELIVERED_NOTIFICATIONS: &[u8] = b"removeAllDeliveredNotifications\0";
}

// ── UNNotificationRequest (1 methods, 3 properties) ──
pub mod u_n_notification_request {
    pub const SEL_IDENTIFIER: &[u8] = b"identifier\0";
    pub const SEL_SET_IDENTIFIER: &[u8] = b"setIdentifier:\0";
    pub const SEL_CONTENT: &[u8] = b"content\0";
    pub const SEL_SET_CONTENT: &[u8] = b"setContent:\0";
    pub const SEL_TRIGGER: &[u8] = b"trigger\0";
    pub const SEL_SET_TRIGGER: &[u8] = b"setTrigger:\0";
    pub const SEL_REQUEST_WITH_IDENTIFIER: &[u8] = b"requestWithIdentifier:content:trigger:\0";
}

// ── UNMutableNotificationContent (0 methods, 1 properties) ──
pub mod u_n_mutable_notification_content {
    pub const SEL_BADGE: &[u8] = b"badge\0";
    pub const SEL_SET_BADGE: &[u8] = b"setBadge:\0";
}

// ── UNNotificationSettings (0 methods, 1 properties) ──
pub mod u_n_notification_settings {
    pub const SEL_AUTHORIZATION_STATUS: &[u8] = b"authorizationStatus\0";
    pub const SEL_SET_AUTHORIZATION_STATUS: &[u8] = b"setAuthorizationStatus:\0";
}

// ── UNNotificationCategory (3 methods, 4 properties) ──
pub mod u_n_notification_category {
    pub const SEL_ACTIONS: &[u8] = b"actions\0";
    pub const SEL_SET_ACTIONS: &[u8] = b"setActions:\0";
    pub const SEL_INTENT_IDENTIFIERS: &[u8] = b"intentIdentifiers\0";
    pub const SEL_SET_INTENT_IDENTIFIERS: &[u8] = b"setIntentIdentifiers:\0";
    pub const SEL_OPTIONS: &[u8] = b"options\0";
    pub const SEL_SET_OPTIONS: &[u8] = b"setOptions:\0";
    pub const SEL_CATEGORY_WITH_IDENTIFIER: &[u8] = b"categoryWithIdentifier:actions:intentIdentifiers:options:\0";
}

// ── UNNotificationAction (2 methods, 3 properties) ──
pub mod u_n_notification_action {
    pub const SEL_TITLE: &[u8] = b"title\0";
    pub const SEL_SET_TITLE: &[u8] = b"setTitle:\0";
    pub const SEL_ACTION_WITH_IDENTIFIER: &[u8] = b"actionWithIdentifier:title:options:\0";
}

// ── UNTimeIntervalNotificationTrigger (2 methods, 1 properties) ──
pub mod u_n_time_interval_notification_trigger {
    pub const SEL_TIME_INTERVAL: &[u8] = b"timeInterval\0";
    pub const SEL_SET_TIME_INTERVAL: &[u8] = b"setTimeInterval:\0";
    pub const SEL_TRIGGER_WITH_TIME_INTERVAL: &[u8] = b"triggerWithTimeInterval:repeats:\0";
    pub const SEL_NEXT_TRIGGER_DATE: &[u8] = b"nextTriggerDate\0";
}

// Total: 48 selector constants
