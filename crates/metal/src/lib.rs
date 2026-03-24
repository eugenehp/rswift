//! Apple Metal — GPU programming from Rust.
//!
//! The Swift bridge (`swift/bridge.swift`) is compiled automatically by
//! `build.rs`.  No separate build step and no runtime `load()` call needed —
//! just add the crate as a dependency and use the API.
//!
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

#![allow(non_snake_case, clippy::missing_transmute_annotations)]

use core::ffi::c_void;

type Handle = *mut c_void;

// ── Extern declarations (resolved at link time by build.rs) ──────────────────

unsafe extern "C" {
    fn mtl_create_system_default_device() -> *mut c_void;
    fn mtl_release(h: Handle);
    fn mtl_device_name(h: Handle, buf: *mut u8, len: usize) -> usize;
    fn mtl_device_has_unified_memory(h: Handle) -> bool;
    fn mtl_device_max_buffer_length(h: Handle) -> usize;
    fn mtl_device_max_threads_per_threadgroup(h: Handle) -> usize;
    fn mtl_device_supports_raytracing(h: Handle) -> bool;
    fn mtl_device_supports_family(h: Handle, family: usize) -> bool;
    fn mtl_device_make_command_queue(h: Handle) -> *mut c_void;
    fn mtl_command_queue_command_buffer(h: Handle) -> *mut c_void;
    fn mtl_command_buffer_commit(h: Handle);
    fn mtl_command_buffer_wait(h: Handle);
    fn mtl_command_buffer_status(h: Handle) -> usize;
    fn mtl_command_buffer_gpu_start_time(h: Handle) -> f64;
    fn mtl_command_buffer_gpu_end_time(h: Handle) -> f64;
    fn mtl_device_make_buffer(h: Handle, len: usize, opts: u64) -> *mut c_void;
    fn mtl_device_make_buffer_with_bytes(h: Handle, ptr: *const c_void, len: usize, opts: u64) -> *mut c_void;
    fn mtl_buffer_contents(h: Handle) -> *mut c_void;
    fn mtl_buffer_length(h: Handle) -> usize;
    fn mtl_device_make_texture(h: Handle, pf: u64, w: usize, ht: usize, mm: usize, sc: usize, al: usize, ty: usize, u: u64, o: u64) -> *mut c_void;
    fn mtl_texture_width(h: Handle) -> usize;
    fn mtl_texture_height(h: Handle) -> usize;
    fn mtl_texture_pixel_format(h: Handle) -> u64;
    fn mtl_texture_replace_region(h: Handle, x: usize, y: usize, w: usize, ht: usize, mip: usize, bpr: usize, ptr: *const c_void);
    fn mtl_device_make_default_library(h: Handle) -> *mut c_void;
    fn mtl_device_make_library_source(h: Handle, src: *const u8, slen: usize, err: *mut u8, elen: usize) -> *mut c_void;
    fn mtl_library_make_function(h: Handle, name: *const u8, nlen: usize) -> *mut c_void;
    fn mtl_library_function_names(h: Handle, buf: *mut u8, len: usize) -> usize;
    fn mtl_device_make_compute_pipeline(h: Handle, fn_h: Handle, err: *mut u8, elen: usize) -> *mut c_void;
    fn mtl_compute_pipeline_max_threads(h: Handle) -> usize;
    fn mtl_compute_pipeline_thread_execution_width(h: Handle) -> usize;
    fn mtl_command_buffer_compute_encoder(h: Handle) -> *mut c_void;
    fn mtl_compute_encoder_set_pipeline(enc: Handle, pip: Handle);
    fn mtl_compute_encoder_set_buffer(enc: Handle, buf: Handle, offset: usize, idx: usize);
    fn mtl_compute_encoder_set_bytes(enc: Handle, ptr: *const c_void, len: usize, idx: usize);
    fn mtl_compute_encoder_set_texture(enc: Handle, tex: Handle, idx: usize);
    fn mtl_compute_encoder_dispatch_threads(enc: Handle, gx: usize, gy: usize, gz: usize, tx: usize, ty: usize, tz: usize);
    fn mtl_compute_encoder_dispatch_threadgroups(enc: Handle, gx: usize, gy: usize, gz: usize, tx: usize, ty: usize, tz: usize);
    fn mtl_compute_encoder_end(h: Handle);
    fn mtl_device_make_render_pipeline(h: Handle, vfn: Handle, ffn: Handle, pf: u64, err: *mut u8, elen: usize) -> *mut c_void;
    fn mtl_command_buffer_blit_encoder(h: Handle) -> *mut c_void;
    fn mtl_blit_encoder_copy_buffer(enc: Handle, src: Handle, soff: usize, dst: Handle, doff: usize, size: usize);
    fn mtl_blit_encoder_fill_buffer(enc: Handle, buf: Handle, off: usize, len: usize, val: u8);
    fn mtl_blit_encoder_end(h: Handle);
}

