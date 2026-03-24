#![allow(unsafe_op_in_unsafe_fn)]
//! Apple CreateML — train ML models from Rust with full parameter control.
//!
//! **Platform:** macOS 10.15+ (macOS only).
//!
//! Every CreateML model type is supported with every configurable parameter
//! exposed via a builder API. No hidden defaults — you control everything.
//!
//! # Text classifier
//! ```ignore
//! createml::Train::text_classifier()
//!     .text_data(r#"{"positive":["great","love"],"negative":["bad","awful"]}"#)
//!     .language("en")
//!     .output("/tmp/sentiment.mlmodel")
//!     .author("My App")
//!     .run(|r| println!("{:?}", r));
//! ```
//!
//! # Boosted tree with full tuning
//! ```ignore
//! createml::Train::boosted_tree_classifier()
//!     .csv("/data/train.csv")
//!     .target("label")
//!     .features(&["age", "income", "score"])
//!     .max_iterations(200)
//!     .max_depth(8)
//!     .min_loss_reduction(0.1)
//!     .step_size(0.2)
//!     .row_subsample(0.8)
//!     .column_subsample(0.9)
//!     .early_stopping_rounds(10)
//!     .random_seed(42)
//!     .output("/tmp/model.mlmodel")
//!     .run(|r| println!("{:?}", r));
//! ```
//!
//! # Predict
//! ```ignore
//! let label = createml::Predict::text("/tmp/model.mlmodel", "I love this!");
//! let preds = createml::Predict::csv("/tmp/model.mlmodel", "/data/test.csv");
//! ```
//!
//! # Evaluate
//! ```ignore
//! let eval = createml::Evaluate::csv("/tmp/model.mlmodel", "/test.csv", "label");
//! println!("Accuracy: {:.2}%", eval.accuracy * 100.0);
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use core::ffi::c_void;
use std::collections::HashMap;

unsafe extern "C" { fn createml_available() -> bool; }
pub fn is_available() -> bool { unsafe { createml_available() } }

// ── Result types ────────────────────────────────────────────────────────────

/// Training result for classifiers.
#[derive(Debug, Clone)]
pub struct ClassifierResult {
    pub training_error: f64,
    pub validation_error: f64,
    pub training_valid: bool,
    pub validation_valid: bool,
    pub model_saved: bool,
}

/// Training result for regressors.
#[derive(Debug, Clone)]
pub struct RegressorResult {
    pub training_max_error: f64,
    pub training_rmse: f64,
    pub validation_max_error: f64,
    pub validation_rmse: f64,
    pub model_saved: bool,
}

/// Evaluation result.
#[derive(Debug, Clone)]
pub struct EvalResult {
    pub accuracy: f64,
    pub rmse: f64,
    pub correct: usize,
    pub total: usize,
    pub is_regressor: bool,
}

/// Model metadata.
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub inputs: Vec<(String, i64)>,
    pub outputs: Vec<(String, i64)>,
    pub predicted_feature: String,
}

/// Data table info.
#[derive(Debug, Clone)]
pub struct DataTableInfo {
    pub rows: usize,
    pub columns: usize,
    pub column_info: Vec<(String, String)>,
}

// ── JSON helpers ────────────────────────────────────────────────────────────

fn jf(j: &str, k: &str) -> f64 {
    let p = format!("\"{}\":", k);
    j.find(&p).and_then(|i| {
        let r = &j[i+p.len()..];
        r[..r.find(|c:char| c==',' || c=='}').unwrap_or(r.len())].trim().parse().ok()
    }).unwrap_or(0.0)
}
fn jb(j: &str, k: &str) -> bool {
    let p = format!("\"{}\":", k);
    j.find(&p).map(|i| j[i+p.len()..].trim_start().starts_with("true")).unwrap_or(false)
}
fn js(j: &str, k: &str) -> String {
    let p = format!("\"{}\":\"", k);
    j.find(&p).and_then(|i| j[i+p.len()..].find('"').map(|e| j[i+p.len()..i+p.len()+e].to_string())).unwrap_or_default()
}

