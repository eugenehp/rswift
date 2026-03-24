//! ObjC selector constants for MapKit.
#![allow(dead_code)]

// ── MKMapView (27 methods, 12 properties) ──
pub mod m_k_map_view {
    pub const CLASS: &[u8] = b"MKMapView\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_REGION: &[u8] = b"region\0";
    pub const SEL_SET_REGION: &[u8] = b"setRegion:\0";
    pub const SEL_CENTER_COORDINATE: &[u8] = b"centerCoordinate\0";
    pub const SEL_SET_CENTER_COORDINATE: &[u8] = b"setCenterCoordinate:\0";
    pub const SEL_VISIBLE_MAP_RECT: &[u8] = b"visibleMapRect\0";
    pub const SEL_SET_VISIBLE_MAP_RECT: &[u8] = b"setVisibleMapRect:\0";
    pub const SEL_ZOOM_ENABLED: &[u8] = b"zoomEnabled\0";
    pub const SEL_SET_ZOOM_ENABLED: &[u8] = b"setZoomEnabled:\0";
    pub const SEL_SCROLL_ENABLED: &[u8] = b"scrollEnabled\0";
    pub const SEL_SET_SCROLL_ENABLED: &[u8] = b"setScrollEnabled:\0";
    pub const SEL_SHOWS_USER_LOCATION: &[u8] = b"showsUserLocation\0";
    pub const SEL_SET_SHOWS_USER_LOCATION: &[u8] = b"setShowsUserLocation:\0";
    pub const SEL_USER_LOCATION: &[u8] = b"userLocation\0";
    pub const SEL_SET_USER_LOCATION: &[u8] = b"setUserLocation:\0";
    pub const SEL_USER_LOCATION_VISIBLE: &[u8] = b"userLocationVisible\0";
    pub const SEL_SET_USER_LOCATION_VISIBLE: &[u8] = b"setUserLocationVisible:\0";
    pub const SEL_ANNOTATIONS: &[u8] = b"annotations\0";
    pub const SEL_SET_ANNOTATIONS: &[u8] = b"setAnnotations:\0";
    pub const SEL_SELECTED_ANNOTATIONS: &[u8] = b"selectedAnnotations\0";
    pub const SEL_SET_SELECTED_ANNOTATIONS: &[u8] = b"setSelectedAnnotations:\0";
    pub const SEL_ANNOTATION_VISIBLE_RECT: &[u8] = b"annotationVisibleRect\0";
    pub const SEL_SET_ANNOTATION_VISIBLE_RECT: &[u8] = b"setAnnotationVisibleRect:\0";
    pub const SEL_REGION_THAT_FITS: &[u8] = b"regionThatFits:\0";
    pub const SEL_MAP_RECT_THAT_FITS: &[u8] = b"mapRectThatFits:\0";
    pub const SEL_SET_CAMERA: &[u8] = b"setCamera:animated:\0";
    pub const SEL_SET_CAMERA_ZOOM_RANGE: &[u8] = b"setCameraZoomRange:animated:\0";
    pub const SEL_SET_CAMERA_BOUNDARY: &[u8] = b"setCameraBoundary:animated:\0";
    pub const SEL_CONVERT_COORDINATE: &[u8] = b"convertCoordinate:toPointToView:\0";
    pub const SEL_CONVERT_POINT: &[u8] = b"convertPoint:toCoordinateFromView:\0";
    pub const SEL_CONVERT_REGION: &[u8] = b"convertRegion:toRectToView:\0";
    pub const SEL_CONVERT_RECT: &[u8] = b"convertRect:toRegionFromView:\0";
    pub const SEL_SET_USER_TRACKING_MODE: &[u8] = b"setUserTrackingMode:animated:\0";
    pub const SEL_ADD_ANNOTATION: &[u8] = b"addAnnotation:\0";
    pub const SEL_ADD_ANNOTATIONS: &[u8] = b"addAnnotations:\0";
    pub const SEL_REMOVE_ANNOTATION: &[u8] = b"removeAnnotation:\0";
    pub const SEL_REMOVE_ANNOTATIONS: &[u8] = b"removeAnnotations:\0";
    pub const SEL_ANNOTATIONS_IN_MAP_RECT: &[u8] = b"annotationsInMapRect:\0";
    pub const SEL_VIEW_FOR_ANNOTATION: &[u8] = b"viewForAnnotation:\0";
    pub const SEL_DEQUEUE_REUSABLE_ANNOTATION_VIEW_WITH_IDENTIFIER: &[u8] = b"dequeueReusableAnnotationViewWithIdentifier:\0";
    pub const SEL_REGISTER_CLASS: &[u8] = b"registerClass:forAnnotationViewWithReuseIdentifier:\0";
    pub const SEL_SELECT_ANNOTATION: &[u8] = b"selectAnnotation:animated:\0";
    pub const SEL_DESELECT_ANNOTATION: &[u8] = b"deselectAnnotation:animated:\0";
    pub const SEL_SHOW_ANNOTATIONS: &[u8] = b"showAnnotations:animated:\0";
}

