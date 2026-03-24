//! Apple HealthKit — health and fitness data from Rust.
//!
//! **Platform:** macOS 13+, iOS 8+, visionOS 1+, watchOS 2+.
//!
//! ```ignore
//! if healthkit::is_health_data_available() {
//!     let store = healthkit::HealthStore::new();
//!     let step_type = healthkit::QuantityType::steps();
//!     let status = store.authorization_status(&step_type);
//!     println!("Step count auth: {:?}", status);
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

/// Whether health data is available on this device.
pub fn is_health_data_available() -> bool {
    unsafe { msg_send_t![bool; class!(b"HKHealthStore\0"), isHealthDataAvailable] }
}

// ── Authorization ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus { NotDetermined = 0, SharingDenied = 1, SharingAuthorized = 2 }
impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self {
        match v { 1=>Self::SharingDenied, 2=>Self::SharingAuthorized, _=>Self::NotDetermined }
    }
}

// ── ObjectType wrappers ─────────────────────────────────────────────────────

/// Wraps `HKQuantityType` — a type for quantity samples (steps, distance, etc.).
pub struct QuantityType { inner: Id }

impl QuantityType {
    /// Create from a HealthKit type identifier string.
    fn from_identifier(id: &str) -> Self {
        unsafe {
            let ns = nsstring(id);
            let t: Id = msg_send![class!(b"HKQuantityType\0"), quantityTypeForIdentifier: ns];
            CFRelease(ns as CFTypeRef);
            Self { inner: t }
        }
    }

    pub fn steps() -> Self { Self::from_identifier("HKQuantityTypeIdentifierStepCount") }
    pub fn distance_walking() -> Self { Self::from_identifier("HKQuantityTypeIdentifierDistanceWalkingRunning") }
    pub fn active_energy() -> Self { Self::from_identifier("HKQuantityTypeIdentifierActiveEnergyBurned") }
    pub fn heart_rate() -> Self { Self::from_identifier("HKQuantityTypeIdentifierHeartRate") }
    pub fn body_mass() -> Self { Self::from_identifier("HKQuantityTypeIdentifierBodyMass") }
    pub fn height() -> Self { Self::from_identifier("HKQuantityTypeIdentifierHeight") }
    pub fn blood_glucose() -> Self { Self::from_identifier("HKQuantityTypeIdentifierBloodGlucose") }
    pub fn blood_pressure_systolic() -> Self { Self::from_identifier("HKQuantityTypeIdentifierBloodPressureSystolic") }
    pub fn blood_pressure_diastolic() -> Self { Self::from_identifier("HKQuantityTypeIdentifierBloodPressureDiastolic") }
    pub fn body_temperature() -> Self { Self::from_identifier("HKQuantityTypeIdentifierBodyTemperature") }
    pub fn oxygen_saturation() -> Self { Self::from_identifier("HKQuantityTypeIdentifierOxygenSaturation") }
    pub fn respiratory_rate() -> Self { Self::from_identifier("HKQuantityTypeIdentifierRespiratoryRate") }

    pub fn as_ptr(&self) -> Id { self.inner }
}

/// Wraps `HKCategoryType` — a type for category samples (sleep, mood, etc.).
pub struct CategoryType { inner: Id }

impl CategoryType {
    fn from_identifier(id: &str) -> Self {
        unsafe {
            let ns = nsstring(id);
            let t: Id = msg_send![class!(b"HKCategoryType\0"), categoryTypeForIdentifier: ns];
            CFRelease(ns as CFTypeRef);
            Self { inner: t }
        }
    }

    pub fn sleep_analysis() -> Self { Self::from_identifier("HKCategoryTypeIdentifierSleepAnalysis") }
    pub fn mindful_session() -> Self { Self::from_identifier("HKCategoryTypeIdentifierMindfulSession") }

    pub fn as_ptr(&self) -> Id { self.inner }
}

/// Wraps `HKCharacteristicType`.
pub struct CharacteristicType { inner: Id }

