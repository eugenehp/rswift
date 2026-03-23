//! Apple Accelerate — vDSP vector math and BLAS from Rust.
//!
//! **Platform support:** all Apple platforms.
//!
//! Hardware-accelerated vector operations, FFT, and matrix multiplication.
//!
//! # Quick start
//!
//! ```ignore
//! let a = vec![1.0f32, 2.0, 3.0, 4.0];
//! let b = vec![5.0f32, 6.0, 7.0, 8.0];
//!
//! // Vector add
//! let sum = accelerate::vdsp::add_f32(&a, &b);
//! assert_eq!(sum, vec![6.0, 8.0, 10.0, 12.0]);
//!
//! // Dot product
//! let dot = accelerate::vdsp::dot_f32(&a, &b);
//! assert_eq!(dot, 70.0);
//!
//! // Matrix multiply (2×2 × 2×2)
//! let c = accelerate::blas::sgemm(&a, &b, 2, 2, 2);
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"accelerate_available");

unsafe extern "C" {
    fn accelerate_vdsp_add_f32(a: *const f32, b: *const f32, o: *mut f32, n: usize);
    fn accelerate_vdsp_sub_f32(a: *const f32, b: *const f32, o: *mut f32, n: usize);
    fn accelerate_vdsp_mul_f32(a: *const f32, b: *const f32, o: *mut f32, n: usize);
    fn accelerate_vdsp_div_f32(a: *const f32, b: *const f32, o: *mut f32, n: usize);
    fn accelerate_vdsp_scale_f32(a: *const f32, s: f32, o: *mut f32, n: usize);
    fn accelerate_vdsp_dot_f32(a: *const f32, b: *const f32, n: usize) -> f32;
    fn accelerate_vdsp_sum_f32(a: *const f32, n: usize) -> f32;
    fn accelerate_vdsp_mean_f32(a: *const f32, n: usize) -> f32;
    fn accelerate_vdsp_max_f32(a: *const f32, n: usize) -> f32;
    fn accelerate_vdsp_min_f32(a: *const f32, n: usize) -> f32;
    fn accelerate_vdsp_rms_f32(a: *const f32, n: usize) -> f32;
    fn accelerate_vdsp_normalize_f32(a: *const f32, o: *mut f32, n: usize, mean: *mut f32, std: *mut f32);
    fn accelerate_vdsp_add_f64(a: *const f64, b: *const f64, o: *mut f64, n: usize);
    fn accelerate_vdsp_mul_f64(a: *const f64, b: *const f64, o: *mut f64, n: usize);
    fn accelerate_vdsp_dot_f64(a: *const f64, b: *const f64, n: usize) -> f64;
    fn accelerate_vdsp_sum_f64(a: *const f64, n: usize) -> f64;
    fn accelerate_vdsp_fft_f32(input: *const f32, real: *mut f32, imag: *mut f32, log2n: usize, fwd: bool);
    fn accelerate_blas_sgemm(a: *const f32, b: *const f32, o: *mut f32, m: i32, n: i32, k: i32, alpha: f32, beta: f32);
    fn accelerate_blas_dgemm(a: *const f64, b: *const f64, o: *mut f64, m: i32, n: i32, k: i32, alpha: f64, beta: f64);
}

/// vDSP — hardware-accelerated vector operations.
pub mod vdsp {
    use super::*;

    // ── f32 operations ──

