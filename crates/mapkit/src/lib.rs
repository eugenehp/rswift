#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]
//! Apple MapKit — maps, directions, and geocoding from Rust.
//!
//! **Platform:** macOS 10.9+, iOS 3+, tvOS 9+, visionOS 1+.
//!
//! ```ignore
//! let sf = mapkit::Coordinate::new(37.7749, -122.4194);
//! let la = mapkit::Coordinate::new(34.0522, -118.2437);
//! println!("SF → LA: {:.0} km", sf.distance_to(&la) / 1000.0);
//!
//! let region = mapkit::CoordinateRegion::new(sf, 0.1, 0.1);
//! println!("Region center: {:?}", region.center);
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

// ── Coordinate ──────────────────────────────────────────────────────────────

/// A geographic coordinate (latitude/longitude in degrees).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }

    /// Distance to another coordinate in meters using CoreLocation.
    pub fn distance_to(&self, other: &Coordinate) -> f64 {
        unsafe {
            let sel_init = sel_registerName(b"initWithLatitude:longitude:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let a = f(msg_send![class!(b"CLLocation\0"), alloc], sel_init, self.latitude, self.longitude);
            let b = f(msg_send![class!(b"CLLocation\0"), alloc], sel_init, other.latitude, other.longitude);
            let sel_dist = sel_registerName(b"distanceFromLocation:\0".as_ptr());
            let df: unsafe extern "C" fn(Id, Sel, Id) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            let d = df(a, sel_dist, b);
            CFRelease(a as CFTypeRef);
            CFRelease(b as CFTypeRef);
            d
        }
    }
}

/// A span of degrees (used with coordinate regions).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CoordinateSpan {
    pub latitude_delta: f64,
    pub longitude_delta: f64,
}

/// A rectangular geographic region.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CoordinateRegion {
    pub center: Coordinate,
    pub span: CoordinateSpan,
}

impl CoordinateRegion {
    pub fn new(center: Coordinate, lat_delta: f64, lon_delta: f64) -> Self {
        Self {
            center,
            span: CoordinateSpan { latitude_delta: lat_delta, longitude_delta: lon_delta },
        }
    }
}

// ── MapPoint / MapRect (for projection) ─────────────────────────────────────

/// A point in the map's 2D projection.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapPoint {
    pub x: f64,
    pub y: f64,
}

/// A rectangle in the map's 2D projection.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapRect {
    pub origin: MapPoint,
    pub width: f64,
    pub height: f64,
}

// MKMapPoint C functions
#[allow(non_snake_case)]
unsafe extern "C" {
    fn MKMapPointForCoordinate(coord: Coordinate) -> MapPoint;
    fn MKCoordinateForMapPoint(point: MapPoint) -> Coordinate;
    fn MKMetersBetweenMapPoints(a: MapPoint, b: MapPoint) -> f64;
    fn MKMetersPerMapPointAtLatitude(latitude: f64) -> f64;
    fn MKMapPointsPerMeterAtLatitude(latitude: f64) -> f64;
}

impl MapPoint {
    /// Convert a coordinate to a map point.
    pub fn from_coordinate(coord: Coordinate) -> Self {
        unsafe { MKMapPointForCoordinate(coord) }
    }

    /// Convert back to a coordinate.
    pub fn to_coordinate(self) -> Coordinate {
        unsafe { MKCoordinateForMapPoint(self) }
    }

    /// Distance in meters to another map point.
    pub fn distance_to(self, other: MapPoint) -> f64 {
        unsafe { MKMetersBetweenMapPoints(self, other) }
    }
}

/// Meters per map point at a given latitude.
pub fn meters_per_map_point(latitude: f64) -> f64 {
    unsafe { MKMetersPerMapPointAtLatitude(latitude) }
}

/// Map points per meter at a given latitude.
pub fn map_points_per_meter(latitude: f64) -> f64 {
    unsafe { MKMapPointsPerMeterAtLatitude(latitude) }
}

// ── Placemark (from geocoding results) ──────────────────────────────────────

