import Foundation
import CoreLocation

// ═══════════════════════════════════════════════════════════════════════════
// CoreLocation — location, geocoding
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("corelocation_available")
public func corelocationAvailable() -> Bool { true }

// ── Authorization ───────────────────────────────────────────────────────────

@_cdecl("corelocation_authorization_status")
public func corelocationAuthorizationStatus() -> Int {
    let mgr = CLLocationManager()
    return Int(mgr.authorizationStatus.rawValue)
}

// ── Geocoding ───────────────────────────────────────────────────────────────

private class GeoHelper {
    static let geocoder = CLGeocoder()
}

@_cdecl("corelocation_geocode")
public func corelocationGeocode(
    _ addrPtr: UnsafePointer<UInt8>, _ addrLen: Int,
    _ cb: @convention(c) (Double, Double, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let addr = String(bytes: UnsafeBufferPointer(start: addrPtr, count: addrLen), encoding: .utf8) ?? ""
    GeoHelper.geocoder.geocodeAddressString(addr) { placemarks, error in
        if let loc = placemarks?.first?.location {
            cb(loc.coordinate.latitude, loc.coordinate.longitude, true, ud)
        } else {
            cb(0, 0, false, ud)
        }
    }
}

@_cdecl("corelocation_reverse_geocode")
public func corelocationReverseGeocode(
    _ lat: Double, _ lon: Double,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let location = CLLocation(latitude: lat, longitude: lon)
    GeoHelper.geocoder.reverseGeocodeLocation(location) { placemarks, error in
        if let pm = placemarks?.first {
            var parts: [String] = []
            if let street = pm.thoroughfare { parts.append(street) }
            if let city = pm.locality { parts.append(city) }
            if let state = pm.administrativeArea { parts.append(state) }
            if let country = pm.country { parts.append(country) }
            let result = parts.joined(separator: ", ")
            result.withCString { ptr in
                cb(UnsafePointer(OpaquePointer(ptr)), result.utf8.count, true, ud)
            }
        } else {
            cb(UnsafePointer(OpaquePointer(bitPattern: 1))!, 0, false, ud)
        }
    }
}

// ── Distance ────────────────────────────────────────────────────────────────

@_cdecl("corelocation_distance")
public func corelocationDistance(_ lat1: Double, _ lon1: Double, _ lat2: Double, _ lon2: Double) -> Double {
    let a = CLLocation(latitude: lat1, longitude: lon1)
    let b = CLLocation(latitude: lat2, longitude: lon2)
    return a.distance(from: b)
}
