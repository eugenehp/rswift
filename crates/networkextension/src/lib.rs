//! Apple NetworkExtension — VPN, content filter, DNS proxy from Rust.
//!
//! **Platform:** macOS 10.11+, iOS 8+, tvOS 17+.

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

pub fn is_available() -> bool { true }

/// Check if a VPN configuration is installed.
pub fn vpn_status() -> isize {
    unsafe {
        let mgr: Id = msg_send![class!(b"NEVPNManager\0"), sharedManager];
        let conn: Id = msg_send![mgr, connection];
        msg_send_t![isize; conn, status]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vpn_status() { let _ = vpn_status(); }
}
