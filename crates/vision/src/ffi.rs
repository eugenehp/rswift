//! ObjC selector constants for Vision.
#![allow(dead_code)]

// ── VNRecognizeTextRequest (2 methods, 6 properties) ──
pub mod v_n_recognize_text_request {
    pub const CLASS: &[u8] = b"VNRecognizeTextRequest\0";
    pub const SEL_RECOGNITION_LANGUAGES: &[u8] = b"recognitionLanguages\0";
    pub const SEL_SET_RECOGNITION_LANGUAGES: &[u8] = b"setRecognitionLanguages:\0";
    pub const SEL_CUSTOM_WORDS: &[u8] = b"customWords\0";
    pub const SEL_SET_CUSTOM_WORDS: &[u8] = b"setCustomWords:\0";
    pub const SEL_RECOGNITION_LEVEL: &[u8] = b"recognitionLevel\0";
    pub const SEL_SET_RECOGNITION_LEVEL: &[u8] = b"setRecognitionLevel:\0";
    pub const SEL_USES_LANGUAGE_CORRECTION: &[u8] = b"usesLanguageCorrection\0";
    pub const SEL_SET_USES_LANGUAGE_CORRECTION: &[u8] = b"setUsesLanguageCorrection:\0";
    pub const SEL_MINIMUM_TEXT_HEIGHT: &[u8] = b"minimumTextHeight\0";
    pub const SEL_SET_MINIMUM_TEXT_HEIGHT: &[u8] = b"setMinimumTextHeight:\0";
    pub const SEL_RESULTS: &[u8] = b"results\0";
    pub const SEL_SET_RESULTS: &[u8] = b"setResults:\0";
    pub const SEL_SUPPORTED_RECOGNITION_LANGUAGES_FOR_TEXT_RECOGNITION_LEVEL: &[u8] = b"supportedRecognitionLanguagesForTextRecognitionLevel:revision:error:\0";
    pub const SEL_SUPPORTED_RECOGNITION_LANGUAGES_AND_RETURN_ERROR: &[u8] = b"supportedRecognitionLanguagesAndReturnError:\0";
}

// ── VNDetectFaceRectanglesRequest (0 methods, 1 properties) ──
pub mod v_n_detect_face_rectangles_request {
}

// ── VNDetectBarcodesRequest (1 methods, 2 properties) ──
pub mod v_n_detect_barcodes_request {
    pub const SEL_SYMBOLOGIES: &[u8] = b"symbologies\0";
    pub const SEL_SET_SYMBOLOGIES: &[u8] = b"setSymbologies:\0";
    pub const SEL_SUPPORTED_SYMBOLOGIES_AND_RETURN_ERROR: &[u8] = b"supportedSymbologiesAndReturnError:\0";
}

// ── VNImageRequestHandler (1 methods, 0 properties) ──
pub mod v_n_image_request_handler {
    pub const SEL_PERFORM_REQUESTS: &[u8] = b"performRequests:error:\0";
}

// ── VNSequenceRequestHandler (12 methods, 0 properties) ──
pub mod v_n_sequence_request_handler {
}

// ── VNObservation (0 methods, 2 properties) ──
pub mod v_n_observation {
    pub const SEL_UUID: &[u8] = b"uuid\0";
    pub const SEL_SET_UUID: &[u8] = b"setUuid:\0";
    pub const SEL_CONFIDENCE: &[u8] = b"confidence\0";
    pub const SEL_SET_CONFIDENCE: &[u8] = b"setConfidence:\0";
}

// ── VNRecognizedTextObservation (1 methods, 0 properties) ──
pub mod v_n_recognized_text_observation {
    pub const SEL_TOP_CANDIDATES: &[u8] = b"topCandidates:\0";
}

// Total: 28 selector constants
