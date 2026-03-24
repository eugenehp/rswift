//! ObjC selector constants for BackgroundTasks.
#![allow(dead_code)]

// ── BGTaskScheduler (5 methods, 1 properties) ──
pub mod b_g_task_scheduler {
    pub const CLASS: &[u8] = b"BGTaskScheduler\0";
    pub const SEL_SHARED_SCHEDULER: &[u8] = b"sharedScheduler\0";
    pub const SEL_SET_SHARED_SCHEDULER: &[u8] = b"setSharedScheduler:\0";
    pub const SEL_SUBMIT_TASK_REQUEST: &[u8] = b"submitTaskRequest:error:\0";
    pub const SEL_CANCEL_TASK_REQUEST_WITH_IDENTIFIER: &[u8] = b"cancelTaskRequestWithIdentifier:\0";
    pub const SEL_CANCEL_ALL_TASK_REQUESTS: &[u8] = b"cancelAllTaskRequests\0";
}

// ── BGTask (1 methods, 1 properties) ──
pub mod b_g_task {
    pub const SEL_IDENTIFIER: &[u8] = b"identifier\0";
    pub const SEL_SET_IDENTIFIER: &[u8] = b"setIdentifier:\0";
    pub const SEL_SET_TASK_COMPLETED_WITH_SUCCESS: &[u8] = b"setTaskCompletedWithSuccess:\0";
}

// ── BGAppRefreshTaskRequest (0 methods, 0 properties) ──
pub mod b_g_app_refresh_task_request {
}

// ── BGProcessingTaskRequest (0 methods, 2 properties) ──
pub mod b_g_processing_task_request {
    pub const SEL_REQUIRES_NETWORK_CONNECTIVITY: &[u8] = b"requiresNetworkConnectivity\0";
    pub const SEL_SET_REQUIRES_NETWORK_CONNECTIVITY: &[u8] = b"setRequiresNetworkConnectivity:\0";
    pub const SEL_REQUIRES_EXTERNAL_POWER: &[u8] = b"requiresExternalPower\0";
    pub const SEL_SET_REQUIRES_EXTERNAL_POWER: &[u8] = b"setRequiresExternalPower:\0";
}

// Total: 14 selector constants