/// A place result with address components.
#[derive(Debug, Clone, Default)]
pub struct Placemark {
    pub name: Option<String>,
    pub thoroughfare: Option<String>,   // street
    pub sub_thoroughfare: Option<String>, // house number
    pub locality: Option<String>,        // city
    pub sub_locality: Option<String>,
    pub administrative_area: Option<String>, // state/province
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub iso_country_code: Option<String>,
    pub coordinate: Coordinate,
}

unsafe fn read_placemark(pm: Id) -> Placemark {
    let coord_sel = sel_registerName(b"coordinate\0".as_ptr());
    let coord_f: unsafe extern "C" fn(Id, Sel) -> Coordinate =
        core::mem::transmute(objc_msgSend as *const ());
    Placemark {
        name: nsstring_to_string(msg_send![pm, name]),
        thoroughfare: nsstring_to_string(msg_send![pm, thoroughfare]),
        sub_thoroughfare: nsstring_to_string(msg_send![pm, subThoroughfare]),
        locality: nsstring_to_string(msg_send![pm, locality]),
        sub_locality: nsstring_to_string(msg_send![pm, subLocality]),
        administrative_area: nsstring_to_string(msg_send![pm, administrativeArea]),
        postal_code: nsstring_to_string(msg_send![pm, postalCode]),
        country: nsstring_to_string(msg_send![pm, country]),
        iso_country_code: nsstring_to_string(msg_send![pm, ISOcountryCode]),
        coordinate: coord_f(pm, coord_sel),
    }
}

// ── Local Search ────────────────────────────────────────────────────────────

/// A local search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub phone_number: Option<String>,
    pub url: Option<String>,
    pub placemark: Placemark,
}

// ── Directions ──────────────────────────────────────────────────────────────

/// Transport type for directions.
#[derive(Debug, Clone, Copy)]
pub enum TransportType {
    Automobile = 1,
    Walking = 2,
    Transit = 4,
}

// ── Utility ─────────────────────────────────────────────────────────────────

/// Haversine distance between two coordinates (pure Rust, no framework call).
pub fn haversine_distance(a: Coordinate, b: Coordinate) -> f64 {
    let r = 6_371_000.0_f64;
    let d_lat = (b.latitude - a.latitude).to_radians();
    let d_lon = (b.longitude - a.longitude).to_radians();
    let lat1 = a.latitude.to_radians();
    let lat2 = b.latitude.to_radians();
    let h = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
    2.0 * r * h.sqrt().asin()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_distance() {
        let sf = Coordinate::new(37.7749, -122.4194);
        let la = Coordinate::new(34.0522, -118.2437);
        let d = sf.distance_to(&la);
        assert!((d - 559_000.0).abs() < 10_000.0, "Expected ~559km, got {:.0}m", d);
    }

    #[test]
    fn test_haversine() {
        let sf = Coordinate::new(37.7749, -122.4194);
        let la = Coordinate::new(34.0522, -118.2437);
        let d = haversine_distance(sf, la);
        assert!((d - 559_000.0).abs() < 10_000.0);
    }

    #[test]
    fn test_map_point_roundtrip() {
        let coord = Coordinate::new(37.7749, -122.4194);
        let mp = MapPoint::from_coordinate(coord);
        let back = mp.to_coordinate();
        assert!((back.latitude - coord.latitude).abs() < 0.0001);
        assert!((back.longitude - coord.longitude).abs() < 0.0001);
    }

    #[test]
    fn test_map_point_distance() {
        let sf = MapPoint::from_coordinate(Coordinate::new(37.7749, -122.4194));
        let la = MapPoint::from_coordinate(Coordinate::new(34.0522, -118.2437));
        let d = sf.distance_to(la);
        assert!((d - 559_000.0).abs() < 10_000.0);
    }

    #[test]
    fn test_meters_per_map_point() {
        let m = meters_per_map_point(37.0);
        assert!(m > 0.0 && m < 1.0); // very small at mid-latitudes
    }

    #[test]
    fn test_coordinate_region() {
        let r = CoordinateRegion::new(Coordinate::new(37.0, -122.0), 1.0, 1.0);
        assert_eq!(r.center.latitude, 37.0);
        assert_eq!(r.span.latitude_delta, 1.0);
    }
}
