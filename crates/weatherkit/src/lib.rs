#![allow(unsafe_op_in_unsafe_fn)]
//! Apple WeatherKit — comprehensive weather data from Rust.
//!
//! **Platform:** macOS 13+, iOS 16+, tvOS 16+, watchOS 9+.
//! Requires WeatherKit entitlement.
//!
//! ```ignore
//! weatherkit::fetch_current(37.7749, -122.4194, |result| {
//!     if let Ok(w) = result {
//!         println!("{:.1}°C, {}, UV {}, wind {:.1} m/s {:.0}°",
//!             w.temperature, w.condition_description,
//!             w.uv_index, w.wind_speed, w.wind_direction);
//!     }
//! });
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use core::ffi::c_void;

pub fn is_available() -> bool {
    unsafe {
        extern "C" { fn dlsym(h: *mut c_void, s: *const u8) -> *mut c_void; }
        let f = dlsym((-2isize) as *mut c_void, b"weatherkit_available\0".as_ptr());
        if f.is_null() { return false; }
        let func: unsafe extern "C" fn() -> bool = core::mem::transmute(f);
        func()
    }
}

// ── Types ───────────────────────────────────────────────────────────────────

/// Current weather conditions.
#[derive(Debug, Clone)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub apparent_temperature: f64,
    pub humidity: f64,
    pub dew_point: f64,
    pub wind_speed: f64,
    pub wind_gust: f64,
    pub wind_direction: f64,
    pub pressure: f64,
    pub pressure_trend: String,
    pub visibility: f64,
    pub uv_index: i32,
    pub cloud_cover: f64,
    pub condition: String,
    pub condition_description: String,
    pub symbol_name: String,
    pub is_daylight: bool,
}

/// Hourly forecast entry.
#[derive(Debug, Clone)]
pub struct HourlyForecast {
    pub date: String,
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub precipitation_chance: f64,
    pub condition: String,
    pub uv_index: i32,
    pub is_daylight: bool,
}

/// Daily forecast entry.
#[derive(Debug, Clone)]
pub struct DailyForecast {
    pub date: String,
    pub high_temperature: f64,
    pub low_temperature: f64,
    pub precipitation_chance: f64,
    pub precipitation_amount_mm: f64,
    pub snowfall_amount_cm: f64,
    pub condition: String,
    pub condition_description: String,
    pub uv_index_max: i32,
    pub wind_speed_max: f64,
    pub moon_phase: String,
    pub sunrise: String,
    pub sunset: String,
}

/// Weather alert.
#[derive(Debug, Clone)]
pub struct WeatherAlert {
    pub summary: String,
    pub severity: String,
    pub source: String,
    pub region: String,
}

/// Available weather data types for a location.
#[derive(Debug, Clone)]
pub struct WeatherAvailability {
    pub current: bool,
    pub hourly: bool,
    pub daily: bool,
    pub minute: bool,
    pub alerts: bool,
}

// ── JSON parser helper ──────────────────────────────────────────────────────

fn json_str(json: &str, key: &str) -> String {
    let pattern = format!("\"{}\":\"", key);
    if let Some(start) = json.find(&pattern) {
        let rest = &json[start + pattern.len()..];
        if let Some(end) = rest.find('"') {
            return rest[..end].to_string();
        }
    }
    String::new()
}

fn json_f64(json: &str, key: &str) -> f64 {
    let pattern = format!("\"{}\":", key);
    if let Some(start) = json.find(&pattern) {
        let rest = &json[start + pattern.len()..];
        let end = rest.find(|c: char| c == ',' || c == '}').unwrap_or(rest.len());
        return rest[..end].trim().parse().unwrap_or(0.0);
    }
    0.0
}

fn json_i32(json: &str, key: &str) -> i32 { json_f64(json, key) as i32 }

fn json_bool(json: &str, key: &str) -> bool {
    let pattern = format!("\"{}\":", key);
    if let Some(start) = json.find(&pattern) {
        let rest = &json[start + pattern.len()..];
        return rest.trim_start().starts_with("true");
    }
    false
}

