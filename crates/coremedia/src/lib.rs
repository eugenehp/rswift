//! Apple CoreMedia — CMTime and media timing from Rust.
//!
//! **Platform support:** macOS 10.7+, iOS 4+, tvOS 9+, visionOS 1+.
//!
//! # Quick start
//!
//! ```ignore
//! let t1 = coremedia::CMTime::new(3, 1); // 3 seconds
//! let t2 = coremedia::CMTime::from_seconds(1.5);
//! let sum = t1.add(&t2);
//! println!("{:.1}s", sum.seconds()); // 4.5s
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"coremedia_available");

unsafe extern "C" {
    fn coremedia_time_make(v: i64, ts: i32, ov: *mut i64, ots: *mut i32);
    fn coremedia_time_make_with_seconds(s: f64, ts: i32, ov: *mut i64, ots: *mut i32);
    fn coremedia_time_get_seconds(v: i64, ts: i32) -> f64;
    fn coremedia_time_add(v1: i64, ts1: i32, v2: i64, ts2: i32, ov: *mut i64, ots: *mut i32);
    fn coremedia_time_subtract(v1: i64, ts1: i32, v2: i64, ts2: i32, ov: *mut i64, ots: *mut i32);
    fn coremedia_time_compare(v1: i64, ts1: i32, v2: i64, ts2: i32) -> i32;
    fn coremedia_time_is_valid(v: i64, ts: i32) -> bool;
    fn coremedia_time_is_indefinite(v: i64, ts: i32) -> bool;
}

/// A time value with rational timescale (wraps `CMTime`).
#[derive(Debug, Clone, Copy)]
pub struct CMTime {
    pub value: i64,
    pub timescale: i32,
}

impl CMTime {
    /// Create a time from a value and timescale.
    /// E.g. `CMTime::new(3000, 600)` = 5 seconds.
    pub fn new(value: i64, timescale: i32) -> Self {
        let mut v = 0i64;
        let mut ts = 0i32;
        unsafe { coremedia_time_make(value, timescale, &mut v, &mut ts) }
        Self { value: v, timescale: ts }
    }

    /// Create a time from seconds with a preferred timescale.
    pub fn from_seconds(seconds: f64) -> Self {
        Self::from_seconds_with_timescale(seconds, 600)
    }

    /// Create a time from seconds with a specific timescale.
    pub fn from_seconds_with_timescale(seconds: f64, timescale: i32) -> Self {
        let mut v = 0i64;
        let mut ts = 0i32;
        unsafe { coremedia_time_make_with_seconds(seconds, timescale, &mut v, &mut ts) }
        Self { value: v, timescale: ts }
    }

    /// Convert to seconds.
    pub fn seconds(&self) -> f64 {
        unsafe { coremedia_time_get_seconds(self.value, self.timescale) }
    }

    /// Add two times.
    pub fn add(&self, other: &CMTime) -> CMTime {
        let mut v = 0i64;
        let mut ts = 0i32;
        unsafe { coremedia_time_add(self.value, self.timescale, other.value, other.timescale, &mut v, &mut ts) }
        CMTime { value: v, timescale: ts }
    }

    /// Subtract another time.
    pub fn subtract(&self, other: &CMTime) -> CMTime {
        let mut v = 0i64;
        let mut ts = 0i32;
        unsafe { coremedia_time_subtract(self.value, self.timescale, other.value, other.timescale, &mut v, &mut ts) }
        CMTime { value: v, timescale: ts }
    }

    /// Whether this time is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { coremedia_time_is_valid(self.value, self.timescale) }
    }

    /// Whether this time is indefinite (e.g. live stream).
    pub fn is_indefinite(&self) -> bool {
        unsafe { coremedia_time_is_indefinite(self.value, self.timescale) }
    }
}

impl PartialEq for CMTime {
    fn eq(&self, other: &Self) -> bool {
        unsafe { coremedia_time_compare(self.value, self.timescale, other.value, other.timescale) == 0 }
    }
}

impl PartialOrd for CMTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let c = unsafe { coremedia_time_compare(self.value, self.timescale, other.value, other.timescale) };
        Some(c.cmp(&0))
    }
}

impl std::ops::Add for CMTime {
    type Output = CMTime;
    fn add(self, rhs: Self) -> Self::Output { CMTime::add(&self, &rhs) }
}

impl std::ops::Sub for CMTime {
    type Output = CMTime;
    fn sub(self, rhs: Self) -> Self::Output { CMTime::subtract(&self, &rhs) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_seconds() {
        let t = CMTime::from_seconds(2.5);
        assert!((t.seconds() - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_add() {
        let a = CMTime::from_seconds(1.0);
        let b = CMTime::from_seconds(2.0);
        assert!((( a + b).seconds() - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_sub() {
        let a = CMTime::from_seconds(5.0);
        let b = CMTime::from_seconds(2.0);
        assert!(((a - b).seconds() - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_compare() {
        let a = CMTime::from_seconds(1.0);
        let b = CMTime::from_seconds(2.0);
        assert!(a < b);
        assert!(b > a);
        assert!(a == a);
    }

    #[test]
    fn test_valid() {
        let t = CMTime::from_seconds(1.0);
        assert!(t.is_valid());
        assert!(!t.is_indefinite());
    }

    #[test]
    fn test_new() {
        let t = CMTime::new(3000, 600);
        assert!((t.seconds() - 5.0).abs() < 0.001);
    }
}
