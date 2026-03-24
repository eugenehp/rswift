//! ObjC selector constants for CoreLocation.
#![allow(dead_code)]

// ── CLLocationManager (34 methods, 4 properties) ──
pub mod c_l_location_manager {
    pub const CLASS: &[u8] = b"CLLocationManager\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_DISTANCE_FILTER: &[u8] = b"distanceFilter\0";
    pub const SEL_SET_DISTANCE_FILTER: &[u8] = b"setDistanceFilter:\0";
    pub const SEL_DESIRED_ACCURACY: &[u8] = b"desiredAccuracy\0";
    pub const SEL_SET_DESIRED_ACCURACY: &[u8] = b"setDesiredAccuracy:\0";
    pub const SEL_LOCATION: &[u8] = b"location\0";
    pub const SEL_SET_LOCATION: &[u8] = b"setLocation:\0";
    pub const SEL_LOCATION_SERVICES_ENABLED: &[u8] = b"locationServicesEnabled\0";
    pub const SEL_HEADING_AVAILABLE: &[u8] = b"headingAvailable\0";
    pub const SEL_SIGNIFICANT_LOCATION_CHANGE_MONITORING_AVAILABLE: &[u8] = b"significantLocationChangeMonitoringAvailable\0";
    pub const SEL_IS_MONITORING_AVAILABLE_FOR_CLASS: &[u8] = b"isMonitoringAvailableForClass:\0";
    pub const SEL_REGION_MONITORING_AVAILABLE: &[u8] = b"regionMonitoringAvailable\0";
    pub const SEL_REGION_MONITORING_ENABLED: &[u8] = b"regionMonitoringEnabled\0";
    pub const SEL_IS_RANGING_AVAILABLE: &[u8] = b"isRangingAvailable\0";
    pub const SEL_AUTHORIZATION_STATUS: &[u8] = b"authorizationStatus\0";
    pub const SEL_REQUEST_WHEN_IN_USE_AUTHORIZATION: &[u8] = b"requestWhenInUseAuthorization\0";
    pub const SEL_REQUEST_ALWAYS_AUTHORIZATION: &[u8] = b"requestAlwaysAuthorization\0";
    pub const SEL_START_UPDATING_LOCATION: &[u8] = b"startUpdatingLocation\0";
    pub const SEL_STOP_UPDATING_LOCATION: &[u8] = b"stopUpdatingLocation\0";
    pub const SEL_REQUEST_LOCATION: &[u8] = b"requestLocation\0";
    pub const SEL_START_UPDATING_HEADING: &[u8] = b"startUpdatingHeading\0";
    pub const SEL_STOP_UPDATING_HEADING: &[u8] = b"stopUpdatingHeading\0";
    pub const SEL_DISMISS_HEADING_CALIBRATION_DISPLAY: &[u8] = b"dismissHeadingCalibrationDisplay\0";
    pub const SEL_START_MONITORING_SIGNIFICANT_LOCATION_CHANGES: &[u8] = b"startMonitoringSignificantLocationChanges\0";
    pub const SEL_STOP_MONITORING_SIGNIFICANT_LOCATION_CHANGES: &[u8] = b"stopMonitoringSignificantLocationChanges\0";
    pub const SEL_STOP_MONITORING_LOCATION_PUSHES: &[u8] = b"stopMonitoringLocationPushes\0";
    pub const SEL_START_MONITORING_FOR_REGION: &[u8] = b"startMonitoringForRegion:desiredAccuracy:\0";
    pub const SEL_STOP_MONITORING_FOR_REGION: &[u8] = b"stopMonitoringForRegion:\0";
    pub const SEL_REQUEST_STATE_FOR_REGION: &[u8] = b"requestStateForRegion:\0";
    pub const SEL_START_RANGING_BEACONS_IN_REGION: &[u8] = b"startRangingBeaconsInRegion:\0";
    pub const SEL_STOP_RANGING_BEACONS_IN_REGION: &[u8] = b"stopRangingBeaconsInRegion:\0";
    pub const SEL_START_RANGING_BEACONS_SATISFYING_CONSTRAINT: &[u8] = b"startRangingBeaconsSatisfyingConstraint:\0";
    pub const SEL_STOP_RANGING_BEACONS_SATISFYING_CONSTRAINT: &[u8] = b"stopRangingBeaconsSatisfyingConstraint:\0";
    pub const SEL_ALLOW_DEFERRED_LOCATION_UPDATES_UNTIL_TRAVELED: &[u8] = b"allowDeferredLocationUpdatesUntilTraveled:timeout:\0";
    pub const SEL_DISALLOW_DEFERRED_LOCATION_UPDATES: &[u8] = b"disallowDeferredLocationUpdates\0";
    pub const SEL_DEFERRED_LOCATION_UPDATES_AVAILABLE: &[u8] = b"deferredLocationUpdatesAvailable\0";
}

