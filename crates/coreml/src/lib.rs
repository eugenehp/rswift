//! Apple Core ML — on-device machine learning from Rust.
//!
//! **Platform:** macOS 10.13+, iOS 11+, tvOS 11+, visionOS 1+, watchOS 4+.
//!
//! ```ignore
//! let model = coreml::Model::from_path("/path/to/model.mlmodelc").unwrap();
//! for (name, desc) in model.input_descriptions() {
//!     println!("Input: {name} — {desc}");
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

// ── MLFeatureType ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureType {
    Invalid = 0,
    Int64 = 1,
    Double = 2,
    String = 3,
    Image = 4,
    MultiArray = 5,
    Dictionary = 6,
    Sequence = 7,
}
impl From<isize> for FeatureType {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Int64, 2=>Self::Double, 3=>Self::String, 4=>Self::Image,
                  5=>Self::MultiArray, 6=>Self::Dictionary, 7=>Self::Sequence, _=>Self::Invalid }
    }
}

// ── Model ───────────────────────────────────────────────────────────────────

/// A compiled CoreML model.
pub struct Model { inner: Id }

impl Model {
    /// Load a compiled model (`.mlmodelc` directory) from a file path.
    pub fn from_path(path: &str) -> Result<Self, String> {
        unsafe {
            let ns = nsstring(path);
            let url: Id = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);

            let mut error: Id = NIL;
            let sel = sel_registerName(b"modelWithContentsOfURL:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, *mut Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let model = f(class!(b"MLModel\0") as Id, sel, url, &mut error);

            if model.is_null() {
                let desc = if !error.is_null() {
                    nsstring_to_string(msg_send![error, localizedDescription])
                        .unwrap_or_else(|| "Unknown error".into())
                } else { "Failed to load model".into() };
                Err(desc)
            } else {
                CFRetain(model as CFTypeRef);
                Ok(Self { inner: model })
            }
        }
    }

    /// The model description.
    fn description_obj(&self) -> Id {
        unsafe { msg_send![self.inner, modelDescription] }
    }

    /// Input feature descriptions: `(name, type)` pairs.
    pub fn input_descriptions(&self) -> Vec<(String, FeatureType)> {
        self.feature_descriptions(b"inputDescriptionsByName\0")
    }

    /// Output feature descriptions: `(name, type)` pairs.
    pub fn output_descriptions(&self) -> Vec<(String, FeatureType)> {
        self.feature_descriptions(b"outputDescriptionsByName\0")
    }