// ── Callback trampoline ─────────────────────────────────────────────────────

type JsonCallback = Box<dyn FnOnce(Result<String, String>) + Send>;

unsafe extern "C" fn json_trampoline(
    ptr: *const u8, len: usize, success: bool, ud: *mut c_void,
) {
    let cb: Box<JsonCallback> = Box::from_raw(ud as *mut _);
    if success && len > 0 && !ptr.is_null() {
        let s = String::from_utf8_lossy(core::slice::from_raw_parts(ptr, len)).into_owned();
        cb(Ok(s));
    } else if !ptr.is_null() && len > 0 {
        let s = String::from_utf8_lossy(core::slice::from_raw_parts(ptr, len)).into_owned();
        cb(Err(s));
    } else {
        cb(Err("Weather fetch failed".into()));
    }
}

fn call_with_json<F: FnOnce(Result<String, String>) + Send + 'static>(
    f: impl FnOnce(unsafe extern "C" fn(*const u8, usize, bool, *mut c_void), *mut c_void),
    callback: F,
) {
    let cb: Box<JsonCallback> = Box::new(Box::new(callback));
    let ud = Box::into_raw(cb) as *mut c_void;
    f(json_trampoline, ud);
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Fetch current weather (async via callback).
pub fn fetch_current<F: FnOnce(Result<CurrentWeather, String>) + Send + 'static>(
    lat: f64, lon: f64, callback: F,
) {
    call_with_json(|cb, ud| unsafe {
        extern "C" {
            fn weatherkit_fetch_current(lat: f64, lon: f64,
                cb: unsafe extern "C" fn(*const u8, usize, bool, *mut c_void), ud: *mut c_void);
        }
        weatherkit_fetch_current(lat, lon, cb, ud);
    }, move |result| {
        callback(result.map(|json| CurrentWeather {
            temperature: json_f64(&json, "temperature"),
            apparent_temperature: json_f64(&json, "apparentTemperature"),
            humidity: json_f64(&json, "humidity"),
            dew_point: json_f64(&json, "dewPoint"),
            wind_speed: json_f64(&json, "windSpeed"),
            wind_gust: json_f64(&json, "windGust"),
            wind_direction: json_f64(&json, "windDirection"),
            pressure: json_f64(&json, "pressure"),
            pressure_trend: json_str(&json, "pressureTrend"),
            visibility: json_f64(&json, "visibility"),
            uv_index: json_i32(&json, "uvIndex"),
            cloud_cover: json_f64(&json, "cloudCover"),
            condition: json_str(&json, "condition"),
            condition_description: json_str(&json, "conditionDescription"),
            symbol_name: json_str(&json, "symbolName"),
            is_daylight: json_bool(&json, "isDaylight"),
        }));
    });
}

/// Fetch hourly forecast (async via callback).
pub fn fetch_hourly<F: FnOnce(Result<Vec<HourlyForecast>, String>) + Send + 'static>(
    lat: f64, lon: f64, hours: usize, callback: F,
) {
    call_with_json(|cb, ud| unsafe {
        extern "C" {
            fn weatherkit_fetch_hourly(lat: f64, lon: f64, hours: i32,
                cb: unsafe extern "C" fn(*const u8, usize, bool, *mut c_void), ud: *mut c_void);
        }
        weatherkit_fetch_hourly(lat, lon, hours as i32, cb, ud);
    }, move |result| {
        callback(result.map(|json| {
            parse_json_array(&json, |entry| HourlyForecast {
                date: json_str(entry, "date"),
                temperature: json_f64(entry, "temperature"),
                humidity: json_f64(entry, "humidity"),
                wind_speed: json_f64(entry, "windSpeed"),
                precipitation_chance: json_f64(entry, "precipitationChance"),
                condition: json_str(entry, "condition"),
                uv_index: json_i32(entry, "uvIndex"),
                is_daylight: json_bool(entry, "isDaylight"),
            })
        }));
    });
}

