//! ObjC selector constants for CallKit.
#![allow(dead_code)]

// ── CXCallDirectoryManager (3 methods, 1 properties) ──
pub mod c_x_call_directory_manager {
    pub const CLASS: &[u8] = b"CXCallDirectoryManager\0";
    pub const SEL_SHARED_INSTANCE: &[u8] = b"sharedInstance\0";
    pub const SEL_SET_SHARED_INSTANCE: &[u8] = b"setSharedInstance:\0";
}

// ── CXProvider (9 methods, 2 properties) ──
pub mod c_x_provider {
    pub const SEL_CONFIGURATION: &[u8] = b"configuration\0";
    pub const SEL_SET_CONFIGURATION: &[u8] = b"setConfiguration:\0";
    pub const SEL_PENDING_TRANSACTIONS: &[u8] = b"pendingTransactions\0";
    pub const SEL_SET_PENDING_TRANSACTIONS: &[u8] = b"setPendingTransactions:\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:queue:\0";
    pub const SEL_REPORT_CALL_WITH_U_U_I_D: &[u8] = b"reportCallWithUUID:updated:\0";
    pub const SEL_REPORT_OUTGOING_CALL_WITH_U_U_I_D: &[u8] = b"reportOutgoingCallWithUUID:startedConnectingAtDate:\0";
    pub const SEL_INVALIDATE: &[u8] = b"invalidate\0";
    pub const SEL_PENDING_CALL_ACTIONS_OF_CLASS: &[u8] = b"pendingCallActionsOfClass:withCallUUID:\0";
}

// ── CXCallController (3 methods, 1 properties) ──
pub mod c_x_call_controller {
    pub const SEL_CALL_OBSERVER: &[u8] = b"callObserver\0";
    pub const SEL_SET_CALL_OBSERVER: &[u8] = b"setCallObserver:\0";
}

// ── CXTransaction (1 methods, 3 properties) ──
pub mod c_x_transaction {
    pub const SEL_U_U_I_D: &[u8] = b"UUID\0";
    pub const SEL_SET_U_U_I_D: &[u8] = b"setUUID:\0";
    pub const SEL_COMPLETE: &[u8] = b"complete\0";
    pub const SEL_SET_COMPLETE: &[u8] = b"setComplete:\0";
    pub const SEL_ACTIONS: &[u8] = b"actions\0";
    pub const SEL_SET_ACTIONS: &[u8] = b"setActions:\0";
    pub const SEL_ADD_ACTION: &[u8] = b"addAction:\0";
}

// ── CXCallAction (0 methods, 1 properties) ──
pub mod c_x_call_action {
    pub const SEL_CALL_U_U_I_D: &[u8] = b"callUUID\0";
    pub const SEL_SET_CALL_U_U_I_D: &[u8] = b"setCallUUID:\0";
}

// Total: 30 selector constants