    /// Element-wise add: `out[i] = a[i] + b[i]`.
    pub fn add_f32(a: &[f32], b: &[f32]) -> Vec<f32> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f32; n];
        unsafe { accelerate_vdsp_add_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), n) }
        out
    }

    /// Element-wise subtract: `out[i] = a[i] - b[i]`.
    pub fn sub_f32(a: &[f32], b: &[f32]) -> Vec<f32> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f32; n];
        unsafe { accelerate_vdsp_sub_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), n) }
        out
    }

    /// Element-wise multiply: `out[i] = a[i] * b[i]`.
    pub fn mul_f32(a: &[f32], b: &[f32]) -> Vec<f32> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f32; n];
        unsafe { accelerate_vdsp_mul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), n) }
        out
    }

    /// Element-wise divide: `out[i] = a[i] / b[i]`.
    pub fn div_f32(a: &[f32], b: &[f32]) -> Vec<f32> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f32; n];
        unsafe { accelerate_vdsp_div_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), n) }
        out
    }

    /// Scalar multiply: `out[i] = a[i] * scalar`.
    pub fn scale_f32(a: &[f32], scalar: f32) -> Vec<f32> {
        let mut out = vec![0.0f32; a.len()];
        unsafe { accelerate_vdsp_scale_f32(a.as_ptr(), scalar, out.as_mut_ptr(), a.len()) }
        out
    }

    /// Dot product.
    pub fn dot_f32(a: &[f32], b: &[f32]) -> f32 {
        unsafe { accelerate_vdsp_dot_f32(a.as_ptr(), b.as_ptr(), a.len().min(b.len())) }
    }

    /// Sum of all elements.
    pub fn sum_f32(a: &[f32]) -> f32 {
        unsafe { accelerate_vdsp_sum_f32(a.as_ptr(), a.len()) }
    }

    /// Mean of all elements.
    pub fn mean_f32(a: &[f32]) -> f32 {
        unsafe { accelerate_vdsp_mean_f32(a.as_ptr(), a.len()) }
    }

    /// Maximum element.
    pub fn max_f32(a: &[f32]) -> f32 {
        unsafe { accelerate_vdsp_max_f32(a.as_ptr(), a.len()) }
    }

    /// Minimum element.
    pub fn min_f32(a: &[f32]) -> f32 {
        unsafe { accelerate_vdsp_min_f32(a.as_ptr(), a.len()) }
    }

    /// Root mean square.
    pub fn rms_f32(a: &[f32]) -> f32 {
        unsafe { accelerate_vdsp_rms_f32(a.as_ptr(), a.len()) }
    }

    /// Normalize: subtract mean, divide by standard deviation.
    /// Returns `(normalized, mean, stddev)`.
    pub fn normalize_f32(a: &[f32]) -> (Vec<f32>, f32, f32) {
        let mut out = vec![0.0f32; a.len()];
        let mut mean = 0.0f32;
        let mut std = 0.0f32;
        unsafe { accelerate_vdsp_normalize_f32(a.as_ptr(), out.as_mut_ptr(), a.len(), &mut mean, &mut std) }
        (out, mean, std)
    }

    // ── f64 operations ──

    /// Element-wise add (f64).
    pub fn add_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f64; n];
        unsafe { accelerate_vdsp_add_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), n) }
        out
    }

    /// Element-wise multiply (f64).
    pub fn mul_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f64; n];
        unsafe { accelerate_vdsp_mul_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), n) }
        out
    }

    /// Dot product (f64).
    pub fn dot_f64(a: &[f64], b: &[f64]) -> f64 {
        unsafe { accelerate_vdsp_dot_f64(a.as_ptr(), b.as_ptr(), a.len().min(b.len())) }
    }

    /// Sum (f64).
    pub fn sum_f64(a: &[f64]) -> f64 {
        unsafe { accelerate_vdsp_sum_f64(a.as_ptr(), a.len()) }
    }

    // ── FFT ──

    /// Real-to-complex FFT. Input length must be a power of 2.
    /// Returns `(real_part, imaginary_part)` of length `n/2`.
    pub fn fft_f32(input: &[f32]) -> (Vec<f32>, Vec<f32>) {
        let n = input.len();
        let log2n = (n as f64).log2() as usize;
        let half = n / 2;
        let mut real = vec![0.0f32; half];
        let mut imag = vec![0.0f32; half];
        unsafe { accelerate_vdsp_fft_f32(input.as_ptr(), real.as_mut_ptr(), imag.as_mut_ptr(), log2n, true) }
        (real, imag)
    }

    /// Inverse FFT. Input `real` and `imag` must be length `n/2`.
    /// Returns time-domain signal of length `n`.
    pub fn ifft_f32(real: &[f32], imag: &[f32]) -> Vec<f32> {
        let half = real.len();
        let n = half * 2;
        let log2n = (n as f64).log2() as usize;
        // Interleave for inverse
        let mut input = vec![0.0f32; n];
        for i in 0..half {
            input[i * 2] = real[i];
            input[i * 2 + 1] = imag[i];
        }
        let mut r = vec![0.0f32; half];
        let mut im = vec![0.0f32; half];
        unsafe { accelerate_vdsp_fft_f32(input.as_ptr(), r.as_mut_ptr(), im.as_mut_ptr(), log2n, false) }
        // Reconstruct
        let mut out = vec![0.0f32; n];
        for i in 0..half {
            out[i * 2] = r[i];
            out[i * 2 + 1] = im[i];
        }
        out
    }
}

/// BLAS — matrix multiplication.
pub mod blas {
    use super::*;

    /// Matrix multiply C = A × B (f32, row-major).
    ///
    /// A is `m × k`, B is `k × n`, result is `m × n`.
    pub fn sgemm(a: &[f32], b: &[f32], m: usize, n: usize, k: usize) -> Vec<f32> {
        let mut out = vec![0.0f32; m * n];
        unsafe {
            accelerate_blas_sgemm(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(),
                                   m as i32, n as i32, k as i32, 1.0, 0.0)
        }
        out
    }

    /// Matrix multiply C = A × B (f64, row-major).
    pub fn dgemm(a: &[f64], b: &[f64], m: usize, n: usize, k: usize) -> Vec<f64> {
        let mut out = vec![0.0f64; m * n];
        unsafe {
            accelerate_blas_dgemm(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(),
                                   m as i32, n as i32, k as i32, 1.0, 0.0)
        }
        out
    }
}