// ── Callback machinery ──────────────────────────────────────────────────────

unsafe extern "C" fn result_trampoline(p: *const u8, l: usize, ok: bool, ud: *mut c_void) {
    let cb: Box<Box<dyn FnOnce(Result<String, String>) + Send>> = Box::from_raw(ud as *mut _);
    let s = if l > 0 && !p.is_null() {
        String::from_utf8_lossy(core::slice::from_raw_parts(p, l)).into_owned()
    } else { String::new() };
    cb(if ok { Ok(s) } else { Err(s) });
}

fn call_bridge<F: FnOnce(Result<String, String>) + Send + 'static>(
    json: &str, callback: F,
    bridge: unsafe extern "C" fn(*const u8, usize, unsafe extern "C" fn(*const u8, usize, bool, *mut c_void), *mut c_void),
) {
    let cb: Box<Box<dyn FnOnce(Result<String, String>) + Send>> = Box::new(Box::new(callback));
    let ud = Box::into_raw(cb) as *mut c_void;
    unsafe { bridge(json.as_ptr(), json.len(), result_trampoline, ud); }
}

unsafe extern "C" {
    fn createml_train(p: *const u8, l: usize, cb: unsafe extern "C" fn(*const u8, usize, bool, *mut c_void), ud: *mut c_void);
    fn createml_predict(p: *const u8, l: usize, b: *mut u8, bl: usize) -> isize;
    fn createml_evaluate(p: *const u8, l: usize, b: *mut u8, bl: usize) -> isize;
    fn createml_model_info(p: *const u8, l: usize, b: *mut u8, bl: usize) -> isize;
    fn createml_dt_info(p: *const u8, l: usize, b: *mut u8, bl: usize) -> isize;
    fn createml_dt_write_csv(i: *const u8, il: usize, o: *const u8, ol: usize) -> bool;
    fn createml_dt_split(p: *const u8, pl: usize, ratio: f64, seed: isize, to: *const u8, tol: usize, teo: *const u8, teol: usize) -> bool;
    fn createml_dt_drop_missing(i: *const u8, il: usize, o: *const u8, ol: usize) -> bool;
    fn createml_dt_drop_duplicates(i: *const u8, il: usize, o: *const u8, ol: usize) -> bool;
    fn createml_dt_sort(i: *const u8, il: usize, c: *const u8, cl: usize, asc: bool, o: *const u8, ol: usize) -> bool;
    fn createml_dt_prefix(i: *const u8, il: usize, n: isize, o: *const u8, ol: usize) -> bool;
    fn createml_dt_suffix(i: *const u8, il: usize, n: isize, o: *const u8, ol: usize) -> bool;
}

// ══════════════════════════════════════════════════════════════════════════
// Train — Builder API
// ══════════════════════════════════════════════════════════════════════════

/// Builder for training ML models. Every parameter is configurable.
pub struct Train {
    params: HashMap<String, serde_json_lite::Value>,
}

mod serde_json_lite {
    #[derive(Clone)]
    pub enum Value {
        Str(String),
        Int(i64),
        Float(f64),
        Bool(bool),
        Array(Vec<String>),
        Map(std::collections::HashMap<String, Vec<String>>),
        #[allow(dead_code)]
        Null,
    }

