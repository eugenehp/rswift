//! ObjC selector constants for MultipeerConnectivity.
#![allow(dead_code)]

// ── MCPeerID (0 methods, 1 properties) ──
pub mod m_c_peer_i_d {
    pub const CLASS: &[u8] = b"MCPeerID\0";
    pub const SEL_DISPLAY_NAME: &[u8] = b"displayName\0";
    pub const SEL_SET_DISPLAY_NAME: &[u8] = b"setDisplayName:\0";
}

// ── MCSession (4 methods, 5 properties) ──
pub mod m_c_session {
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_MY_PEER_I_D: &[u8] = b"myPeerID\0";
    pub const SEL_SET_MY_PEER_I_D: &[u8] = b"setMyPeerID:\0";
    pub const SEL_SECURITY_IDENTITY: &[u8] = b"securityIdentity\0";
    pub const SEL_SET_SECURITY_IDENTITY: &[u8] = b"setSecurityIdentity:\0";
    pub const SEL_ENCRYPTION_PREFERENCE: &[u8] = b"encryptionPreference\0";
    pub const SEL_SET_ENCRYPTION_PREFERENCE: &[u8] = b"setEncryptionPreference:\0";
    pub const SEL_CONNECTED_PEERS: &[u8] = b"connectedPeers\0";
    pub const SEL_SET_CONNECTED_PEERS: &[u8] = b"setConnectedPeers:\0";
    pub const SEL_SEND_DATA: &[u8] = b"sendData:toPeers:withMode:error:\0";
    pub const SEL_DISCONNECT: &[u8] = b"disconnect\0";
    pub const SEL_START_STREAM_WITH_NAME: &[u8] = b"startStreamWithName:toPeer:error:\0";
}

// ── MCNearbyServiceAdvertiser (2 methods, 4 properties) ──
pub mod m_c_nearby_service_advertiser {
    pub const SEL_DISCOVERY_INFO: &[u8] = b"discoveryInfo\0";
    pub const SEL_SET_DISCOVERY_INFO: &[u8] = b"setDiscoveryInfo:\0";
    pub const SEL_SERVICE_TYPE: &[u8] = b"serviceType\0";
    pub const SEL_SET_SERVICE_TYPE: &[u8] = b"setServiceType:\0";
    pub const SEL_START_ADVERTISING_PEER: &[u8] = b"startAdvertisingPeer\0";
    pub const SEL_STOP_ADVERTISING_PEER: &[u8] = b"stopAdvertisingPeer\0";
}

// ── MCNearbyServiceBrowser (3 methods, 3 properties) ──
pub mod m_c_nearby_service_browser {
    pub const SEL_START_BROWSING_FOR_PEERS: &[u8] = b"startBrowsingForPeers\0";
    pub const SEL_STOP_BROWSING_FOR_PEERS: &[u8] = b"stopBrowsingForPeers\0";
    pub const SEL_INVITE_PEER: &[u8] = b"invitePeer:toSession:withContext:timeout:\0";
}

// ── MCBrowserViewController (0 methods, 5 properties) ──
pub mod m_c_browser_view_controller {
    pub const SEL_BROWSER: &[u8] = b"browser\0";
    pub const SEL_SET_BROWSER: &[u8] = b"setBrowser:\0";
    pub const SEL_SESSION: &[u8] = b"session\0";
    pub const SEL_SET_SESSION: &[u8] = b"setSession:\0";
    pub const SEL_MINIMUM_NUMBER_OF_PEERS: &[u8] = b"minimumNumberOfPeers\0";
    pub const SEL_SET_MINIMUM_NUMBER_OF_PEERS: &[u8] = b"setMinimumNumberOfPeers:\0";
    pub const SEL_MAXIMUM_NUMBER_OF_PEERS: &[u8] = b"maximumNumberOfPeers\0";
    pub const SEL_SET_MAXIMUM_NUMBER_OF_PEERS: &[u8] = b"setMaximumNumberOfPeers:\0";
}

// Total: 45 selector constants
