import Metal
import Foundation

// ─── Boxing helpers ───────────────────────────────────────────────────────────

public typealias MTLHandle = UnsafeMutableRawPointer

@inline(__always) func box<T: AnyObject>(_ o: T) -> MTLHandle { Unmanaged.passRetained(o).toOpaque() }
@inline(__always) func unbox<T: AnyObject>(_ h: MTLHandle) -> T { Unmanaged<T>.fromOpaque(h).takeUnretainedValue() }

final class Wrap<T>: NSObject { let v: T; init(_ v: T) { self.v = v } }
@inline(__always) func boxv<T>(_ v: T) -> MTLHandle { box(Wrap(v)) }
@inline(__always) func unboxv<T>(_ h: MTLHandle) -> T { (unbox(h) as Wrap<T>).v }

func writeErr(_ msg: String, _ buf: UnsafeMutablePointer<UInt8>?, _ len: Int) {
    guard let buf, len > 0 else { return }
    let b = Array(msg.utf8.prefix(len - 1))
    for (i, c) in b.enumerated() { buf[i] = c }
    buf[min(b.count, len - 1)] = 0
}

// ─── Device ──────────────────────────────────────────────────────────────────

@_cdecl("mtl_create_system_default_device")
public func mtlCreateSystemDefaultDevice() -> MTLHandle? {
    guard let d = MTLCreateSystemDefaultDevice() else { return nil }
    return box(d)
}

@_cdecl("mtl_release") public func mtlRelease(_ h: MTLHandle) { Unmanaged<AnyObject>.fromOpaque(h).release() }

@_cdecl("mtl_device_name")
public func mtlDeviceName(_ h: MTLHandle, _ buf: UnsafeMutablePointer<UInt8>, _ len: Int) -> Int {
    let name = (unbox(h) as MTLDevice).name
    let b = Array(name.utf8.prefix(max(0, len-1)))
    for (i, c) in b.enumerated() { buf[i] = c }; if b.count < len { buf[b.count] = 0 }; return b.count
}

@_cdecl("mtl_device_has_unified_memory")
public func mtlDeviceHasUnifiedMemory(_ h: MTLHandle) -> Bool { (unbox(h) as MTLDevice).hasUnifiedMemory }

@_cdecl("mtl_device_max_buffer_length")
public func mtlDeviceMaxBufferLength(_ h: MTLHandle) -> Int { (unbox(h) as MTLDevice).maxBufferLength }

@_cdecl("mtl_device_max_threads_per_threadgroup")
public func mtlDeviceMaxThreadsPerThreadgroup(_ h: MTLHandle) -> Int {
    let d = unbox(h) as MTLDevice
    let s = d.maxThreadsPerThreadgroup
    return s.width * s.height * s.depth
}

@_cdecl("mtl_device_supports_raytracing")
public func mtlDeviceSupportsRaytracing(_ h: MTLHandle) -> Bool { (unbox(h) as MTLDevice).supportsRaytracing }

@_cdecl("mtl_device_supports_family")
public func mtlDeviceSupportsFamily(_ h: MTLHandle, _ family: Int) -> Bool {
    guard let f = MTLGPUFamily(rawValue: family) else { return false }
    return (unbox(h) as MTLDevice).supportsFamily(f)
}

// ─── Command queue ────────────────────────────────────────────────────────────

@_cdecl("mtl_device_make_command_queue")
public func mtlDeviceMakeCommandQueue(_ h: MTLHandle) -> MTLHandle? {
    guard let q = (unbox(h) as MTLDevice).makeCommandQueue() else { return nil }
    return box(q)
}

@_cdecl("mtl_command_queue_command_buffer")
public func mtlCommandQueueCommandBuffer(_ h: MTLHandle) -> MTLHandle? {
    guard let b = (unbox(h) as MTLCommandQueue).makeCommandBuffer() else { return nil }
    return box(b)
}

// ─── Command buffer ───────────────────────────────────────────────────────────

@_cdecl("mtl_command_buffer_commit")
public func mtlCommandBufferCommit(_ h: MTLHandle) { (unbox(h) as MTLCommandBuffer).commit() }

@_cdecl("mtl_command_buffer_wait")
public func mtlCommandBufferWait(_ h: MTLHandle) { (unbox(h) as MTLCommandBuffer).waitUntilCompleted() }