    impl Value {
        pub fn to_json(&self) -> String {
            match self {
                Value::Str(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
                Value::Int(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Array(a) => format!("[{}]", a.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")),
                Value::Map(m) => {
                    let entries: Vec<String> = m.iter().map(|(k, vs)| {
                        let vals = vs.iter().map(|v| format!("\"{}\"", v.replace('"', "\\\""))).collect::<Vec<_>>().join(",");
                        format!("\"{}\":[{}]", k, vals)
                    }).collect();
                    format!("{{{}}}", entries.join(","))
                }
                Value::Null => "null".into(),
            }
        }
    }
}

use serde_json_lite::Value as JV;

impl Train {
    fn new(model_type: &str) -> Self {
        let mut params = HashMap::new();
        params.insert("model_type".into(), JV::Str(model_type.into()));
        Self { params }
    }

    // ── Model type constructors ─────────────────────────────────────────

    pub fn text_classifier() -> Self { Self::new("text_classifier") }
    pub fn image_classifier() -> Self { Self::new("image_classifier") }
    pub fn sound_classifier() -> Self { Self::new("sound_classifier") }
    pub fn hand_pose_classifier() -> Self { Self::new("hand_pose_classifier") }
    pub fn hand_action_classifier() -> Self { Self::new("hand_action_classifier") }
    pub fn style_transfer() -> Self { Self::new("style_transfer") }
    pub fn boosted_tree_classifier() -> Self { Self::new("boosted_tree_classifier") }
    pub fn decision_tree_classifier() -> Self { Self::new("decision_tree_classifier") }
    pub fn random_forest_classifier() -> Self { Self::new("random_forest_classifier") }
    pub fn logistic_regression_classifier() -> Self { Self::new("logistic_regression_classifier") }
    pub fn boosted_tree_regressor() -> Self { Self::new("boosted_tree_regressor") }
    pub fn decision_tree_regressor() -> Self { Self::new("decision_tree_regressor") }
    pub fn random_forest_regressor() -> Self { Self::new("random_forest_regressor") }
    pub fn linear_regressor() -> Self { Self::new("linear_regressor") }

    // ── Data source ─────────────────────────────────────────────────────

    /// Path to training data (directory for image/sound, CSV for tabular).
    pub fn data(mut self, path: &str) -> Self { self.params.insert("training_data".into(), JV::Str(path.into())); self }
    /// Alias for `data()`.
    pub fn csv(self, path: &str) -> Self { self.data(path) }
    /// Target column for tabular models.
    pub fn target(mut self, column: &str) -> Self { self.params.insert("target_column".into(), JV::Str(column.into())); self }
    /// Feature columns (optional — all columns except target used if omitted).
    pub fn features(mut self, columns: &[&str]) -> Self { self.params.insert("feature_columns".into(), JV::Array(columns.iter().map(|s| s.to_string()).collect())); self }
    /// Text training data: `{"label": ["text1", "text2"]}`.
    pub fn text_data_json(mut self, json: &str) -> Self { self.params.insert("text_data_raw".into(), JV::Str(json.into())); self }
    /// Text training data from a map.
    pub fn text_data(mut self, data: &HashMap<&str, Vec<&str>>) -> Self {
        let map: HashMap<String, Vec<String>> = data.iter().map(|(k, vs)| (k.to_string(), vs.iter().map(|v| v.to_string()).collect())).collect();
        self.params.insert("text_data".into(), JV::Map(map));
        self
    }
    /// Style image path (for style transfer).
    pub fn style_image(mut self, path: &str) -> Self { self.params.insert("style_image".into(), JV::Str(path.into())); self }
    /// Content directory path (for style transfer).
    pub fn content_dir(mut self, path: &str) -> Self { self.params.insert("content_dir".into(), JV::Str(path.into())); self }

    // ── Output ──────────────────────────────────────────────────────────

    /// Save trained model to this path.
    pub fn output(mut self, path: &str) -> Self { self.params.insert("output_path".into(), JV::Str(path.into())); self }

    // ── Metadata ────────────────────────────────────────────────────────

    pub fn author(mut self, author: &str) -> Self { self.params.insert("author".into(), JV::Str(author.into())); self }
    pub fn description(mut self, desc: &str) -> Self { self.params.insert("description".into(), JV::Str(desc.into())); self }
    pub fn version(mut self, ver: &str) -> Self { self.params.insert("version".into(), JV::Str(ver.into())); self }

    // ── Common parameters ───────────────────────────────────────────────

    pub fn max_iterations(mut self, n: usize) -> Self { self.params.insert("max_iterations".into(), JV::Int(n as i64)); self }
    pub fn max_depth(mut self, n: usize) -> Self { self.params.insert("max_depth".into(), JV::Int(n as i64)); self }
    pub fn random_seed(mut self, seed: i64) -> Self { self.params.insert("random_seed".into(), JV::Int(seed)); self }

    // ── Boosted tree parameters ─────────────────────────────────────────

    pub fn min_loss_reduction(mut self, v: f64) -> Self { self.params.insert("min_loss_reduction".into(), JV::Float(v)); self }
    pub fn min_child_weight(mut self, v: f64) -> Self { self.params.insert("min_child_weight".into(), JV::Float(v)); self }
    pub fn step_size(mut self, v: f64) -> Self { self.params.insert("step_size".into(), JV::Float(v)); self }
    pub fn early_stopping_rounds(mut self, n: usize) -> Self { self.params.insert("early_stopping_rounds".into(), JV::Int(n as i64)); self }
    pub fn row_subsample(mut self, v: f64) -> Self { self.params.insert("row_subsample".into(), JV::Float(v)); self }
    pub fn column_subsample(mut self, v: f64) -> Self { self.params.insert("column_subsample".into(), JV::Float(v)); self }

    // ── Linear regressor parameters ─────────────────────────────────────

    pub fn l1_penalty(mut self, v: f64) -> Self { self.params.insert("l1_penalty".into(), JV::Float(v)); self }
    pub fn l2_penalty(mut self, v: f64) -> Self { self.params.insert("l2_penalty".into(), JV::Float(v)); self }
    pub fn convergence_threshold(mut self, v: f64) -> Self { self.params.insert("convergence_threshold".into(), JV::Float(v)); self }
    pub fn feature_rescaling(mut self, v: bool) -> Self { self.params.insert("feature_rescaling".into(), JV::Bool(v)); self }

    // ── Sound classifier parameters ─────────────────────────────────────

    pub fn overlap_factor(mut self, v: f64) -> Self { self.params.insert("overlap_factor".into(), JV::Float(v)); self }

    // ── Style transfer parameters ───────────────────────────────────────

    pub fn textel_density(mut self, v: usize) -> Self { self.params.insert("textel_density".into(), JV::Int(v as i64)); self }
    pub fn style_strength(mut self, v: usize) -> Self { self.params.insert("style_strength".into(), JV::Int(v as i64)); self }

    // ── Image classifier parameters ─────────────────────────────────────

    /// Image augmentation options: "rotation", "blur", "crop", "flip", "noise", "exposure".
    pub fn augmentation(mut self, options: &[&str]) -> Self {
        self.params.insert("augmentation".into(), JV::Array(options.iter().map(|s| s.to_string()).collect()));
        self
    }

    // ── Text classifier parameters ──────────────────────────────────────

    pub fn language(mut self, lang: &str) -> Self { self.params.insert("language".into(), JV::Str(lang.into())); self }

    // ── Execute ─────────────────────────────────────────────────────────

    fn to_json(&self) -> String {
        let entries: Vec<String> = self.params.iter().map(|(k, v)| {
            if k == "text_data_raw" {
                if let JV::Str(s) = v {
                    return format!("\"text_data\":{}", s);
                }
            }
            format!("\"{}\":{}", k, v.to_json())
        }).collect();
        format!("{{{}}}", entries.join(","))
    }

    /// Run the training (async via callback).
    pub fn run<F: FnOnce(Result<String, String>) + Send + 'static>(self, callback: F) {
        let json = self.to_json();
        call_bridge(&json, callback, createml_train);
    }

    /// Run and parse as classifier result.
    pub fn run_classifier<F: FnOnce(Result<ClassifierResult, String>) + Send + 'static>(self, callback: F) {
        self.run(move |result| {
            callback(result.map(|j| ClassifierResult {
                training_error: jf(&j, "training_error"),
                validation_error: jf(&j, "validation_error"),
                training_valid: jb(&j, "training_valid"),
                validation_valid: jb(&j, "validation_valid"),
                model_saved: jb(&j, "model_saved"),
            }));
        });
    }