impl CharacteristicType {
    fn from_identifier(id: &str) -> Self {
        unsafe {
            let ns = nsstring(id);
            let t: Id = msg_send![class!(b"HKCharacteristicType\0"), characteristicTypeForIdentifier: ns];
            CFRelease(ns as CFTypeRef);
            Self { inner: t }
        }
    }

    pub fn biological_sex() -> Self { Self::from_identifier("HKCharacteristicTypeIdentifierBiologicalSex") }
    pub fn date_of_birth() -> Self { Self::from_identifier("HKCharacteristicTypeIdentifierDateOfBirth") }
    pub fn blood_type() -> Self { Self::from_identifier("HKCharacteristicTypeIdentifierBloodType") }

    pub fn as_ptr(&self) -> Id { self.inner }
}

// ── Unit ────────────────────────────────────────────────────────────────────

/// Wraps `HKUnit`.
pub struct Unit { inner: Id }

impl Unit {
    fn from_string(s: &str) -> Self {
        unsafe {
            let ns = nsstring(s);
            let u: Id = msg_send![class!(b"HKUnit\0"), unitFromString: ns];
            CFRelease(ns as CFTypeRef);
            Self { inner: u }
        }
    }

    pub fn count() -> Self { Self::from_string("count") }
    pub fn meter() -> Self { Self::from_string("m") }
    pub fn kilocalorie() -> Self { Self::from_string("kcal") }
    pub fn kilogram() -> Self { Self::from_string("kg") }
    pub fn pound() -> Self { Self::from_string("lb") }
    pub fn beats_per_minute() -> Self { Self::from_string("count/min") }
    pub fn milligrams_per_deciliter() -> Self { Self::from_string("mg/dL") }
    pub fn mmhg() -> Self { Self::from_string("mmHg") }
    pub fn degree_celsius() -> Self { Self::from_string("degC") }
    pub fn percent() -> Self { Self::from_string("%") }

    pub fn as_ptr(&self) -> Id { self.inner }
}

// ── HealthStore ─────────────────────────────────────────────────────────────

/// Wraps `HKHealthStore`.
pub struct HealthStore { inner: Id }

impl HealthStore {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"HKHealthStore\0"), new] } }
    }

    /// Check authorization status for a specific type.
    pub fn authorization_status(&self, quantity_type: &QuantityType) -> AuthorizationStatus {
        unsafe {
            let sel = sel_registerName(b"authorizationStatusForType:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id) -> isize =
                core::mem::transmute(objc_msgSend as *const ());
            AuthorizationStatus::from(f(self.inner, sel, quantity_type.inner))
        }
    }

    /// Whether the device supports clinical records.
    pub fn supports_health_records(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, supportsHealthRecords] }
    }

    /// Get the user's biological sex.
    pub fn biological_sex(&self) -> Option<isize> {
        unsafe {
            let obj: Id = msg_send![self.inner, biologicalSexWithError: NIL];
            if obj.is_null() { return None; }
            Some(msg_send_t![isize; obj, biologicalSex])
        }
    }

    /// Get the user's blood type.
    pub fn blood_type(&self) -> Option<isize> {
        unsafe {
            let obj: Id = msg_send![self.inner, bloodTypeWithError: NIL];
            if obj.is_null() { return None; }
            Some(msg_send_t![isize; obj, bloodType])
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for HealthStore { fn default() -> Self { Self::new() } }
impl Drop for HealthStore { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_availability() { let _ = is_health_data_available(); }

    #[test]
    fn test_quantity_types() {
        let steps = QuantityType::steps();
        assert!(!steps.inner.is_null());
        let hr = QuantityType::heart_rate();
        assert!(!hr.inner.is_null());
    }

    #[test]
    fn test_units() {
        let u = Unit::count();
        assert!(!u.inner.is_null());
        let kg = Unit::kilogram();
        assert!(!kg.inner.is_null());
    }
}