@_cdecl("mtl_command_buffer_status")
public func mtlCommandBufferStatus(_ h: MTLHandle) -> Int { Int((unbox(h) as MTLCommandBuffer).status.rawValue) }

@_cdecl("mtl_command_buffer_gpu_start_time")
public func mtlCommandBufferGPUStartTime(_ h: MTLHandle) -> Double { (unbox(h) as MTLCommandBuffer).gpuStartTime }

@_cdecl("mtl_command_buffer_gpu_end_time")
public func mtlCommandBufferGPUEndTime(_ h: MTLHandle) -> Double { (unbox(h) as MTLCommandBuffer).gpuEndTime }

// ─── Compute encoder ─────────────────────────────────────────────────────────

@_cdecl("mtl_command_buffer_compute_encoder")
public func mtlCommandBufferComputeEncoder(_ h: MTLHandle) -> MTLHandle? {
    guard let e = (unbox(h) as MTLCommandBuffer).makeComputeCommandEncoder() else { return nil }
    return box(e)
}

@_cdecl("mtl_compute_encoder_set_pipeline")
public func mtlComputeEncoderSetPipeline(_ enc: MTLHandle, _ pip: MTLHandle) {
    (unbox(enc) as MTLComputeCommandEncoder).setComputePipelineState(unbox(pip))
}

@_cdecl("mtl_compute_encoder_set_buffer")
public func mtlComputeEncoderSetBuffer(_ enc: MTLHandle, _ buf: MTLHandle, _ offset: Int, _ idx: Int) {
    (unbox(enc) as MTLComputeCommandEncoder).setBuffer(unbox(buf), offset: offset, index: idx)
}

@_cdecl("mtl_compute_encoder_set_bytes")
public func mtlComputeEncoderSetBytes(_ enc: MTLHandle, _ ptr: UnsafeRawPointer, _ len: Int, _ idx: Int) {
    (unbox(enc) as MTLComputeCommandEncoder).setBytes(ptr, length: len, index: idx)
}

@_cdecl("mtl_compute_encoder_set_texture")
public func mtlComputeEncoderSetTexture(_ enc: MTLHandle, _ tex: MTLHandle, _ idx: Int) {
    (unbox(enc) as MTLComputeCommandEncoder).setTexture(unbox(tex), index: idx)
}

@_cdecl("mtl_compute_encoder_dispatch_threads")
public func mtlComputeEncoderDispatchThreads(
    _ enc: MTLHandle,
    _ gx: Int, _ gy: Int, _ gz: Int,
    _ tx: Int, _ ty: Int, _ tz: Int
) {
    let e = unbox(enc) as MTLComputeCommandEncoder
    e.dispatchThreads(MTLSize(width:gx,height:gy,depth:gz),
                      threadsPerThreadgroup:MTLSize(width:tx,height:ty,depth:tz))
}

@_cdecl("mtl_compute_encoder_dispatch_threadgroups")
public func mtlComputeEncoderDispatchThreadgroups(
    _ enc: MTLHandle,
    _ gx: Int, _ gy: Int, _ gz: Int,
    _ tx: Int, _ ty: Int, _ tz: Int
) {
    let e = unbox(enc) as MTLComputeCommandEncoder
    e.dispatchThreadgroups(MTLSize(width:gx,height:gy,depth:gz),
                           threadsPerThreadgroup:MTLSize(width:tx,height:ty,depth:tz))
}

@_cdecl("mtl_compute_encoder_end")
public func mtlComputeEncoderEnd(_ h: MTLHandle) { (unbox(h) as MTLComputeCommandEncoder).endEncoding() }

// ─── Blit encoder ─────────────────────────────────────────────────────────────

@_cdecl("mtl_command_buffer_blit_encoder")
public func mtlCommandBufferBlitEncoder(_ h: MTLHandle) -> MTLHandle? {
    guard let e = (unbox(h) as MTLCommandBuffer).makeBlitCommandEncoder() else { return nil }
    return box(e)
}

