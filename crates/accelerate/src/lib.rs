//! Apple Accelerate — vDSP vector math and BLAS from Rust.
//!
//! **Platform support:** all Apple platforms.
//!
//! Hardware-accelerated vector operations, FFT, and matrix multiplication.
//! Links the Accelerate framework directly — no Swift bridge needed.
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

/// Accelerate is always available on Apple platforms.
pub fn is_available() -> bool { true }

// ── Raw Accelerate C symbols ────────────────────────────────────────────────
// These are the real framework symbols — no wrapper dylib needed.

type VDSPLength = u64; // vDSP_Length is unsigned long on Apple

// Stride type is signed long on Apple (vDSP_Stride)
type VDSPStride = isize;

#[allow(non_snake_case)]
unsafe extern "C" {
    // vDSP f32
    fn vDSP_vadd(a: *const f32, ia: VDSPStride, b: *const f32, ib: VDSPStride,
                 c: *mut f32, ic: VDSPStride, n: VDSPLength);
    fn vDSP_vsub(a: *const f32, ia: VDSPStride, b: *const f32, ib: VDSPStride,
                 c: *mut f32, ic: VDSPStride, n: VDSPLength);
    fn vDSP_vmul(a: *const f32, ia: VDSPStride, b: *const f32, ib: VDSPStride,
                 c: *mut f32, ic: VDSPStride, n: VDSPLength);
    fn vDSP_vdiv(a: *const f32, ia: VDSPStride, b: *const f32, ib: VDSPStride,
                 c: *mut f32, ic: VDSPStride, n: VDSPLength);
    fn vDSP_vsmul(a: *const f32, ia: VDSPStride, b: *const f32,
                  c: *mut f32, ic: VDSPStride, n: VDSPLength);
    fn vDSP_dotpr(a: *const f32, ia: VDSPStride, b: *const f32, ib: VDSPStride,
                  c: *mut f32, n: VDSPLength);
    fn vDSP_sve(a: *const f32, ia: VDSPStride, c: *mut f32, n: VDSPLength);
    fn vDSP_meanv(a: *const f32, ia: VDSPStride, c: *mut f32, n: VDSPLength);
    fn vDSP_maxv(a: *const f32, ia: VDSPStride, c: *mut f32, n: VDSPLength);
    fn vDSP_minv(a: *const f32, ia: VDSPStride, c: *mut f32, n: VDSPLength);
    fn vDSP_rmsqv(a: *const f32, ia: VDSPStride, c: *mut f32, n: VDSPLength);
    fn vDSP_normalize(a: *const f32, ia: VDSPStride, c: *mut f32, ic: VDSPStride,
                      mean: *mut f32, stddev: *mut f32, n: VDSPLength);

    // vDSP f64
    fn vDSP_vaddD(a: *const f64, ia: VDSPStride, b: *const f64, ib: VDSPStride,
                  c: *mut f64, ic: VDSPStride, n: VDSPLength);
    fn vDSP_vmulD(a: *const f64, ia: VDSPStride, b: *const f64, ib: VDSPStride,
                  c: *mut f64, ic: VDSPStride, n: VDSPLength);
    fn vDSP_dotprD(a: *const f64, ia: VDSPStride, b: *const f64, ib: VDSPStride,
                   c: *mut f64, n: VDSPLength);
    fn vDSP_sveD(a: *const f64, ia: VDSPStride, c: *mut f64, n: VDSPLength);

    // FFT
    fn vDSP_create_fftsetup(log2n: VDSPLength, radix: i32) -> *mut core::ffi::c_void;
    fn vDSP_destroy_fftsetup(setup: *mut core::ffi::c_void);
    fn vDSP_ctoz(input: *const [f32; 2], input_stride: VDSPStride,
                 output: *mut DSPSplitComplex, output_stride: VDSPStride, n: VDSPLength);
    fn vDSP_fft_zrip(setup: *mut core::ffi::c_void, c: *mut DSPSplitComplex,
                     ic: VDSPStride, log2n: VDSPLength, direction: i32);

    // BLAS
    fn cblas_sgemm(order: i32, transa: i32, transb: i32,
                   m: i32, n: i32, k: i32, alpha: f32,
                   a: *const f32, lda: i32, b: *const f32, ldb: i32,
                   beta: f32, c: *mut f32, ldc: i32);
    fn cblas_dgemm(order: i32, transa: i32, transb: i32,
                   m: i32, n: i32, k: i32, alpha: f64,
                   a: *const f64, lda: i32, b: *const f64, ldb: i32,
                   beta: f64, c: *mut f64, ldc: i32);
}

