//! Apple CoreMedia — CMTime and media timing from Rust.
//!
//! **Platform support:** macOS 10.7+, iOS 4+, tvOS 9+, visionOS 1+.
//!
//! Links CoreMedia directly — no Swift bridge needed.
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

/// CoreMedia is always available on Apple platforms.
/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


// ── Raw CoreMedia C symbols ─────────────────────────────────────────────────

/// Raw CMTime as the framework defines it.
#[repr(C)]
#[derive(Clone, Copy)]
struct RawCMTime {
    value: i64,
    timescale: i32,
    flags: u32,
    epoch: i64,
}

const K_CMTIME_VALID: u32 = 1;
const K_CMTIME_INDEFINITE: u32 = 16;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn CMTimeMake(value: i64, timescale: i32) -> RawCMTime;
    fn CMTimeMakeWithSeconds(seconds: f64, preferredTimescale: i32) -> RawCMTime;
    fn CMTimeGetSeconds(time: RawCMTime) -> f64;
    fn CMTimeAdd(lhs: RawCMTime, rhs: RawCMTime) -> RawCMTime;
    fn CMTimeSubtract(lhs: RawCMTime, rhs: RawCMTime) -> RawCMTime;
    fn CMTimeCompare(time1: RawCMTime, time2: RawCMTime) -> i32;
}

/// A time value with rational timescale (wraps `CMTime`).
#[derive(Debug, Clone, Copy)]
pub struct CMTime {
    pub value: i64,
    pub timescale: i32,
    flags: u32,
    epoch: i64,
}

impl CMTime {
    fn from_raw(r: RawCMTime) -> Self {
        Self { value: r.value, timescale: r.timescale, flags: r.flags, epoch: r.epoch }
    }
    fn to_raw(&self) -> RawCMTime {
        RawCMTime { value: self.value, timescale: self.timescale, flags: self.flags, epoch: self.epoch }
    }

    /// Create a time from a value and timescale.
    pub fn new(value: i64, timescale: i32) -> Self {
        Self::from_raw(unsafe { CMTimeMake(value, timescale) })
    }

    /// Create a time from seconds with a preferred timescale.
    pub fn from_seconds(seconds: f64) -> Self {
        Self::from_seconds_with_timescale(seconds, 600)
    }

    /// Create a time from seconds with a specific timescale.
    pub fn from_seconds_with_timescale(seconds: f64, timescale: i32) -> Self {
        Self::from_raw(unsafe { CMTimeMakeWithSeconds(seconds, timescale) })
    }

    /// Convert to seconds.
    pub fn seconds(&self) -> f64 {
        unsafe { CMTimeGetSeconds(self.to_raw()) }
    }

    /// Add two times.
    pub fn add(&self, other: &CMTime) -> CMTime {
        Self::from_raw(unsafe { CMTimeAdd(self.to_raw(), other.to_raw()) })
    }

    /// Subtract another time.
    pub fn subtract(&self, other: &CMTime) -> CMTime {
        Self::from_raw(unsafe { CMTimeSubtract(self.to_raw(), other.to_raw()) })
    }

    /// Whether this time is valid.
    pub fn is_valid(&self) -> bool {
        self.flags & K_CMTIME_VALID != 0
    }

    /// Whether this time is indefinite (e.g. live stream).
    pub fn is_indefinite(&self) -> bool {
        self.flags & K_CMTIME_INDEFINITE != 0
    }
}

impl PartialEq for CMTime {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CMTimeCompare(self.to_raw(), other.to_raw()) == 0 }
    }
}

impl PartialOrd for CMTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let c = unsafe { CMTimeCompare(self.to_raw(), other.to_raw()) };
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
