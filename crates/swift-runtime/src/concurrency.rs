//! Safe concurrency runtime accessors.
//!
//! Wraps the Swift concurrency runtime (tasks, executors, time).
//! Only exposes the current (non-deprecated) APIs from Swift 6.3.

use core::ffi::c_void;

/// An executor reference (identity + implementation pair).
#[derive(Debug, Clone, Copy)]
pub struct Executor {
    pub identity: *const c_void,
    pub implementation: *const c_void,
}

/// Get the main executor.
pub fn main_executor() -> Option<Executor> {
    let result = unsafe { swift_runtime_sys::ConcurrencyThunks::swift_task_getMainExecutor() };
    result.ok().map(|e| Executor {
        identity: e.identity,
        implementation: e.implementation,
    })
}

/// Get the current executor (if running inside a task).
pub fn current_executor() -> Option<Executor> {
    let result = unsafe { swift_runtime_sys::ConcurrencyThunks::swift_task_getCurrentExecutor() };
    result.ok().map(|e| Executor {
        identity: e.identity,
        implementation: e.implementation,
    })
}

/// Check whether an executor is the main executor.
pub fn is_main_executor(executor: &Executor) -> Option<bool> {
    let raw = swift_runtime_sys::ConcurrencyThunks::SerialExecutorRef {
        identity: executor.identity,
        implementation: executor.implementation,
    };
    unsafe { swift_runtime_sys::ConcurrencyThunks::swift_task_isMainExecutor(raw) }.ok()
}

/// A monotonic time reading from the Swift concurrency clock.
#[derive(Debug, Clone, Copy)]
pub struct SwiftTime {
    pub seconds: i64,
    pub nanoseconds: i64,
}

/// Read the continuous (monotonic) clock used by Swift concurrency.
pub fn continuous_time() -> Option<SwiftTime> {
    let mut sec: i64 = 0;
    let mut nsec: i64 = 0;
    let result =
        unsafe { swift_runtime_sys::ConcurrencyThunks::swift_get_time(&mut sec, &mut nsec, 1) };
    result.ok().map(|_| SwiftTime {
        seconds: sec,
        nanoseconds: nsec,
    })
}

/// Read the suspending clock used by Swift concurrency.
pub fn suspending_time() -> Option<SwiftTime> {
    let mut sec: i64 = 0;
    let mut nsec: i64 = 0;
    // clock 1 = continuous, clock 2 = suspending
    let result =
        unsafe { swift_runtime_sys::ConcurrencyThunks::swift_get_time(&mut sec, &mut nsec, 2) };
    result.ok().map(|_| SwiftTime {
        seconds: sec,
        nanoseconds: nsec,
    })
}
