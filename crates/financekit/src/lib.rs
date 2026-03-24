#![allow(unsafe_op_in_unsafe_fn)]
//! Apple FinanceKit — financial data access from Rust.
//!
//! **Platform:** macOS 15+, iOS 17+.
//!
//! Requires FinanceKit entitlement.
//!
//! ```ignore
//! println!("Auth: {:?}", financekit::authorization_status());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use core::ffi::c_void;

unsafe extern "C" {
    fn financekit_swift_available() -> bool;
    fn financekit_authorization_status() -> isize;
}

pub fn is_available() -> bool { unsafe { financekit_swift_available() } }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus { NotDetermined = 0, Authorized = 1, Denied = 2 }
impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Authorized, 2=>Self::Denied, _=>Self::NotDetermined }
    }
}

pub fn authorization_status() -> AuthorizationStatus {
    AuthorizationStatus::from(unsafe { financekit_authorization_status() })
}

/// A financial account.
#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: String,
}

/// Fetch accounts (async via callback).
pub fn fetch_accounts<F: FnOnce(Result<Vec<Account>, ()>) + Send + 'static>(callback: F) {
    unsafe extern "C" fn trampoline(
        json_ptr: *const u8, json_len: usize, success: bool, ud: *mut c_void,
    ) {
        let cb: Box<Box<dyn FnOnce(Result<Vec<Account>, ()>) + Send>> =
            Box::from_raw(ud as *mut _);
        if success && json_len > 0 && !json_ptr.is_null() {
            let json = String::from_utf8_lossy(
                core::slice::from_raw_parts(json_ptr, json_len)
            ).into_owned();
            let mut accounts = Vec::new();
            for entry in json.trim_matches(|c| c == '[' || c == ']').split("},{") {
                let entry = entry.trim_matches(|c| c == '{' || c == '}');
                let mut id = String::new();
                let mut name = String::new();
                let mut atype = String::new();
                for field in entry.split(',') {
                    let parts: Vec<&str> = field.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim().trim_matches('"');
                        let val = parts[1].trim().trim_matches('"');
                        match key {
                            "id" => id = val.to_string(),
                            "name" => name = val.to_string(),
                            "type" => atype = val.to_string(),
                            _ => {}
                        }
                    }
                }
                if !id.is_empty() {
                    accounts.push(Account { id, name, account_type: atype });
                }
            }
            cb(Ok(accounts));
        } else {
            cb(Err(()));
        }
    }

    let cb: Box<Box<dyn FnOnce(Result<Vec<Account>, ()>) + Send>> =
        Box::new(Box::new(callback));
    let ud = Box::into_raw(cb) as *mut c_void;
    unsafe {
        extern "C" {
            fn financekit_fetch_accounts(
                cb: unsafe extern "C" fn(*const u8, usize, bool, *mut c_void),
                ud: *mut c_void,
            );
        }
        financekit_fetch_accounts(trampoline, ud);
    }
}