// ── MKMapSnapshotter (3 methods, 1 properties) ──
pub mod m_k_map_snapshotter {
    pub const SEL_LOADING: &[u8] = b"loading\0";
    pub const SEL_SET_LOADING: &[u8] = b"setLoading:\0";
    pub const SEL_START_WITH_COMPLETION_HANDLER: &[u8] = b"startWithCompletionHandler:\0";
    pub const SEL_START_WITH_QUEUE: &[u8] = b"startWithQueue:completionHandler:\0";
    pub const SEL_CANCEL: &[u8] = b"cancel\0";
}

// ── MKMapSnapshotOptions (0 methods, 4 properties) ──
pub mod m_k_map_snapshot_options {
    pub const SEL_CAMERA: &[u8] = b"camera\0";
    pub const SEL_MAP_RECT: &[u8] = b"mapRect\0";
    pub const SEL_SET_MAP_RECT: &[u8] = b"setMapRect:\0";
    pub const SEL_SIZE: &[u8] = b"size\0";
    pub const SEL_SET_SIZE: &[u8] = b"setSize:\0";
}

// ── MKDirections (3 methods, 1 properties) ──
pub mod m_k_directions {
    pub const SEL_CALCULATING: &[u8] = b"calculating\0";
    pub const SEL_SET_CALCULATING: &[u8] = b"setCalculating:\0";
    pub const SEL_CALCULATE_DIRECTIONS_WITH_COMPLETION_HANDLER: &[u8] = b"calculateDirectionsWithCompletionHandler:\0";
    pub const SEL_CALCULATE_E_T_A_WITH_COMPLETION_HANDLER: &[u8] = b"calculateETAWithCompletionHandler:\0";
}

// ── MKDirectionsRequest (2 methods, 2 properties) ──
pub mod m_k_directions_request {
    pub const SEL_SOURCE: &[u8] = b"source\0";
    pub const SEL_SET_SOURCE: &[u8] = b"setSource:\0";
    pub const SEL_DESTINATION: &[u8] = b"destination\0";
    pub const SEL_SET_DESTINATION: &[u8] = b"setDestination:\0";
}

// ── MKPointAnnotation (0 methods, 0 properties) ──
pub mod m_k_point_annotation {
}

// ── MKLocalSearch (2 methods, 1 properties) ──
pub mod m_k_local_search {
    pub const SEL_SEARCHING: &[u8] = b"searching\0";
    pub const SEL_SET_SEARCHING: &[u8] = b"setSearching:\0";
}

// ── MKLocalSearchRequest (0 methods, 0 properties) ──
pub mod m_k_local_search_request {
}

// ── MKPlacemark (0 methods, 1 properties) ──
pub mod m_k_placemark {
    pub const SEL_COUNTRY_CODE: &[u8] = b"countryCode\0";
    pub const SEL_SET_COUNTRY_CODE: &[u8] = b"setCountryCode:\0";
}

// Total: 78 selector constants