/// Backward-compatibility no-op — bridge is linked at compile time by build.rs.
pub fn load(_path: &str) {}
/// Backward-compatibility no-op — see `load`.
pub fn auto_load() {}


/// Metal resource storage options.
pub struct ResourceOptions;

impl ResourceOptions {
    pub const STORAGE_SHARED: u64 = 0;
    pub const STORAGE_MANAGED: u64 = 0x10;
    pub const STORAGE_PRIVATE: u64 = 0x20;
    pub const CPU_CACHE_DEFAULT: u64 = 0;
    pub const CPU_CACHE_WRITE_COMBINED: u64 = 0x100;
    pub const HAZARD_TRACKING_DEFAULT: u64 = 0;
    pub const HAZARD_TRACKING_UNTRACKED: u64 = 0x1_0000;
    pub const HAZARD_TRACKING_TRACKED: u64 = 0x2_0000;
}

/// Pixel formats (common subset).
pub struct PixelFormat;

impl PixelFormat {
    pub const BGRA8_UNORM: u64 = 80;
    pub const BGRA8_UNORM_SRGB: u64 = 81;
    pub const RGBA8_UNORM: u64 = 70;
    pub const RGBA8_UNORM_SRGB: u64 = 71;
    pub const RGBA16_FLOAT: u64 = 115;
    pub const RGBA32_FLOAT: u64 = 125;
    pub const R8_UNORM: u64 = 10;
    pub const R16_FLOAT: u64 = 25;
    pub const R32_FLOAT: u64 = 55;
    pub const DEPTH32_FLOAT: u64 = 252;
    pub const DEPTH32_FLOAT_STENCIL8: u64 = 260;
}

/// GPU family identifiers.
pub struct GPUFamily;

impl GPUFamily {
    pub const APPLE1: usize = 1001;
    pub const APPLE2: usize = 1002;
    pub const APPLE3: usize = 1003;
    pub const APPLE4: usize = 1004;
    pub const APPLE5: usize = 1005;
    pub const APPLE6: usize = 1006;
    pub const APPLE7: usize = 1007;
    pub const APPLE8: usize = 1008;
    pub const APPLE9: usize = 1009;
    pub const COMMON1: usize = 3001;
    pub const COMMON2: usize = 3002;
    pub const COMMON3: usize = 3003;
    pub const METAL3: usize = 5001;
}

/// Command buffer execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

impl From<usize> for CommandBufferStatus {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::NotEnqueued,
            1 => Self::Enqueued,
            2 => Self::Committed,
            3 => Self::Scheduled,
            4 => Self::Completed,
            _ => Self::Error,
        }
    }
}

// ── Device ──

/// A Metal GPU device.
pub struct Device {
    handle: Handle,
}

impl Device {
    /// Get the system default Metal device.
    pub fn system_default() -> Option<Self> {
        let h = unsafe { mtl_create_system_default_device() };
        if h.is_null() { None } else { Some(Self { handle: h }) }
    }

    /// GPU name.
    pub fn name(&self) -> String {
        let mut buf = vec![0u8; 256];
        let len = unsafe { mtl_device_name(self.handle, buf.as_mut_ptr(), buf.len()) };
        String::from_utf8_lossy(&buf[..len]).to_string()
    }

    /// Maximum threads per threadgroup (width dimension).
    pub fn max_threads_per_threadgroup(&self) -> usize {
        unsafe { mtl_device_max_threads_per_threadgroup(self.handle) }
    }

    /// Whether the device has unified memory (Apple Silicon).
    pub fn has_unified_memory(&self) -> bool {
        unsafe { mtl_device_has_unified_memory(self.handle) }
    }

    /// Maximum buffer size in bytes.
    pub fn max_buffer_length(&self) -> usize {
        unsafe { mtl_device_max_buffer_length(self.handle) }
    }

    /// Check GPU family support.
    pub fn supports_family(&self, family: usize) -> bool {
        unsafe { mtl_device_supports_family(self.handle, family) }
    }

