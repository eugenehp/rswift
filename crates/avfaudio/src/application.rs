//! `AVAudioApplication` — app-level audio management (iOS 17+).

use apple_objc_sys::*;
use crate::*;

/// Record permission status.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordPermission {
    Undetermined = 0,
    Denied = 1,
    Granted = 2,
}

/// App-level audio configuration.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudioapplication>
pub struct AudioApplication { h: Id }

unsafe impl Send for AudioApplication {}
unsafe impl Sync for AudioApplication {}

impl AudioApplication {
    /// Get the shared instance.
    pub fn shared() -> Self {
        unsafe {
            let cls = class!(b"AVAudioApplication\0") as Id;
            let h = msg_send_id(cls, b"sharedInstance\0");
            Self { h }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    /// The current record permission.
    pub fn record_permission(&self) -> RecordPermission {
        let v = unsafe { msg_send_isize(self.h, b"recordPermission\0") };
        match v {
            1 => RecordPermission::Denied,
            2 => RecordPermission::Granted,
            _ => RecordPermission::Undetermined,
        }
    }

    /// Whether the input is muted.
    pub fn is_input_muted(&self) -> bool { unsafe { msg_send_bool(self.h, b"isInputMuted\0") } }

    /// Set input muted.
    pub fn set_input_muted(&self, muted: bool) -> Result<(), String> {
        unsafe {
            let sel = sel_registerName(b"setInputMuted:error:\0".as_ptr());
            let mut err: Id = NIL;
            let f: unsafe extern "C" fn(Id, Sel, bool, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let ok = f(self.h, sel, muted, &mut err);
            if ok { Ok(()) } else {
                let desc = if !err.is_null() {
                    let d = msg_send_id(err, b"localizedDescription\0");
                    nsstring_to_string(d).unwrap_or_else(|| "unknown error".into())
                } else { "unknown error".into() };
                Err(desc)
            }
        }
    }
}
