//! `AVAudioConverter` — sample rate / format conversion.

use apple_objc_sys::*;
use crate::*;

/// Converter input status.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConverterInputStatus {
    HaveData = 0,
    NoDataNow = 1,
    EndOfStream = 2,
}

/// Converter output status.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConverterOutputStatus {
    HaveData = 0,
    InputRanDry = 1,
    EndOfStream = 2,
    Error = 3,
}

/// Prime method for the converter.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConverterPrimeMethod {
    Pre = 0,
    Normal = 1,
    None = 2,
}

/// Audio format converter.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudioconverter>
pub struct AudioConverter { h: Id }

unsafe impl Send for AudioConverter {}

impl AudioConverter {
    /// Create a converter from one format to another.
    pub fn new(from: &AudioFormat, to: &AudioFormat) -> Option<Self> {
        unsafe {
            let obj = msg_send![class!(b"AVAudioConverter\0"), alloc];
            let h = msg_send![obj, initFromFormat: from.as_raw(), toFormat: to.as_raw()];
            if h.is_null() { None } else { Some(Self { h }) }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    /// The input format.
    pub fn input_format(&self) -> AudioFormat {
        AudioFormat::from_raw(unsafe { msg_send_id(self.h, b"inputFormat\0") })
    }

    /// The output format.
    pub fn output_format(&self) -> AudioFormat {
        AudioFormat::from_raw(unsafe { msg_send_id(self.h, b"outputFormat\0") })
    }

    /// Reset the converter.
    pub fn reset(&self) { unsafe { msg_send_void(self.h, b"reset\0"); } }

    /// Convert a buffer (simple, no input block).
    pub fn convert_to_buffer(&self, output: &AudioPCMBuffer, from: &AudioPCMBuffer) -> bool {
        unsafe {
            msg_send_t![bool; self.h, convertToBuffer: output.as_raw(), fromBuffer: from.as_raw(), error: NIL]
        }
    }

    /// Maximum output packet size (useful for compressed formats).
    pub fn maximum_output_packet_size(&self) -> isize {
        unsafe { msg_send_isize(self.h, b"maximumOutputPacketSize\0") }
    }

    /// Sample rate conversion ratio.
    pub fn sample_rate_conversion_ratio(&self) -> f64 {
        unsafe { msg_send_f64(self.h, b"sampleRateConversionRatio\0") }
    }

    /// Prime method.
    pub fn prime_method(&self) -> ConverterPrimeMethod {
        let v = unsafe { msg_send_isize(self.h, b"primeMethod\0") };
        match v {
            0 => ConverterPrimeMethod::Pre,
            1 => ConverterPrimeMethod::Normal,
            _ => ConverterPrimeMethod::None,
        }
    }

    /// Set the prime method.
    pub fn set_prime_method(&self, method: ConverterPrimeMethod) {
        unsafe {
            let sel = sel_registerName(b"setPrimeMethod:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, method as isize);
        }
    }
}

impl Drop for AudioConverter { fn drop(&mut self) { unsafe { release(self.h); } } }