    /// Whether the device supports ray tracing.
    pub fn supports_raytracing(&self) -> bool {
        unsafe { mtl_device_supports_raytracing(self.handle) }
    }

    /// Create a command queue.
    pub fn command_queue(&self) -> CommandQueue {
        let h = unsafe { mtl_device_make_command_queue(self.handle) };
        assert!(!h.is_null(), "Failed to create command queue");
        CommandQueue { handle: h }
    }

    /// Create a buffer of the given size.
    pub fn buffer(&self, length: usize, options: u64) -> Buffer {
        let h = unsafe { mtl_device_make_buffer(self.handle, length, options) };
        assert!(!h.is_null(), "Failed to create buffer");
        Buffer { handle: h, len: length }
    }

    /// Create a buffer initialized with data.
    pub fn buffer_with_data<T: Copy>(&self, data: &[T], options: u64) -> Buffer {
        let len = data.len() * std::mem::size_of::<T>();
        let h = unsafe {
            mtl_device_make_buffer_with_bytes(self.handle, data.as_ptr() as *const c_void, len, options)
        };
        assert!(!h.is_null(), "Failed to create buffer");
        Buffer { handle: h, len }
    }

    /// Create a 2D texture.
    pub fn texture_2d(&self, width: usize, height: usize, pixel_format: u64, usage: u64) -> Texture {
        let h = unsafe {
            // Swift signature: pf, width, height, mipmaps, sampleCount, arrayLen, type, usage, opts
            mtl_device_make_texture(
                self.handle,
                pixel_format, width, height,
                1,     // mipmapLevelCount
                1,     // sampleCount
                1,     // arrayLength
                2,     // MTLTextureType.type2D
                usage,
                0,     // default resource options
            )
        };
        assert!(!h.is_null(), "Failed to create texture");
        Texture { handle: h }
    }

    /// Load the default Metal library (.metallib bundled with the app).
    pub fn default_library(&self) -> Option<Library> {
        let h = unsafe { mtl_device_make_default_library(self.handle) };
        if h.is_null() { None } else { Some(Library { handle: h }) }
    }

    /// Compile a Metal shader library from source code.
    pub fn library_from_source(&self, source: &str) -> Result<Library, String> {
        let mut err_buf = vec![0u8; 2048];
        let h = unsafe {
            mtl_device_make_library_source(
                self.handle,
                source.as_ptr(),
                source.len(),
                err_buf.as_mut_ptr(),
                err_buf.len(),
            )
        };
        if h.is_null() {
            let err_len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
            Err(String::from_utf8_lossy(&err_buf[..err_len]).to_string())
        } else {
            Ok(Library { handle: h })
        }
    }

    /// Create a compute pipeline from a function.
    pub fn compute_pipeline(&self, function: &Function) -> Result<ComputePipeline, String> {
        let mut err_buf = vec![0u8; 2048];
        let h = unsafe {
            mtl_device_make_compute_pipeline(
                self.handle,
                function.handle,
                err_buf.as_mut_ptr(),
                err_buf.len(),
            )
        };
        if h.is_null() {
            let err_len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
            Err(String::from_utf8_lossy(&err_buf[..err_len]).to_string())
        } else {
            Ok(ComputePipeline { handle: h })
        }
    }

    /// Create a render pipeline.
    pub fn render_pipeline(
        &self,
        vertex: &Function,
        fragment: &Function,
        pixel_format: u64,
    ) -> Result<RenderPipeline, String> {
        let mut err_buf = vec![0u8; 2048];
        let h = unsafe {
            mtl_device_make_render_pipeline(
                self.handle,
                vertex.handle,
                fragment.handle,
                pixel_format,
                err_buf.as_mut_ptr(),
                err_buf.len(),
            )
        };
        if h.is_null() {
            let err_len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
            Err(String::from_utf8_lossy(&err_buf[..err_len]).to_string())
        } else {
            Ok(RenderPipeline { handle: h })
        }
    }

    pub fn raw_handle(&self) -> Handle { self.handle }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { mtl_release(self.handle) }
    }
}

// ── Command Queue ──

pub struct CommandQueue {
    handle: Handle,
}

impl CommandQueue {
    /// Create a command buffer.
    pub fn command_buffer(&self) -> CommandBuffer {
        let h = unsafe { mtl_command_queue_command_buffer(self.handle) };
        assert!(!h.is_null(), "Failed to create command buffer");
        CommandBuffer { handle: h }
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe { mtl_release(self.handle) }
    }
}

