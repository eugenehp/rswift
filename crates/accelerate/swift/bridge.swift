import Foundation
import Accelerate

// ═══════════════════════════════════════════════════════════════════════════
// Accelerate — vDSP, BLAS, vector math
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("accelerate_available")
public func accelerateAvailable() -> Bool { true }

// ── vDSP — vector operations ────────────────────────────────────────────────

@_cdecl("accelerate_vdsp_add_f32")
public func accelerateVdspAddF32(_ a: UnsafePointer<Float>, _ b: UnsafePointer<Float>,
                                  _ out: UnsafeMutablePointer<Float>, _ count: Int) {
    vDSP_vadd(a, 1, b, 1, out, 1, vDSP_Length(count))
}

@_cdecl("accelerate_vdsp_sub_f32")
public func accelerateVdspSubF32(_ a: UnsafePointer<Float>, _ b: UnsafePointer<Float>,
                                  _ out: UnsafeMutablePointer<Float>, _ count: Int) {
    vDSP_vsub(b, 1, a, 1, out, 1, vDSP_Length(count))  // vDSP_vsub: out = a - b (note: args swapped)
}

@_cdecl("accelerate_vdsp_mul_f32")
public func accelerateVdspMulF32(_ a: UnsafePointer<Float>, _ b: UnsafePointer<Float>,
                                  _ out: UnsafeMutablePointer<Float>, _ count: Int) {
    vDSP_vmul(a, 1, b, 1, out, 1, vDSP_Length(count))
}

@_cdecl("accelerate_vdsp_div_f32")
public func accelerateVdspDivF32(_ a: UnsafePointer<Float>, _ b: UnsafePointer<Float>,
                                  _ out: UnsafeMutablePointer<Float>, _ count: Int) {
    vDSP_vdiv(b, 1, a, 1, out, 1, vDSP_Length(count))  // vDSP_vdiv: out = a / b (note: args swapped)
}

@_cdecl("accelerate_vdsp_scale_f32")
public func accelerateVdspScaleF32(_ a: UnsafePointer<Float>, _ scalar: Float,
                                    _ out: UnsafeMutablePointer<Float>, _ count: Int) {
    var s = scalar
    vDSP_vsmul(a, 1, &s, out, 1, vDSP_Length(count))
}

@_cdecl("accelerate_vdsp_dot_f32")
public func accelerateVdspDotF32(_ a: UnsafePointer<Float>, _ b: UnsafePointer<Float>,
                                  _ count: Int) -> Float {
    var result: Float = 0
    vDSP_dotpr(a, 1, b, 1, &result, vDSP_Length(count))
    return result
}

@_cdecl("accelerate_vdsp_sum_f32")
public func accelerateVdspSumF32(_ a: UnsafePointer<Float>, _ count: Int) -> Float {
    var result: Float = 0
    vDSP_sve(a, 1, &result, vDSP_Length(count))
    return result
}

@_cdecl("accelerate_vdsp_mean_f32")
public func accelerateVdspMeanF32(_ a: UnsafePointer<Float>, _ count: Int) -> Float {
    var result: Float = 0
    vDSP_meanv(a, 1, &result, vDSP_Length(count))
    return result
}

@_cdecl("accelerate_vdsp_max_f32")
public func accelerateVdspMaxF32(_ a: UnsafePointer<Float>, _ count: Int) -> Float {
    var result: Float = 0
    vDSP_maxv(a, 1, &result, vDSP_Length(count))
    return result
}

@_cdecl("accelerate_vdsp_min_f32")
public func accelerateVdspMinF32(_ a: UnsafePointer<Float>, _ count: Int) -> Float {
    var result: Float = 0
    vDSP_minv(a, 1, &result, vDSP_Length(count))
    return result
}

@_cdecl("accelerate_vdsp_rms_f32")
public func accelerateVdspRmsF32(_ a: UnsafePointer<Float>, _ count: Int) -> Float {
    var result: Float = 0
    vDSP_rmsqv(a, 1, &result, vDSP_Length(count))
    return result
}

@_cdecl("accelerate_vdsp_normalize_f32")
public func accelerateVdspNormalizeF32(_ a: UnsafePointer<Float>,
                                        _ out: UnsafeMutablePointer<Float>,
                                        _ count: Int,
                                        _ mean: UnsafeMutablePointer<Float>,
                                        _ stddev: UnsafeMutablePointer<Float>) {
    vDSP_normalize(a, 1, out, 1, mean, stddev, vDSP_Length(count))
}

