//! `AVAudioPCMBuffer` and `AVAudioCompressedBuffer`.

use apple_objc_sys::*;
use crate::*;

/// A buffer of PCM audio data.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiopcmbuffer>
pub struct AudioPCMBuffer { h: Id }

unsafe impl Send for AudioPCMBuffer {}

impl AudioPCMBuffer {
    /// Create a new PCM buffer with the given format and capacity.
    pub fn new(format: &AudioFormat, frame_capacity: AudioFrameCount) -> Option<Self> {
        unsafe {
            let obj = msg_send![class!(b"AVAudioPCMBuffer\0"), alloc];
            let sel = sel_registerName(b"initWithPCMFormat:frameCapacity:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, u32) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let h = f(obj, sel, format.as_raw(), frame_capacity);
            if h.is_null() { None } else { Some(Self { h }) }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    /// Wrap an existing `AVAudioPCMBuffer*` (non-owning).
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }

    // ── Properties ──────────────────────────────────────────────────────

    /// The buffer's format.
    pub fn format(&self) -> AudioFormat {
        AudioFormat::from_raw(unsafe { msg_send_id(self.h, b"format\0") })
    }

    /// The buffer's capacity in frames.
    pub fn frame_capacity(&self) -> AudioFrameCount {
        unsafe { msg_send_usize(self.h, b"frameCapacity\0") as AudioFrameCount }
    }

    /// The current number of valid frames.
    pub fn frame_length(&self) -> AudioFrameCount {
        unsafe { msg_send_usize(self.h, b"frameLength\0") as AudioFrameCount }
    }

    /// Set the number of valid frames.
    pub fn set_frame_length(&self, len: AudioFrameCount) {
        unsafe {
            let sel = sel_registerName(b"setFrameLength:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, len);
        }
    }

    /// Stride (number of interleaved channels).
    pub fn stride(&self) -> usize { unsafe { msg_send_usize(self.h, b"stride\0") } }

    /// Pointer to float channel data (nil if not float format).
    /// Returns the raw `float * const *` pointer.
    pub fn float_channel_data(&self) -> *const *mut f32 {
        unsafe { msg_send_id(self.h, b"floatChannelData\0") as *const *mut f32 }
    }

    /// Pointer to int16 channel data.
    pub fn int16_channel_data(&self) -> *const *mut i16 {
        unsafe { msg_send_id(self.h, b"int16ChannelData\0") as *const *mut i16 }
    }

    /// Pointer to int32 channel data.
    pub fn int32_channel_data(&self) -> *const *mut i32 {
        unsafe { msg_send_id(self.h, b"int32ChannelData\0") as *const *mut i32 }
    }
}

impl Drop for AudioPCMBuffer { fn drop(&mut self) { unsafe { release(self.h); } } }

// ── AVAudioCompressedBuffer ─────────────────────────────────────────────────

/// A buffer of compressed audio data.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiocompressedbuffer>
pub struct AudioCompressedBuffer { h: Id }

unsafe impl Send for AudioCompressedBuffer {}

impl AudioCompressedBuffer {
    /// Create with format, packet capacity, and maximum packet size.
    pub fn new(format: &AudioFormat, packet_capacity: AudioPacketCount, max_packet_size: isize) -> Self {
        unsafe {
            let obj = msg_send![class!(b"AVAudioCompressedBuffer\0"), alloc];
            let sel = sel_registerName(b"initWithFormat:packetCapacity:maximumPacketSize:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, u32, isize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let h = f(obj, sel, format.as_raw(), packet_capacity, max_packet_size);
            Self { h }
        }
    }

    /// Create with format and packet capacity (constant bytes per packet).
    pub fn with_packet_capacity(format: &AudioFormat, packet_capacity: AudioPacketCount) -> Self {
        unsafe {
            let obj = msg_send![class!(b"AVAudioCompressedBuffer\0"), alloc];
            let sel = sel_registerName(b"initWithFormat:packetCapacity:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, u32) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let h = f(obj, sel, format.as_raw(), packet_capacity);
            Self { h }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    pub fn format(&self) -> AudioFormat {
        AudioFormat::from_raw(unsafe { msg_send_id(self.h, b"format\0") })
    }

    pub fn packet_capacity(&self) -> AudioPacketCount {
        unsafe { msg_send_usize(self.h, b"packetCapacity\0") as AudioPacketCount }
    }

    pub fn packet_count(&self) -> AudioPacketCount {
        unsafe { msg_send_usize(self.h, b"packetCount\0") as AudioPacketCount }
    }

    pub fn set_packet_count(&self, count: AudioPacketCount) {
        unsafe {
            let sel = sel_registerName(b"setPacketCount:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, count);
        }
    }

    pub fn maximum_packet_size(&self) -> isize { unsafe { msg_send_isize(self.h, b"maximumPacketSize\0") } }

    pub fn byte_capacity(&self) -> u32 {
        unsafe { msg_send_usize(self.h, b"byteCapacity\0") as u32 }
    }

    pub fn byte_length(&self) -> u32 {
        unsafe { msg_send_usize(self.h, b"byteLength\0") as u32 }
    }

    pub fn set_byte_length(&self, len: u32) {
        unsafe {
            let sel = sel_registerName(b"setByteLength:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, len);
        }
    }

    /// Raw pointer to compressed data.
    pub fn data(&self) -> *mut core::ffi::c_void {
        unsafe { msg_send_id(self.h, b"data\0") as *mut core::ffi::c_void }
    }
}

impl Drop for AudioCompressedBuffer { fn drop(&mut self) { unsafe { release(self.h); } } }