@_cdecl("mtl_blit_encoder_copy_buffer")
public func mtlBlitEncoderCopyBuffer(
    _ enc: MTLHandle,
    _ src: MTLHandle, _ srcOff: Int,
    _ dst: MTLHandle, _ dstOff: Int,
    _ size: Int
) {
    (unbox(enc) as MTLBlitCommandEncoder)
        .copy(from:unbox(src), sourceOffset:srcOff, to:unbox(dst), destinationOffset:dstOff, size:size)
}

@_cdecl("mtl_blit_encoder_fill_buffer")
public func mtlBlitEncoderFillBuffer(_ enc: MTLHandle, _ buf: MTLHandle, _ off: Int, _ len: Int, _ val: UInt8) {
    (unbox(enc) as MTLBlitCommandEncoder)
        .fill(buffer:unbox(buf), range:off..<(off+len), value:val)
}

@_cdecl("mtl_blit_encoder_end")
public func mtlBlitEncoderEnd(_ h: MTLHandle) { (unbox(h) as MTLBlitCommandEncoder).endEncoding() }

// ─── Buffers ──────────────────────────────────────────────────────────────────

@_cdecl("mtl_device_make_buffer")
public func mtlDeviceMakeBuffer(_ dev: MTLHandle, _ len: Int, _ opts: UInt64) -> MTLHandle? {
    guard let b = (unbox(dev) as MTLDevice).makeBuffer(length:len, options:MTLResourceOptions(rawValue: UInt(opts))) else { return nil }
    return box(b)
}

@_cdecl("mtl_device_make_buffer_with_bytes")
public func mtlDeviceMakeBufferWithBytes(_ dev: MTLHandle, _ ptr: UnsafeRawPointer, _ len: Int, _ opts: UInt64) -> MTLHandle? {
    guard let b = (unbox(dev) as MTLDevice).makeBuffer(bytes:ptr, length:len, options:MTLResourceOptions(rawValue: UInt(opts))) else { return nil }
    return box(b)
}

@_cdecl("mtl_buffer_contents")
public func mtlBufferContents(_ h: MTLHandle) -> UnsafeMutableRawPointer? { (unbox(h) as MTLBuffer).contents() }

@_cdecl("mtl_buffer_length")
public func mtlBufferLength(_ h: MTLHandle) -> Int { (unbox(h) as MTLBuffer).length }

// ─── Textures ─────────────────────────────────────────────────────────────────

@_cdecl("mtl_device_make_texture")
public func mtlDeviceMakeTexture(
    _ dev: MTLHandle,
    _ pixFmt: UInt64, _ width: Int, _ height: Int,
    _ mipmaps: Int, _ sampleCount: Int,
    _ arrayLen: Int, _ type: Int,
    _ usage: UInt64, _ opts: UInt64
) -> MTLHandle? {
    let desc = MTLTextureDescriptor()
    desc.pixelFormat    = MTLPixelFormat(rawValue: UInt(pixFmt)) ?? .bgra8Unorm
    desc.width          = width
    desc.height         = height
    desc.mipmapLevelCount = mipmaps
    desc.sampleCount    = sampleCount
    desc.arrayLength    = arrayLen
    desc.textureType    = MTLTextureType(rawValue:UInt(type)) ?? .type2D
    desc.usage          = MTLTextureUsage(rawValue: UInt(usage))
    desc.resourceOptions = MTLResourceOptions(rawValue: UInt(opts))
    guard let t = (unbox(dev) as MTLDevice).makeTexture(descriptor:desc) else { return nil }
    return box(t)
}

@_cdecl("mtl_texture_width")  public func mtlTextureWidth(_ h:MTLHandle) -> Int { (unbox(h) as MTLTexture).width }
@_cdecl("mtl_texture_height") public func mtlTextureHeight(_ h:MTLHandle) -> Int { (unbox(h) as MTLTexture).height }
@_cdecl("mtl_texture_pixel_format")
public func mtlTexturePixelFormat(_ h:MTLHandle) -> UInt64 { UInt64((unbox(h) as MTLTexture).pixelFormat.rawValue) }

@_cdecl("mtl_texture_replace_region")
public func mtlTextureReplaceRegion(
    _ h: MTLHandle,
    _ x: Int, _ y: Int, _ w: Int, _ ht: Int,
    _ mip: Int, _ bpr: Int,
    _ ptr: UnsafeRawPointer
) {
    (unbox(h) as MTLTexture).replace(
        region:MTLRegionMake2D(x,y,w,ht), mipmapLevel:mip, withBytes:ptr, bytesPerRow:bpr)
}

