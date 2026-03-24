//! Apple MultipeerConnectivity — peer-to-peer networking from Rust.
//!
//! **Platform:** macOS 10.10+, iOS 7+, tvOS 10+, visionOS 1+.
//!
//! ```ignore
//! let peer = multipeerconnectivity::PeerId::new("MyDevice");
//! println!("Peer: {}", peer.display_name());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState { NotConnected = 0, Connecting = 1, Connected = 2 }
impl From<isize> for SessionState {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Connecting, 2=>Self::Connected, _=>Self::NotConnected }
    }
}

/// Wraps `MCPeerID`.
pub struct PeerId { inner: Id }

impl PeerId {
    pub fn new(display_name: &str) -> Self {
        unsafe {
            let ns = nsstring(display_name);
            let p: Id = msg_send![class!(b"MCPeerID\0"), alloc];
            let p = msg_send![p, initWithDisplayName: ns];
            CFRelease(ns as CFTypeRef);
            Self { inner: p }
        }
    }

    pub fn display_name(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, displayName]).unwrap_or_default() }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for PeerId { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Wraps `MCSession`.
pub struct Session { inner: Id }

impl Session {
    /// Create a session with the local peer.
    pub fn new(peer: &PeerId) -> Self {
        unsafe {
            let s: Id = msg_send![class!(b"MCSession\0"), alloc];
            let s = msg_send![s, initWithPeer: peer.inner];
            Self { inner: s }
        }
    }

    /// Connected peers.
    pub fn connected_peers(&self) -> Vec<String> {
        unsafe {
            let arr: Id = msg_send![self.inner, connectedPeers];
            if arr.is_null() { return vec![]; }
            let count: usize = msg_send_t![usize; arr, count];
            (0..count).filter_map(|i| {
                let p: Id = msg_send![arr, objectAtIndex: i];
                nsstring_to_string(msg_send![p, displayName])
            }).collect()
        }
    }

    /// Disconnect from all peers.
    pub fn disconnect(&self) {
        unsafe { msg_send_void![self.inner, disconnect]; }
    }

    /// Send data to all connected peers.
    pub fn send_to_all(&self, data: &[u8], reliable: bool) -> bool {
        unsafe {
            let nsdata: Id = msg_send![class!(b"NSData\0"), dataWithBytes: data.as_ptr(), length: data.len()];
            let peers: Id = msg_send![self.inner, connectedPeers];
            let mode: isize = if reliable { 0 } else { 1 }; // MCSessionSendDataReliable=0
            let sel = sel_registerName(b"sendData:toPeers:withMode:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, isize, Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, nsdata, peers, mode, NIL)
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Session { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_id() {
        let p = PeerId::new("TestDevice");
        assert_eq!(p.display_name(), "TestDevice");
    }

    #[test]
    fn test_session() {
        let peer = PeerId::new("LocalPeer");
        let session = Session::new(&peer);
        assert!(session.connected_peers().is_empty());
    }
}