// ── CLGeocoder (8 methods, 1 properties) ──
pub mod c_l_geocoder {
    pub const SEL_GEOCODING: &[u8] = b"geocoding\0";
    pub const SEL_SET_GEOCODING: &[u8] = b"setGeocoding:\0";
    pub const SEL_REVERSE_GEOCODE_LOCATION: &[u8] = b"reverseGeocodeLocation:completionHandler:\0";
    pub const SEL_GEOCODE_ADDRESS_DICTIONARY: &[u8] = b"geocodeAddressDictionary:completionHandler:\0";
    pub const SEL_GEOCODE_ADDRESS_STRING: &[u8] = b"geocodeAddressString:inRegion:completionHandler:\0";
    pub const SEL_CANCEL_GEOCODE: &[u8] = b"cancelGeocode\0";
}

// ── CLPlacemark (0 methods, 14 properties) ──
pub mod c_l_placemark {
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_SET_NAME: &[u8] = b"setName:\0";
    pub const SEL_THOROUGHFARE: &[u8] = b"thoroughfare\0";
    pub const SEL_SET_THOROUGHFARE: &[u8] = b"setThoroughfare:\0";
    pub const SEL_SUB_THOROUGHFARE: &[u8] = b"subThoroughfare\0";
    pub const SEL_SET_SUB_THOROUGHFARE: &[u8] = b"setSubThoroughfare:\0";
    pub const SEL_LOCALITY: &[u8] = b"locality\0";
    pub const SEL_SET_LOCALITY: &[u8] = b"setLocality:\0";
    pub const SEL_SUB_LOCALITY: &[u8] = b"subLocality\0";
    pub const SEL_SET_SUB_LOCALITY: &[u8] = b"setSubLocality:\0";
    pub const SEL_ADMINISTRATIVE_AREA: &[u8] = b"administrativeArea\0";
    pub const SEL_SET_ADMINISTRATIVE_AREA: &[u8] = b"setAdministrativeArea:\0";
    pub const SEL_SUB_ADMINISTRATIVE_AREA: &[u8] = b"subAdministrativeArea\0";
    pub const SEL_SET_SUB_ADMINISTRATIVE_AREA: &[u8] = b"setSubAdministrativeArea:\0";
    pub const SEL_POSTAL_CODE: &[u8] = b"postalCode\0";
    pub const SEL_SET_POSTAL_CODE: &[u8] = b"setPostalCode:\0";
    pub const SEL_I_S_OCOUNTRY_CODE: &[u8] = b"ISOcountryCode\0";
    pub const SEL_SET_I_S_OCOUNTRY_CODE: &[u8] = b"setISOcountryCode:\0";
    pub const SEL_COUNTRY: &[u8] = b"country\0";
    pub const SEL_SET_COUNTRY: &[u8] = b"setCountry:\0";
    pub const SEL_INLAND_WATER: &[u8] = b"inlandWater\0";
    pub const SEL_SET_INLAND_WATER: &[u8] = b"setInlandWater:\0";
    pub const SEL_OCEAN: &[u8] = b"ocean\0";
    pub const SEL_SET_OCEAN: &[u8] = b"setOcean:\0";
    pub const SEL_AREAS_OF_INTEREST: &[u8] = b"areasOfInterest\0";
    pub const SEL_SET_AREAS_OF_INTEREST: &[u8] = b"setAreasOfInterest:\0";
}

// ── CLLocation (0 methods, 0 properties) ──
pub mod c_l_location {
}

// ── CLRegion (1 methods, 0 properties) ──
pub mod c_l_region {
    pub const SEL_CONTAINS_COORDINATE: &[u8] = b"containsCoordinate:\0";
}

// ── CLCircularRegion (1 methods, 2 properties) ──
pub mod c_l_circular_region {
    pub const SEL_CENTER: &[u8] = b"center\0";
    pub const SEL_SET_CENTER: &[u8] = b"setCenter:\0";
    pub const SEL_RADIUS: &[u8] = b"radius\0";
    pub const SEL_SET_RADIUS: &[u8] = b"setRadius:\0";
}

// Total: 80 selector constants
