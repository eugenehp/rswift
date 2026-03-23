//! Apple CoreLocation — GPS, geocoding, and distance from Rust.
//!
//! **Platform support:** macOS 10.6+, iOS 2+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! # Quick start
//!
//! ```ignore
//! // Distance between two coordinates (meters)
//! let d = corelocation::distance(37.7749, -122.4194, 34.0522, -118.2437);
//! println!("SF → LA: {:.0} km", d / 1000.0);
//!
//! // Check authorization
//! println!("Auth: {:?}", corelocation::authorization_status());
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"corelocation_available");

unsafe extern "C" {
    fn corelocation_authorization_status() -> isize;
    fn corelocation_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64;
}

/// Location authorization status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus {
    NotDetermined = 0,
    Restricted = 1,
    Denied = 2,
    AuthorizedAlways = 3,
    AuthorizedWhenInUse = 4,
}

impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self {
        match v {
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::AuthorizedAlways,
            4 => Self::AuthorizedWhenInUse,
            _ => Self::NotDetermined,
        }
    }
}

/// Current location authorization status.
pub fn authorization_status() -> AuthorizationStatus {
    AuthorizationStatus::from(unsafe { corelocation_authorization_status() })
}

/// Great-circle distance between two coordinates in meters.
pub fn distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    unsafe { corelocation_distance(lat1, lon1, lat2, lon2) }
}