/// Fetch daily forecast (async via callback).
pub fn fetch_daily<F: FnOnce(Result<Vec<DailyForecast>, String>) + Send + 'static>(
    lat: f64, lon: f64, days: usize, callback: F,
) {
    call_with_json(|cb, ud| unsafe {
        extern "C" {
            fn weatherkit_fetch_daily(lat: f64, lon: f64, days: i32,
                cb: unsafe extern "C" fn(*const u8, usize, bool, *mut c_void), ud: *mut c_void);
        }
        weatherkit_fetch_daily(lat, lon, days as i32, cb, ud);
    }, move |result| {
        callback(result.map(|json| {
            parse_json_array(&json, |entry| DailyForecast {
                date: json_str(entry, "date"),
                high_temperature: json_f64(entry, "highTemperature"),
                low_temperature: json_f64(entry, "lowTemperature"),
                precipitation_chance: json_f64(entry, "precipitationChance"),
                precipitation_amount_mm: json_f64(entry, "precipitationAmount"),
                snowfall_amount_cm: json_f64(entry, "snowfallAmount"),
                condition: json_str(entry, "condition"),
                condition_description: json_str(entry, "conditionDescription"),
                uv_index_max: json_i32(entry, "uvIndexMax"),
                wind_speed_max: json_f64(entry, "windSpeedMax"),
                moon_phase: json_str(entry, "moonPhase"),
                sunrise: json_str(entry, "sunriseDate"),
                sunset: json_str(entry, "sunsetDate"),
            })
        }));
    });
}

/// Fetch weather alerts (async via callback).
pub fn fetch_alerts<F: FnOnce(Result<Vec<WeatherAlert>, String>) + Send + 'static>(
    lat: f64, lon: f64, callback: F,
) {
    call_with_json(|cb, ud| unsafe {
        extern "C" {
            fn weatherkit_fetch_alerts(lat: f64, lon: f64,
                cb: unsafe extern "C" fn(*const u8, usize, bool, *mut c_void), ud: *mut c_void);
        }
        weatherkit_fetch_alerts(lat, lon, cb, ud);
    }, move |result| {
        callback(result.map(|json| {
            parse_json_array(&json, |entry| WeatherAlert {
                summary: json_str(entry, "summary"),
                severity: json_str(entry, "severity"),
                source: json_str(entry, "source"),
                region: json_str(entry, "region"),
            })
        }));
    });
}

/// Check weather data availability for a location (async via callback).
pub fn check_availability<F: FnOnce(Result<WeatherAvailability, String>) + Send + 'static>(
    lat: f64, lon: f64, callback: F,
) {
    call_with_json(|cb, ud| unsafe {
        extern "C" {
            fn weatherkit_check_availability(lat: f64, lon: f64,
                cb: unsafe extern "C" fn(*const u8, usize, bool, *mut c_void), ud: *mut c_void);
        }
        weatherkit_check_availability(lat, lon, cb, ud);
    }, move |result| {
        callback(result.map(|json| WeatherAvailability {
            current: json.contains("current"),
            hourly: json.contains("hourly"),
            daily: json.contains("daily"),
            minute: json.contains("minute"),
            alerts: json.contains("alerts"),
        }));
    });
}

/// Get the WeatherKit legal attribution URL (async via callback).
pub fn attribution_legal_url<F: FnOnce(String) + Send + 'static>(callback: F) {
    unsafe extern "C" fn trampoline(ptr: *const u8, len: usize, ud: *mut c_void) {
        let cb: Box<Box<dyn FnOnce(String) + Send>> = Box::from_raw(ud as *mut _);
        let s = if len > 0 && !ptr.is_null() {
            String::from_utf8_lossy(core::slice::from_raw_parts(ptr, len)).into_owned()
        } else { String::new() };
        cb(s);
    }
    let cb: Box<Box<dyn FnOnce(String) + Send>> = Box::new(Box::new(callback));
    let ud = Box::into_raw(cb) as *mut c_void;
    unsafe {
        extern "C" {
            fn weatherkit_attribution_legal_page_url(
                cb: unsafe extern "C" fn(*const u8, usize, *mut c_void), ud: *mut c_void);
        }
        weatherkit_attribution_legal_page_url(trampoline, ud);
    }
}