// ── Command Buffer ──

pub struct CommandBuffer {
    handle: Handle,
}

impl CommandBuffer {
    /// Commit the command buffer for execution.
    pub fn commit(&self) {
        unsafe { mtl_command_buffer_commit(self.handle) }
    }

    /// Wait until the GPU has finished executing.
    pub fn wait(&self) {
        unsafe { mtl_command_buffer_wait(self.handle) }
    }

    /// Current execution status.
    pub fn status(&self) -> CommandBufferStatus {
        let s = unsafe { mtl_command_buffer_status(self.handle) };
        CommandBufferStatus::from(s)
    }

    /// GPU start time (seconds since boot).
    pub fn gpu_start_time(&self) -> f64 {
        unsafe { mtl_command_buffer_gpu_start_time(self.handle) }
    }

    /// GPU end time (seconds since boot).
    pub fn gpu_end_time(&self) -> f64 {
        unsafe { mtl_command_buffer_gpu_end_time(self.handle) }
    }

    /// GPU execution duration in seconds.
    pub fn gpu_duration(&self) -> f64 {
        self.gpu_end_time() - self.gpu_start_time()
    }

    /// Create a compute command encoder.
    pub fn compute_encoder(&self) -> ComputeEncoder {
        let h = unsafe { mtl_command_buffer_compute_encoder(self.handle) };
        assert!(!h.is_null(), "Failed to create compute encoder");
        ComputeEncoder { handle: h }
    }

    /// Create a blit command encoder.
    pub fn blit_encoder(&self) -> BlitEncoder {
        let h = unsafe { mtl_command_buffer_blit_encoder(self.handle) };
        assert!(!h.is_null(), "Failed to create blit encoder");
        BlitEncoder { handle: h }
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe { mtl_release(self.handle) }
    }
}

// ── Buffer ──

pub struct Buffer {
    handle: Handle,
    len: usize,
}

impl Buffer {
    /// Raw pointer to buffer contents (CPU-accessible for shared/managed).
    pub fn contents(&self) -> *mut c_void {
        unsafe { mtl_buffer_contents(self.handle) }
    }

    /// Buffer length in bytes.
    pub fn length(&self) -> usize {
        unsafe { mtl_buffer_length(self.handle) }
    }

    /// View buffer contents as a typed slice.
    ///
    /// # Safety
    /// The buffer must contain valid data of type `T` and be CPU-accessible.
    pub fn as_slice<T: Copy>(&self) -> &[T] {
        let ptr = self.contents() as *const T;
        let count = self.len / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts(ptr, count) }
    }

    /// View buffer contents as a mutable typed slice.
    ///
    /// # Safety
    /// The buffer must be CPU-accessible and not in use by the GPU.
    pub fn as_mut_slice<T: Copy>(&self) -> &mut [T] {
        let ptr = self.contents() as *mut T;
        let count = self.len / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts_mut(ptr, count) }
    }

    pub fn raw_handle(&self) -> Handle { self.handle }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { mtl_release(self.handle) }
    }
}

// ── Texture ──

pub struct Texture {
    handle: Handle,
}

impl Texture {
    pub fn width(&self) -> usize { unsafe { mtl_texture_width(self.handle) } }
    pub fn height(&self) -> usize { unsafe { mtl_texture_height(self.handle) } }
    pub fn pixel_format(&self) -> u64 { unsafe { mtl_texture_pixel_format(self.handle) } }

    /// Upload pixel data to a region of the texture.
    pub fn replace_region(&self, x: usize, y: usize, w: usize, h: usize, mip: usize, bytes_per_row: usize, data: &[u8]) {
        unsafe {
            mtl_texture_replace_region(self.handle, x, y, w, h, mip, bytes_per_row, data.as_ptr() as *const c_void)
        }
    }

    pub fn raw_handle(&self) -> Handle { self.handle }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { mtl_release(self.handle) }
    }
}

// ── Library ──

pub struct Library {
    handle: Handle,
}

impl Library {
    /// Get a function by name.
    pub fn function(&self, name: &str) -> Option<Function> {
        let h = unsafe { mtl_library_make_function(self.handle, name.as_ptr(), name.len()) };
        if h.is_null() { None } else { Some(Function { handle: h }) }
    }