    /// Run and parse as regressor result.
    pub fn run_regressor<F: FnOnce(Result<RegressorResult, String>) + Send + 'static>(self, callback: F) {
        self.run(move |result| {
            callback(result.map(|j| RegressorResult {
                training_max_error: jf(&j, "training_max_error"),
                training_rmse: jf(&j, "training_rmse"),
                validation_max_error: jf(&j, "validation_max_error"),
                validation_rmse: jf(&j, "validation_rmse"),
                model_saved: jb(&j, "model_saved"),
            }));
        });
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Predict
// ══════════════════════════════════════════════════════════════════════════

pub struct Predict;

impl Predict {
    /// Predict label for text.
    pub fn text(model_path: &str, text: &str) -> Option<String> {
        let json = format!("{{\"model_path\":\"{}\",\"text\":\"{}\"}}", model_path, text.replace('"', "\\\""));
        let mut buf = vec![0u8; 65536];
        let len = unsafe { createml_predict(json.as_ptr(), json.len(), buf.as_mut_ptr(), buf.len()) };
        if len < 0 { None } else { Some(String::from_utf8_lossy(&buf[..len as usize]).into_owned()) }
    }

    /// Batch predict from CSV. Returns JSON array of predictions.
    pub fn csv(model_path: &str, csv_path: &str) -> Option<String> {
        let json = format!("{{\"model_path\":\"{}\",\"csv_path\":\"{}\"}}", model_path, csv_path);
        let mut buf = vec![0u8; 1 << 20]; // 1MB
        let len = unsafe { createml_predict(json.as_ptr(), json.len(), buf.as_mut_ptr(), buf.len()) };
        if len < 0 { None } else { Some(String::from_utf8_lossy(&buf[..len as usize]).into_owned()) }
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Evaluate
// ══════════════════════════════════════════════════════════════════════════

pub struct Evaluate;

impl Evaluate {
    /// Evaluate model on test CSV with target column.
    pub fn csv(model_path: &str, csv_path: &str, target_column: &str) -> Option<EvalResult> {
        let json = format!("{{\"model_path\":\"{}\",\"csv_path\":\"{}\",\"target_column\":\"{}\"}}",
            model_path, csv_path, target_column);
        let mut buf = vec![0u8; 4096];
        let len = unsafe { createml_evaluate(json.as_ptr(), json.len(), buf.as_mut_ptr(), buf.len()) };
        if len < 0 { return None; }
        let j = String::from_utf8_lossy(&buf[..len as usize]);
        let is_reg = js(&j, "type") == "regressor";
        Some(EvalResult {
            accuracy: jf(&j, "accuracy"),
            rmse: jf(&j, "rmse"),
            correct: jf(&j, "correct") as usize,
            total: jf(&j, "total") as usize,
            is_regressor: is_reg,
        })
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Model info
// ══════════════════════════════════════════════════════════════════════════

/// Inspect a compiled model.
pub fn model_info(path: &str) -> Option<ModelInfo> {
    let mut buf = vec![0u8; 65536];
    let len = unsafe { createml_model_info(path.as_ptr(), path.len(), buf.as_mut_ptr(), buf.len()) };
    if len < 0 { return None; }
    let j = String::from_utf8_lossy(&buf[..len as usize]);
    Some(ModelInfo {
        predicted_feature: js(&j, "predicted_feature"),
        inputs: vec![], // TODO: parse JSON array
        outputs: vec![],
    })
}

// ══════════════════════════════════════════════════════════════════════════
// DataTable
// ══════════════════════════════════════════════════════════════════════════

pub struct DataTable;

impl DataTable {
    /// Load and inspect a CSV/JSON file.
    pub fn info(path: &str) -> Option<DataTableInfo> {
        let mut buf = vec![0u8; 65536];
        let len = unsafe { createml_dt_info(path.as_ptr(), path.len(), buf.as_mut_ptr(), buf.len()) };
        if len < 0 { return None; }
        let j = String::from_utf8_lossy(&buf[..len as usize]);
        Some(DataTableInfo {
            rows: jf(&j, "rows") as usize,
            columns: jf(&j, "columns") as usize,
            column_info: vec![],
        })
    }

    /// Convert to CSV.
    pub fn write_csv(input: &str, output: &str) -> bool {
        unsafe { createml_dt_write_csv(input.as_ptr(), input.len(), output.as_ptr(), output.len()) }
    }

    /// Split into train/test sets.
    pub fn split(path: &str, ratio: f64, seed: i64, train_out: &str, test_out: &str) -> bool {
        unsafe { createml_dt_split(path.as_ptr(), path.len(), ratio, seed as isize,
            train_out.as_ptr(), train_out.len(), test_out.as_ptr(), test_out.len()) }
    }

    /// Remove rows with missing values.
    pub fn drop_missing(input: &str, output: &str) -> bool {
        unsafe { createml_dt_drop_missing(input.as_ptr(), input.len(), output.as_ptr(), output.len()) }
    }

    /// Remove duplicate rows.
    pub fn drop_duplicates(input: &str, output: &str) -> bool {
        unsafe { createml_dt_drop_duplicates(input.as_ptr(), input.len(), output.as_ptr(), output.len()) }
    }

    /// Sort by column.
    pub fn sort(input: &str, column: &str, ascending: bool, output: &str) -> bool {
        unsafe { createml_dt_sort(input.as_ptr(), input.len(), column.as_ptr(), column.len(),
            ascending, output.as_ptr(), output.len()) }
    }

    /// Take first N rows.
    pub fn prefix(input: &str, n: usize, output: &str) -> bool {
        unsafe { createml_dt_prefix(input.as_ptr(), input.len(), n as isize, output.as_ptr(), output.len()) }
    }

    /// Take last N rows.
    pub fn suffix(input: &str, n: usize, output: &str) -> bool {
        unsafe { createml_dt_suffix(input.as_ptr(), input.len(), n as isize, output.as_ptr(), output.len()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available() { assert!(is_available()); }

    #[test]
    fn test_builder_json() {
        let json = Train::boosted_tree_classifier()
            .csv("/data/train.csv")
            .target("label")
            .max_iterations(100)
            .max_depth(6)
            .step_size(0.3)
            .to_json();
        assert!(json.contains("\"model_type\":\"boosted_tree_classifier\""));
        assert!(json.contains("\"max_iterations\":100"));
        assert!(json.contains("\"max_depth\":6"));
        assert!(json.contains("\"step_size\":0.3"));
    }

    #[test]
    fn test_builder_all_params() {
        let json = Train::linear_regressor()
            .csv("/data.csv")
            .target("price")
            .features(&["sqft", "beds"])
            .max_iterations(50)
            .l1_penalty(0.01)
            .l2_penalty(0.1)
            .step_size(0.5)
            .convergence_threshold(0.001)
            .feature_rescaling(true)
            .author("Test")
            .version("2.0")
            .output("/tmp/model.mlmodel")
            .to_json();
        assert!(json.contains("\"l1_penalty\":0.01"));
        assert!(json.contains("\"convergence_threshold\":0.001"));
        assert!(json.contains("\"feature_rescaling\":true"));
        assert!(json.contains("\"feature_columns\":[\"sqft\",\"beds\"]"));
    }
}