/// Get the WeatherKit combined mark (logo) URL (async via callback).
pub fn attribution_mark_url<F: FnOnce(String) + Send + 'static>(callback: F) {
    unsafe extern "C" fn trampoline(ptr: *const u8, len: usize, ud: *mut c_void) {
        let cb: Box<Box<dyn FnOnce(String) + Send>> = Box::from_raw(ud as *mut _);
        let s = if len > 0 && !ptr.is_null() {
            String::from_utf8_lossy(core::slice::from_raw_parts(ptr, len)).into_owned()
        } else { String::new() };
        cb(s);
    }
    let cb: Box<Box<dyn FnOnce(String) + Send>> = Box::new(Box::new(callback));
    let ud = Box::into_raw(cb) as *mut c_void;
    unsafe {
        extern "C" {
            fn weatherkit_attribution_mark_url(
                cb: unsafe extern "C" fn(*const u8, usize, *mut c_void), ud: *mut c_void);
        }
        weatherkit_attribution_mark_url(trampoline, ud);
    }
}

// ── JSON array parser ───────────────────────────────────────────────────────

fn parse_json_array<T>(json: &str, parse_entry: impl Fn(&str) -> T) -> Vec<T> {
    let inner = json.trim().trim_start_matches('[').trim_end_matches(']');
    if inner.is_empty() { return vec![]; }
    // Split on },{ boundaries
    let mut results = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    for (i, c) in inner.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let entry = &inner[start..=i];
                    results.push(parse_entry(entry));
                    start = i + 1;
                    // Skip comma
                    if start < inner.len() && inner.as_bytes()[start] == b',' {
                        start += 1;
                    }
                }
            }
            _ => {}
        }
    }
    results
}

/// Well-known weather condition codes.
pub mod conditions {
    pub const CLEAR: &str = "clear";
    pub const CLOUDY: &str = "cloudy";
    pub const MOSTLY_CLEAR: &str = "mostlyClear";
    pub const MOSTLY_CLOUDY: &str = "mostlyCloudy";
    pub const PARTLY_CLOUDY: &str = "partlyCloudy";
    pub const RAIN: &str = "rain";
    pub const DRIZZLE: &str = "drizzle";
    pub const HEAVY_RAIN: &str = "heavyRain";
    pub const SNOW: &str = "snow";
    pub const HEAVY_SNOW: &str = "heavySnow";
    pub const SLEET: &str = "sleet";
    pub const FREEZING_RAIN: &str = "freezingRain";
    pub const HAIL: &str = "hail";
    pub const THUNDERSTORMS: &str = "thunderstorms";
    pub const FOGGY: &str = "foggy";
    pub const HAZE: &str = "haze";
    pub const SMOKY: &str = "smoky";
    pub const BREEZY: &str = "breezy";
    pub const WINDY: &str = "windy";
    pub const BLIZZARD: &str = "blizzard";
    pub const HOT: &str = "hot";
    pub const FRIGID: &str = "frigid";
}

/// Moon phase values.
pub mod moon_phases {
    pub const NEW: &str = "new";
    pub const WAXING_CRESCENT: &str = "waxingCrescent";
    pub const FIRST_QUARTER: &str = "firstQuarter";
    pub const WAXING_GIBBOUS: &str = "waxingGibbous";
    pub const FULL: &str = "full";
    pub const WANING_GIBBOUS: &str = "waningGibbous";
    pub const LAST_QUARTER: &str = "lastQuarter";
    pub const WANING_CRESCENT: &str = "waningCrescent";
}