// ── vDSP f64 ────────────────────────────────────────────────────────────────

@_cdecl("accelerate_vdsp_add_f64")
public func accelerateVdspAddF64(_ a: UnsafePointer<Double>, _ b: UnsafePointer<Double>,
                                  _ out: UnsafeMutablePointer<Double>, _ count: Int) {
    vDSP_vaddD(a, 1, b, 1, out, 1, vDSP_Length(count))
}

@_cdecl("accelerate_vdsp_mul_f64")
public func accelerateVdspMulF64(_ a: UnsafePointer<Double>, _ b: UnsafePointer<Double>,
                                  _ out: UnsafeMutablePointer<Double>, _ count: Int) {
    vDSP_vmulD(a, 1, b, 1, out, 1, vDSP_Length(count))
}

@_cdecl("accelerate_vdsp_dot_f64")
public func accelerateVdspDotF64(_ a: UnsafePointer<Double>, _ b: UnsafePointer<Double>,
                                  _ count: Int) -> Double {
    var result: Double = 0
    vDSP_dotprD(a, 1, b, 1, &result, vDSP_Length(count))
    return result
}

@_cdecl("accelerate_vdsp_sum_f64")
public func accelerateVdspSumF64(_ a: UnsafePointer<Double>, _ count: Int) -> Double {
    var result: Double = 0
    vDSP_sveD(a, 1, &result, vDSP_Length(count))
    return result
}

// ── FFT ─────────────────────────────────────────────────────────────────────

@_cdecl("accelerate_vdsp_fft_f32")
public func accelerateVdspFftF32(_ input: UnsafePointer<Float>,
                                   _ realOut: UnsafeMutablePointer<Float>,
                                   _ imagOut: UnsafeMutablePointer<Float>,
                                   _ log2n: Int, _ forward: Bool) {
    let n = 1 << log2n
    let halfN = n / 2
    guard let setup = vDSP_create_fftsetup(vDSP_Length(log2n), FFTRadix(kFFTRadix2)) else { return }

    var splitReal = [Float](repeating: 0, count: halfN)
    var splitImag = [Float](repeating: 0, count: halfN)

    splitReal.withUnsafeMutableBufferPointer { realBuf in
        splitImag.withUnsafeMutableBufferPointer { imagBuf in
            var split = DSPSplitComplex(realp: realBuf.baseAddress!, imagp: imagBuf.baseAddress!)
            // Convert interleaved to split complex
            input.withMemoryRebound(to: DSPComplex.self, capacity: halfN) { ptr in
                vDSP_ctoz(ptr, 2, &split, 1, vDSP_Length(halfN))
            }
            let dir = forward ? FFTDirection(kFFTDirection_Forward) : FFTDirection(kFFTDirection_Inverse)
            vDSP_fft_zrip(setup, &split, 1, vDSP_Length(log2n), dir)
            // Copy out
            for i in 0..<halfN {
                realOut[i] = realBuf[i]
                imagOut[i] = imagBuf[i]
            }
        }
    }
    vDSP_destroy_fftsetup(setup)
}

// ── Matrix multiply (BLAS) ──────────────────────────────────────────────────

@_cdecl("accelerate_blas_sgemm")
public func accelerateBlasGemm(_ a: UnsafePointer<Float>, _ b: UnsafePointer<Float>,
                                _ out: UnsafeMutablePointer<Float>,
                                _ m: Int32, _ n: Int32, _ k: Int32,
                                _ alpha: Float, _ beta: Float) {
    // C = alpha * A * B + beta * C
    // A is m×k, B is k×n, C is m×n (row-major)
    cblas_sgemm(CblasRowMajor, CblasNoTrans, CblasNoTrans,
                m, n, k, alpha, a, k, b, n, beta, out, n)
}

@_cdecl("accelerate_blas_dgemm")
public func accelerateBlasDgemm(_ a: UnsafePointer<Double>, _ b: UnsafePointer<Double>,
                                 _ out: UnsafeMutablePointer<Double>,
                                 _ m: Int32, _ n: Int32, _ k: Int32,
                                 _ alpha: Double, _ beta: Double) {
    cblas_dgemm(CblasRowMajor, CblasNoTrans, CblasNoTrans,
                m, n, k, alpha, a, k, b, n, beta, out, n)
}