// ─── Libraries & functions ────────────────────────────────────────────────────

@_cdecl("mtl_device_make_default_library")
public func mtlDeviceMakeDefaultLibrary(_ h: MTLHandle) -> MTLHandle? {
    guard let lib = (unbox(h) as MTLDevice).makeDefaultLibrary() else { return nil }
    return box(lib)
}

@_cdecl("mtl_device_make_library_source")
public func mtlDeviceMakeLibrarySource(
    _ dev: MTLHandle,
    _ src: UnsafePointer<UInt8>, _ srcLen: Int,
    _ errBuf: UnsafeMutablePointer<UInt8>?, _ errLen: Int
) -> MTLHandle? {
    let source = String(bytes:UnsafeBufferPointer(start:src,count:srcLen),encoding:.utf8) ?? ""
    let opts = MTLCompileOptions()
    do {
        let lib = try (unbox(dev) as MTLDevice).makeLibrary(source:source, options:opts)
        return box(lib)
    } catch {
        writeErr(error.localizedDescription, errBuf, errLen)
        return nil
    }
}

@_cdecl("mtl_library_make_function")
public func mtlLibraryMakeFunction(_ lib: MTLHandle, _ np: UnsafePointer<UInt8>, _ nl: Int) -> MTLHandle? {
    let name = String(bytes:UnsafeBufferPointer(start:np,count:nl),encoding:.utf8) ?? ""
    guard let fn = (unbox(lib) as MTLLibrary).makeFunction(name:name) else { return nil }
    return box(fn)
}

@_cdecl("mtl_library_function_names")
public func mtlLibraryFunctionNames(_ lib: MTLHandle, _ buf: UnsafeMutablePointer<UInt8>, _ len: Int) -> Int {
    let names = (unbox(lib) as MTLLibrary).functionNames.joined(separator:",")
    let b = Array(names.utf8.prefix(max(0, len-1)))
    for (i, c) in b.enumerated() { buf[i] = c }; if b.count < len { buf[b.count] = 0 }; return b.count
}

// ─── Pipelines ───────────────────────────────────────────────────────────────

@_cdecl("mtl_device_make_compute_pipeline")
public func mtlDeviceMakeComputePipeline(
    _ dev: MTLHandle, _ fn: MTLHandle,
    _ errBuf: UnsafeMutablePointer<UInt8>?, _ errLen: Int
) -> MTLHandle? {
    do {
        let pip = try (unbox(dev) as MTLDevice)
            .makeComputePipelineState(function: unbox(fn))
        return box(pip)
    } catch {
        writeErr(error.localizedDescription, errBuf, errLen)
        return nil
    }
}

@_cdecl("mtl_compute_pipeline_max_threads")
public func mtlComputePipelineMaxThreads(_ h: MTLHandle) -> Int {
    (unbox(h) as MTLComputePipelineState).maxTotalThreadsPerThreadgroup
}

@_cdecl("mtl_compute_pipeline_thread_execution_width")
public func mtlComputePipelineThreadExecutionWidth(_ h: MTLHandle) -> Int {
    (unbox(h) as MTLComputePipelineState).threadExecutionWidth
}

@_cdecl("mtl_device_make_render_pipeline")
public func mtlDeviceMakeRenderPipeline(
    _ dev: MTLHandle,
    _ vertFn: MTLHandle, _ fragFn: MTLHandle,
    _ pixFmt: UInt64,
    _ errBuf: UnsafeMutablePointer<UInt8>?, _ errLen: Int
) -> MTLHandle? {
    let desc = MTLRenderPipelineDescriptor()
    desc.vertexFunction   = unbox(vertFn)
    desc.fragmentFunction = unbox(fragFn)
    desc.colorAttachments[0].pixelFormat = MTLPixelFormat(rawValue: UInt(pixFmt)) ?? .bgra8Unorm
    do {
        let pip = try (unbox(dev) as MTLDevice).makeRenderPipelineState(descriptor:desc)
        return box(pip)
    } catch {
        writeErr(error.localizedDescription, errBuf, errLen)
        return nil
    }
}
