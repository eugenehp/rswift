//! ObjC selector constants for MetalKit.
#![allow(dead_code)]

// ── MTKView (2 methods, 19 properties) ──
pub mod m_t_k_view {
    pub const CLASS: &[u8] = b"MTKView\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_DEVICE: &[u8] = b"device\0";
    pub const SEL_SET_DEVICE: &[u8] = b"setDevice:\0";
    pub const SEL_CURRENT_DRAWABLE: &[u8] = b"currentDrawable\0";
    pub const SEL_SET_CURRENT_DRAWABLE: &[u8] = b"setCurrentDrawable:\0";
    pub const SEL_FRAMEBUFFER_ONLY: &[u8] = b"framebufferOnly\0";
    pub const SEL_SET_FRAMEBUFFER_ONLY: &[u8] = b"setFramebufferOnly:\0";
    pub const SEL_PRESENTS_WITH_TRANSACTION: &[u8] = b"presentsWithTransaction\0";
    pub const SEL_SET_PRESENTS_WITH_TRANSACTION: &[u8] = b"setPresentsWithTransaction:\0";
    pub const SEL_COLOR_PIXEL_FORMAT: &[u8] = b"colorPixelFormat\0";
    pub const SEL_SET_COLOR_PIXEL_FORMAT: &[u8] = b"setColorPixelFormat:\0";
    pub const SEL_DEPTH_STENCIL_PIXEL_FORMAT: &[u8] = b"depthStencilPixelFormat\0";
    pub const SEL_SET_DEPTH_STENCIL_PIXEL_FORMAT: &[u8] = b"setDepthStencilPixelFormat:\0";
    pub const SEL_SAMPLE_COUNT: &[u8] = b"sampleCount\0";
    pub const SEL_SET_SAMPLE_COUNT: &[u8] = b"setSampleCount:\0";
    pub const SEL_CLEAR_COLOR: &[u8] = b"clearColor\0";
    pub const SEL_SET_CLEAR_COLOR: &[u8] = b"setClearColor:\0";
    pub const SEL_CLEAR_DEPTH: &[u8] = b"clearDepth\0";
    pub const SEL_SET_CLEAR_DEPTH: &[u8] = b"setClearDepth:\0";
    pub const SEL_CLEAR_STENCIL: &[u8] = b"clearStencil\0";
    pub const SEL_SET_CLEAR_STENCIL: &[u8] = b"setClearStencil:\0";
    pub const SEL_DEPTH_STENCIL_TEXTURE: &[u8] = b"depthStencilTexture\0";
    pub const SEL_SET_DEPTH_STENCIL_TEXTURE: &[u8] = b"setDepthStencilTexture:\0";
    pub const SEL_MULTISAMPLE_COLOR_TEXTURE: &[u8] = b"multisampleColorTexture\0";
    pub const SEL_SET_MULTISAMPLE_COLOR_TEXTURE: &[u8] = b"setMultisampleColorTexture:\0";
    pub const SEL_CURRENT_RENDER_PASS_DESCRIPTOR: &[u8] = b"currentRenderPassDescriptor\0";
    pub const SEL_SET_CURRENT_RENDER_PASS_DESCRIPTOR: &[u8] = b"setCurrentRenderPassDescriptor:\0";
    pub const SEL_PREFERRED_FRAMES_PER_SECOND: &[u8] = b"preferredFramesPerSecond\0";
    pub const SEL_SET_PREFERRED_FRAMES_PER_SECOND: &[u8] = b"setPreferredFramesPerSecond:\0";
    pub const SEL_ENABLE_SET_NEEDS_DISPLAY: &[u8] = b"enableSetNeedsDisplay\0";
    pub const SEL_SET_ENABLE_SET_NEEDS_DISPLAY: &[u8] = b"setEnableSetNeedsDisplay:\0";
    pub const SEL_AUTO_RESIZE_DRAWABLE: &[u8] = b"autoResizeDrawable\0";
    pub const SEL_SET_AUTO_RESIZE_DRAWABLE: &[u8] = b"setAutoResizeDrawable:\0";
    pub const SEL_DRAWABLE_SIZE: &[u8] = b"drawableSize\0";
    pub const SEL_SET_DRAWABLE_SIZE: &[u8] = b"setDrawableSize:\0";
    pub const SEL_PAUSED: &[u8] = b"paused\0";
    pub const SEL_SET_PAUSED: &[u8] = b"setPaused:\0";
    pub const SEL_RELEASE_DRAWABLES: &[u8] = b"releaseDrawables\0";
    pub const SEL_DRAW: &[u8] = b"draw\0";
}

// ── MTKTextureLoader (16 methods, 1 properties) ──
pub mod m_t_k_texture_loader {
    pub const SEL_NEW_TEXTURE_WITH_CONTENTS_OF_U_R_L: &[u8] = b"newTextureWithContentsOfURL:options:completionHandler:\0";
    pub const SEL_NEW_TEXTURE_WITH_NAME: &[u8] = b"newTextureWithName:scaleFactor:bundle:options:completionHandler:\0";
    pub const SEL_NEW_TEXTURES_WITH_CONTENTS_OF_U_R_LS: &[u8] = b"newTexturesWithContentsOfURLs:options:completionHandler:\0";
    pub const SEL_NEW_TEXTURES_WITH_NAMES: &[u8] = b"newTexturesWithNames:scaleFactor:bundle:options:completionHandler:\0";
    pub const SEL_NEW_TEXTURE_WITH_DATA: &[u8] = b"newTextureWithData:options:completionHandler:\0";
    pub const SEL_NEW_TEXTURE_WITH_C_G_IMAGE: &[u8] = b"newTextureWithCGImage:options:completionHandler:\0";
    pub const SEL_NEW_TEXTURE_WITH_M_D_L_TEXTURE: &[u8] = b"newTextureWithMDLTexture:options:completionHandler:\0";
}

// ── MTKMesh (1 methods, 5 properties) ──
pub mod m_t_k_mesh {
    pub const SEL_VERTEX_BUFFERS: &[u8] = b"vertexBuffers\0";
    pub const SEL_SET_VERTEX_BUFFERS: &[u8] = b"setVertexBuffers:\0";
    pub const SEL_VERTEX_DESCRIPTOR: &[u8] = b"vertexDescriptor\0";
    pub const SEL_SET_VERTEX_DESCRIPTOR: &[u8] = b"setVertexDescriptor:\0";
    pub const SEL_SUBMESHES: &[u8] = b"submeshes\0";
    pub const SEL_SET_SUBMESHES: &[u8] = b"setSubmeshes:\0";
    pub const SEL_VERTEX_COUNT: &[u8] = b"vertexCount\0";
    pub const SEL_SET_VERTEX_COUNT: &[u8] = b"setVertexCount:\0";
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_SET_NAME: &[u8] = b"setName:\0";
    pub const SEL_NEW_MESHES_FROM_ASSET: &[u8] = b"newMeshesFromAsset:device:sourceMeshes:error:\0";
}

// Total: 60 selector constants