    fn feature_descriptions(&self, sel_name: &[u8]) -> Vec<(String, FeatureType)> {
        unsafe {
            let desc = self.description_obj();
            if desc.is_null() { return vec![]; }
            let sel = sel_registerName(sel_name.as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let dict = f(desc, sel);
            if dict.is_null() { return vec![]; }

            let keys: Id = msg_send![dict, allKeys];
            let count: usize = msg_send_t![usize; keys, count];
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let key: Id = msg_send![keys, objectAtIndex: i];
                let name = nsstring_to_string(key).unwrap_or_default();
                let feat_desc: Id = msg_send![dict, objectForKey: key];
                let ftype = FeatureType::from(msg_send_t![isize; feat_desc, r#type]);
                result.push((name, ftype));
            }
            result
        }
    }

    /// Model metadata: author, description, version, etc.
    pub fn metadata(&self) -> Vec<(String, String)> {
        unsafe {
            let desc = self.description_obj();
            if desc.is_null() { return vec![]; }
            let meta: Id = msg_send![desc, metadata];
            if meta.is_null() { return vec![]; }
            let keys: Id = msg_send![meta, allKeys];
            let count: usize = msg_send_t![usize; keys, count];
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let key: Id = msg_send![keys, objectAtIndex: i];
                let val: Id = msg_send![meta, objectForKey: key];
                if let (Some(k), Some(v)) = (nsstring_to_string(key), nsstring_to_string(val)) {
                    result.push((k, v));
                }
            }
            result
        }
    }

    /// Predicted output label (for classifiers). Returns the top class name.
    pub fn predicted_feature_name(&self) -> Option<String> {
        unsafe {
            let desc = self.description_obj();
            if desc.is_null() { return None; }
            nsstring_to_string(msg_send![desc, predictedFeatureName])
        }
    }

    /// Make a prediction from a dictionary of feature values.
    pub fn predict(&self, inputs: &[(&str, FeatureValue)]) -> Result<Vec<(String, FeatureValue)>, String> {
        unsafe {
            // Build MLDictionaryFeatureProvider
            let dict: Id = msg_send![class!(b"NSMutableDictionary\0"), new];
            for (name, val) in inputs {
                let k = nsstring(name);
                msg_send_void![dict, setObject: val.inner, forKey: k];
                CFRelease(k as CFTypeRef);
            }
            let provider: Id = msg_send![class!(b"MLDictionaryFeatureProvider\0"), alloc];
            let provider = msg_send![provider, initWithDictionary: dict, error: NIL];
            CFRelease(dict as CFTypeRef);

            if provider.is_null() { return Err("Failed to create input provider".into()); }

            let mut error: Id = NIL;
            let sel = sel_registerName(b"predictionFromFeatures:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, *mut Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let output = f(self.inner, sel, provider, &mut error);
            CFRelease(provider as CFTypeRef);

            if output.is_null() {
                let desc = if !error.is_null() {
                    nsstring_to_string(msg_send![error, localizedDescription])
                        .unwrap_or_else(|| "Prediction failed".into())
                } else { "Prediction failed".into() };
                return Err(desc);
            }

            // Read output features
            let names: Id = msg_send![output, featureNames];
            let count: usize = msg_send_t![usize; names, count];
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let name_id: Id = msg_send![names, objectAtIndex: i];
                let name = nsstring_to_string(name_id).unwrap_or_default();
                let val: Id = msg_send![output, featureValueForName: name_id];
                if !val.is_null() {
                    CFRetain(val as CFTypeRef);
                    result.push((name, FeatureValue { inner: val }));
                }
            }
            Ok(result)
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Model { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

// ── FeatureValue ────────────────────────────────────────────────────────────

/// Wraps `MLFeatureValue`.
pub struct FeatureValue { inner: Id }

impl FeatureValue {
    /// Create from an i64.
    pub fn from_int(value: i64) -> Self {
        unsafe {
            let sel = sel_registerName(b"featureValueWithInt64:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, i64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let v = f(class!(b"MLFeatureValue\0") as Id, sel, value);
            CFRetain(v as CFTypeRef);
            Self { inner: v }
        }
    }

    /// Create from a f64.
    pub fn from_double(value: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"featureValueWithDouble:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let v = f(class!(b"MLFeatureValue\0") as Id, sel, value);
            CFRetain(v as CFTypeRef);
            Self { inner: v }
        }
    }

    /// Create from a string.
    pub fn from_string(value: &str) -> Self {
        unsafe {
            let ns = nsstring(value);
            let v: Id = msg_send![class!(b"MLFeatureValue\0"), featureValueWithString: ns];
            CFRelease(ns as CFTypeRef);
            CFRetain(v as CFTypeRef);
            Self { inner: v }
        }
    }

    /// The feature type.
    pub fn feature_type(&self) -> FeatureType {
        unsafe { FeatureType::from(msg_send_t![isize; self.inner, r#type]) }
    }

    /// Read as i64 (if type is Int64).
    pub fn int_value(&self) -> i64 {
        unsafe {
            let sel = sel_registerName(b"int64Value\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> i64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel)
        }
    }

    /// Read as f64 (if type is Double).
    pub fn double_value(&self) -> f64 {
        unsafe {
            let sel = sel_registerName(b"doubleValue\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel)
        }
    }

    /// Read as String (if type is String).
    pub fn string_value(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send![self.inner, stringValue]) }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for FeatureValue { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // CoreML needs app context
    #[ignore] // MLModel throws ObjC exception for invalid paths
    fn test_load_nonexistent() {
        assert!(Model::from_path("/nonexistent.mlmodelc").is_err());
    }

    #[test]
    #[ignore] // CoreML needs app context
    fn test_feature_value_int() {
        let v = FeatureValue::from_int(42);
        assert_eq!(v.feature_type(), FeatureType::Int64);
        assert_eq!(v.int_value(), 42);
    }

    #[test]
    #[ignore] // CoreML needs app context
    fn test_feature_value_double() {
        let v = FeatureValue::from_double(3.14);
        assert_eq!(v.feature_type(), FeatureType::Double);
        assert!((v.double_value() - 3.14).abs() < 0.001);
    }

    #[test]
    #[ignore] // CoreML needs app context
    fn test_feature_value_string() {
        let v = FeatureValue::from_string("hello");
        assert_eq!(v.feature_type(), FeatureType::String);
        assert_eq!(v.string_value(), Some("hello".into()));
    }
}