#[repr(C)]
struct DSPSplitComplex {
    realp: *mut f32,
    imagp: *mut f32,
}

// FFT constants
const FFT_RADIX2: i32 = 0;
const FFT_FORWARD: i32 = 1;
const FFT_INVERSE: i32 = -1;

// BLAS constants
const CBLAS_ROW_MAJOR: i32 = 101;
const CBLAS_NO_TRANS: i32 = 111;

/// vDSP — hardware-accelerated vector operations.
pub mod vdsp {
    use super::*;

    // ── f32 operations ──

    /// Element-wise add: `out[i] = a[i] + b[i]`.
    pub fn add_f32(a: &[f32], b: &[f32]) -> Vec<f32> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f32; n];
        unsafe { vDSP_vadd(a.as_ptr(), 1, b.as_ptr(), 1, out.as_mut_ptr(), 1, n as VDSPLength) }
        out
    }

    /// Element-wise subtract: `out[i] = a[i] - b[i]`.
    pub fn sub_f32(a: &[f32], b: &[f32]) -> Vec<f32> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f32; n];
        // vDSP_vsub: c = b - a (args swapped)
        unsafe { vDSP_vsub(b.as_ptr(), 1, a.as_ptr(), 1, out.as_mut_ptr(), 1, n as VDSPLength) }
        out
    }

    /// Element-wise multiply: `out[i] = a[i] * b[i]`.
    pub fn mul_f32(a: &[f32], b: &[f32]) -> Vec<f32> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f32; n];
        unsafe { vDSP_vmul(a.as_ptr(), 1, b.as_ptr(), 1, out.as_mut_ptr(), 1, n as VDSPLength) }
        out
    }

    /// Element-wise divide: `out[i] = a[i] / b[i]`.
    pub fn div_f32(a: &[f32], b: &[f32]) -> Vec<f32> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f32; n];
        // vDSP_vdiv: c = b / a (args swapped)
        unsafe { vDSP_vdiv(b.as_ptr(), 1, a.as_ptr(), 1, out.as_mut_ptr(), 1, n as VDSPLength) }
        out
    }

    /// Scalar multiply: `out[i] = a[i] * scalar`.
    pub fn scale_f32(a: &[f32], scalar: f32) -> Vec<f32> {
        let mut out = vec![0.0f32; a.len()];
        unsafe { vDSP_vsmul(a.as_ptr(), 1, &scalar, out.as_mut_ptr(), 1, a.len() as VDSPLength) }
        out
    }

    /// Dot product.
    pub fn dot_f32(a: &[f32], b: &[f32]) -> f32 {
        let mut result = 0.0f32;
        let n = a.len().min(b.len());
        unsafe { vDSP_dotpr(a.as_ptr(), 1, b.as_ptr(), 1, &mut result, n as VDSPLength) }
        result
    }

    /// Sum of all elements.
    pub fn sum_f32(a: &[f32]) -> f32 {
        let mut result = 0.0f32;
        unsafe { vDSP_sve(a.as_ptr(), 1, &mut result, a.len() as VDSPLength) }
        result
    }

    /// Mean of all elements.
    pub fn mean_f32(a: &[f32]) -> f32 {
        let mut result = 0.0f32;
        unsafe { vDSP_meanv(a.as_ptr(), 1, &mut result, a.len() as VDSPLength) }
        result
    }

    /// Maximum element.
    pub fn max_f32(a: &[f32]) -> f32 {
        let mut result = 0.0f32;
        unsafe { vDSP_maxv(a.as_ptr(), 1, &mut result, a.len() as VDSPLength) }
        result
    }

    /// Minimum element.
    pub fn min_f32(a: &[f32]) -> f32 {
        let mut result = 0.0f32;
        unsafe { vDSP_minv(a.as_ptr(), 1, &mut result, a.len() as VDSPLength) }
        result
    }

    /// Root mean square.
    pub fn rms_f32(a: &[f32]) -> f32 {
        let mut result = 0.0f32;
        unsafe { vDSP_rmsqv(a.as_ptr(), 1, &mut result, a.len() as VDSPLength) }
        result
    }

    /// Normalize: subtract mean, divide by standard deviation.
    /// Returns `(normalized, mean, stddev)`.
    pub fn normalize_f32(a: &[f32]) -> (Vec<f32>, f32, f32) {
        let mut out = vec![0.0f32; a.len()];
        let mut mean = 0.0f32;
        let mut std = 0.0f32;
        unsafe { vDSP_normalize(a.as_ptr(), 1, out.as_mut_ptr(), 1, &mut mean, &mut std, a.len() as VDSPLength) }
        (out, mean, std)
    }

    // ── f64 operations ──

    /// Element-wise add (f64).
    pub fn add_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f64; n];
        unsafe { vDSP_vaddD(a.as_ptr(), 1, b.as_ptr(), 1, out.as_mut_ptr(), 1, n as VDSPLength) }
        out
    }

    /// Element-wise multiply (f64).
    pub fn mul_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        let n = a.len().min(b.len());
        let mut out = vec![0.0f64; n];
        unsafe { vDSP_vmulD(a.as_ptr(), 1, b.as_ptr(), 1, out.as_mut_ptr(), 1, n as VDSPLength) }
        out
    }

    /// Dot product (f64).
    pub fn dot_f64(a: &[f64], b: &[f64]) -> f64 {
        let mut result = 0.0f64;
        let n = a.len().min(b.len());
        unsafe { vDSP_dotprD(a.as_ptr(), 1, b.as_ptr(), 1, &mut result, n as VDSPLength) }
        result
    }

    /// Sum (f64).
    pub fn sum_f64(a: &[f64]) -> f64 {
        let mut result = 0.0f64;
        unsafe { vDSP_sveD(a.as_ptr(), 1, &mut result, a.len() as VDSPLength) }
        result
    }

    // ── FFT ──

    /// Real-to-complex FFT. Input length must be a power of 2.
    /// Returns `(real_part, imaginary_part)` of length `n/2`.
    pub fn fft_f32(input: &[f32]) -> (Vec<f32>, Vec<f32>) {
        let n = input.len();
        let log2n = (n as f64).log2() as u64;
        let half = n / 2;
        let mut real = vec![0.0f32; half];
        let mut imag = vec![0.0f32; half];
        unsafe {
            let setup = vDSP_create_fftsetup(log2n as VDSPLength, FFT_RADIX2);
            if setup.is_null() { return (real, imag); }
            let mut split = DSPSplitComplex { realp: real.as_mut_ptr(), imagp: imag.as_mut_ptr() };
            vDSP_ctoz(input.as_ptr() as *const [f32; 2], 2, &mut split, 1, half as VDSPLength);
            vDSP_fft_zrip(setup, &mut split, 1, log2n as VDSPLength, FFT_FORWARD);
            vDSP_destroy_fftsetup(setup);
        }
        (real, imag)
    }

    /// Inverse FFT. Input `real` and `imag` must be length `n/2`.
    /// Returns time-domain signal of length `n`.
    pub fn ifft_f32(real: &[f32], imag: &[f32]) -> Vec<f32> {
        let half = real.len();
        let n = half * 2;
        let log2n = (n as f64).log2() as u64;
        let mut input = vec![0.0f32; n];
        for i in 0..half {
            input[i * 2] = real[i];
            input[i * 2 + 1] = imag[i];
        }
        let mut r = vec![0.0f32; half];
        let mut im = vec![0.0f32; half];
        unsafe {
            let setup = vDSP_create_fftsetup(log2n as VDSPLength, FFT_RADIX2);
            if setup.is_null() { return vec![0.0; n]; }
            let mut split = DSPSplitComplex { realp: r.as_mut_ptr(), imagp: im.as_mut_ptr() };
            vDSP_ctoz(input.as_ptr() as *const [f32; 2], 2, &mut split, 1, half as VDSPLength);
            vDSP_fft_zrip(setup, &mut split, 1, log2n as VDSPLength, FFT_INVERSE);
            vDSP_destroy_fftsetup(setup);
        }
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
            cblas_sgemm(CBLAS_ROW_MAJOR, CBLAS_NO_TRANS, CBLAS_NO_TRANS,
                        m as i32, n as i32, k as i32, 1.0,
                        a.as_ptr(), k as i32, b.as_ptr(), n as i32,
                        0.0, out.as_mut_ptr(), n as i32)
        }
        out
    }

    /// Matrix multiply C = A × B (f64, row-major).
    pub fn dgemm(a: &[f64], b: &[f64], m: usize, n: usize, k: usize) -> Vec<f64> {
        let mut out = vec![0.0f64; m * n];
        unsafe {
            cblas_dgemm(CBLAS_ROW_MAJOR, CBLAS_NO_TRANS, CBLAS_NO_TRANS,
                        m as i32, n as i32, k as i32, 1.0,
                        a.as_ptr(), k as i32, b.as_ptr(), n as i32,
                        0.0, out.as_mut_ptr(), n as i32)
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(vdsp::add_f32(&[1.0, 2.0], &[3.0, 4.0]), vec![4.0, 6.0]);
    }

    #[test]
    fn test_sub() {
        assert_eq!(vdsp::sub_f32(&[5.0, 3.0], &[1.0, 1.0]), vec![4.0, 2.0]);
    }

    #[test]
    fn test_mul() {
        assert_eq!(vdsp::mul_f32(&[2.0, 3.0], &[4.0, 5.0]), vec![8.0, 15.0]);
    }

    #[test]
    fn test_scale() {
        assert_eq!(vdsp::scale_f32(&[1.0, 2.0, 3.0], 10.0), vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_dot() {
        assert_eq!(vdsp::dot_f32(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]), 32.0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(vdsp::sum_f32(&[1.0, 2.0, 3.0, 4.0]), 10.0);
    }

    #[test]
    fn test_mean() {
        assert_eq!(vdsp::mean_f32(&[2.0, 4.0, 6.0, 8.0]), 5.0);
    }

    #[test]
    fn test_max_min() {
        assert_eq!(vdsp::max_f32(&[3.0, 1.0, 4.0, 1.0, 5.0]), 5.0);
        assert_eq!(vdsp::min_f32(&[3.0, 1.0, 4.0, 1.0, 5.0]), 1.0);
    }

    #[test]
    fn test_f64_ops() {
        assert_eq!(vdsp::add_f64(&[1.0, 2.0], &[3.0, 4.0]), vec![4.0, 6.0]);
        assert_eq!(vdsp::dot_f64(&[1.0, 2.0], &[3.0, 4.0]), 11.0);
        assert_eq!(vdsp::sum_f64(&[1.0, 2.0, 3.0]), 6.0);
    }

    #[test]
    fn test_sgemm_identity() {
        let id = vec![1.0f32, 0.0, 0.0, 1.0];
        let a = vec![1.0f32, 2.0, 3.0, 4.0];
        let c = blas::sgemm(&id, &a, 2, 2, 2);
        assert_eq!(c, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_sgemm_2x2() {
        let a = vec![1.0f32, 2.0, 3.0, 4.0];
        let b = vec![5.0f32, 6.0, 7.0, 8.0];
        let c = blas::sgemm(&a, &b, 2, 2, 2);
        assert_eq!(c, vec![19.0, 22.0, 43.0, 50.0]);
    }
}