    /// List all function names in the library.
    pub fn function_names(&self) -> Vec<String> {
        let mut buf = vec![0u8; 8192];
        let len = unsafe { mtl_library_function_names(self.handle, buf.as_mut_ptr(), buf.len()) };
        if len == 0 {
            return vec![];
        }
        String::from_utf8_lossy(&buf[..len])
            .split(',')
            .map(|s| s.to_string())
            .collect()
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { mtl_release(self.handle) }
    }
}

// ── Function ──

pub struct Function {
    handle: Handle,
}

impl Function {
    pub fn raw_handle(&self) -> Handle { self.handle }
}

impl Drop for Function {
    fn drop(&mut self) {
        unsafe { mtl_release(self.handle) }
    }
}

// ── Compute Pipeline ──

pub struct ComputePipeline {
    handle: Handle,
}

impl ComputePipeline {
    /// Maximum number of threads per threadgroup for this pipeline.
    pub fn max_threads(&self) -> usize {
        unsafe { mtl_compute_pipeline_max_threads(self.handle) }
    }

    /// Thread execution width (SIMD width).
    pub fn thread_execution_width(&self) -> usize {
        unsafe { mtl_compute_pipeline_thread_execution_width(self.handle) }
    }
}

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        unsafe { mtl_release(self.handle) }
    }
}

// ── Render Pipeline ──

pub struct RenderPipeline {
    handle: Handle,
}

impl Drop for RenderPipeline {
    fn drop(&mut self) {
        unsafe { mtl_release(self.handle) }
    }
}

// ── Compute Encoder ──

pub struct ComputeEncoder {
    handle: Handle,
}

impl ComputeEncoder {
    /// Set the compute pipeline.
    pub fn set_pipeline(&self, pipeline: &ComputePipeline) {
        unsafe { mtl_compute_encoder_set_pipeline(self.handle, pipeline.handle) }
    }

    /// Bind a buffer at an index.
    pub fn set_buffer(&self, buffer: &Buffer, offset: usize, index: usize) {
        unsafe { mtl_compute_encoder_set_buffer(self.handle, buffer.handle, offset, index) }
    }

    /// Set inline bytes at an index.
    pub fn set_bytes<T: Copy>(&self, data: &[T], index: usize) {
        let len = data.len() * std::mem::size_of::<T>();
        unsafe { mtl_compute_encoder_set_bytes(self.handle, data.as_ptr() as *const c_void, len, index) }
    }

    /// Bind a texture at an index.
    pub fn set_texture(&self, texture: &Texture, index: usize) {
        unsafe { mtl_compute_encoder_set_texture(self.handle, texture.handle, index) }
    }

    /// Dispatch compute threads (non-uniform).
    pub fn dispatch_threads(
        &self,
        grid_x: usize, grid_y: usize, grid_z: usize,
        group_x: usize, group_y: usize, group_z: usize,
    ) {
        unsafe {
            mtl_compute_encoder_dispatch_threads(self.handle, grid_x, grid_y, grid_z, group_x, group_y, group_z)
        }
    }

    /// Dispatch compute threadgroups.
    pub fn dispatch_threadgroups(
        &self,
        groups_x: usize, groups_y: usize, groups_z: usize,
        threads_x: usize, threads_y: usize, threads_z: usize,
    ) {
        unsafe {
            mtl_compute_encoder_dispatch_threadgroups(self.handle, groups_x, groups_y, groups_z, threads_x, threads_y, threads_z)
        }
    }

    /// End encoding.
    pub fn end(&self) {
        unsafe { mtl_compute_encoder_end(self.handle) }
    }
}

// ── Blit Encoder ──

pub struct BlitEncoder {
    handle: Handle,
}

impl BlitEncoder {
    /// Copy between buffers.
    pub fn copy_buffer(
        &self,
        src: &Buffer, src_offset: usize,
        dst: &Buffer, dst_offset: usize,
        size: usize,
    ) {
        unsafe {
            mtl_blit_encoder_copy_buffer(self.handle, src.handle, src_offset, dst.handle, dst_offset, size)
        }
    }

    /// Fill a buffer region with a byte value.
    pub fn fill_buffer(&self, buffer: &Buffer, offset: usize, size: usize, value: u8) {
        unsafe { mtl_blit_encoder_fill_buffer(self.handle, buffer.handle, offset, size, value) }
    }

    /// End encoding.
    pub fn end(&self) {
        unsafe { mtl_blit_encoder_end(self.handle) }
    }
}

// Note: Metal is not available on watchOS. All functions will panic at runtime
// if the bridge is loaded on watchOS (which shouldn't happen).
