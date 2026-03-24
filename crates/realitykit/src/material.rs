//! Material types — Simple, Unlit, PhysicallyBased, Occlusion, and Texture.


use core::ffi::c_void;

/// Opaque handle to any RealityKit `Material` (existential `any Material` wrapped in Swift).
pub struct Material {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for Material {}
unsafe impl Sync for Material {}
impl Clone for Material {
    fn clone(&self) -> Self { unsafe { realitykit_sys::rk_retain(self.ptr) }; Material { ptr: self.ptr } }
}
impl Drop for Material {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

/// A loaded `TextureResource`.
pub struct TextureResource {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for TextureResource {}
unsafe impl Sync for TextureResource {}
impl Clone for TextureResource {
    fn clone(&self) -> Self { unsafe { realitykit_sys::rk_retain(self.ptr) }; TextureResource { ptr: self.ptr } }
}
impl Drop for TextureResource {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}
impl TextureResource {
    /// Load from a file path (PNG, JPEG, EXR, etc.).
    pub fn load(path: &str) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe { realitykit_sys::rk_texture_load(path.as_ptr(), path.len(), err.as_mut_ptr(), err.len()) };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(TextureResource { ptr: p })
        }
    }

    /// Create a texture from raw RGBA8 pixel bytes at runtime.
    ///
    /// `data` must be exactly `width * height * 4` bytes (R, G, B, A per pixel,
    /// top-left origin).
    ///
    /// Requires macOS 12+ / iOS 15+. Returns `Err` on older systems.
    pub fn from_rgba8(data: &[u8], width: u32, height: u32) -> Result<Self, String> {
        assert_eq!(data.len(), (width * height * 4) as usize,
            "data length must be width*height*4");
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_texture_from_rgba8(
                data.as_ptr(), data.len(),
                width as i32, height as i32,
                err.as_mut_ptr(), err.len(),
            )
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(TextureResource { ptr: p })
        }
    }
}

// ─── Free constructors ────────────────────────────────────────────────────────

impl Material {
    /// `SimpleMaterial` with RGB colour + roughness + metallic flag.
    pub fn simple(r: f32, g: f32, b: f32) -> Self {
        Material { ptr: unsafe { realitykit_sys::rk_material_simple(r, g, b, 0.5, false) } }
    }
    /// `SimpleMaterial` with full parameters.
    pub fn simple_full(r: f32, g: f32, b: f32, roughness: f32, metallic: bool) -> Self {
        Material { ptr: unsafe { realitykit_sys::rk_material_simple(r, g, b, roughness, metallic) } }
    }
    /// `SimpleMaterial` with alpha transparency.
    pub fn simple_alpha(r: f32, g: f32, b: f32, a: f32, roughness: f32, metallic: bool) -> Self {
        Material { ptr: unsafe { realitykit_sys::rk_material_simple_alpha(r, g, b, a, roughness, metallic) } }
    }
    /// `UnlitMaterial` — no shading, solid colour.
    pub fn unlit(r: f32, g: f32, b: f32) -> Self {
        Material { ptr: unsafe { realitykit_sys::rk_material_unlit(r, g, b) } }
    }
    /// `UnlitMaterial` with alpha transparency.
    pub fn unlit_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
        Material { ptr: unsafe { realitykit_sys::rk_material_unlit_alpha(r, g, b, a) } }
    }
    /// `OcclusionMaterial` — makes the entity occlude other objects without appearing itself.
    pub fn occlusion() -> Self {
        Material { ptr: unsafe { realitykit_sys::rk_material_occlusion() } }
    }
    /// Begin building a `PhysicallyBasedMaterial` with the fluent builder.
    pub fn pbr() -> MaterialBuilder {
        let ptr = unsafe { realitykit_sys::rk_material_pbr_new() };
        MaterialBuilder { ptr }
    }
}

// ─── PBR builder ──────────────────────────────────────────────────────────────

/// Fluent builder for `PhysicallyBasedMaterial`.
///
/// Call `.build()` (or `.into()`) to get a [`Material`] handle.
///
/// ```ignore
/// let mat = Material::pbr()
///     .base_color(0.8, 0.2, 0.2, 1.0)
///     .roughness(0.4)
///     .metallic(0.0)
///     .emissive(1.0, 0.5, 0.0, 2.0)
///     .build();
/// ```
pub struct MaterialBuilder {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for MaterialBuilder {}
unsafe impl Sync for MaterialBuilder {}
impl Drop for MaterialBuilder {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

impl MaterialBuilder {
    /// RGBA base colour (values 0–1).
    pub fn base_color(self, r: f32, g: f32, b: f32, a: f32) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_base_color(self.ptr, r, g, b, a) }; self
    }
    pub fn roughness(self, v: f32) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_roughness(self.ptr, v) }; self
    }
    pub fn metallic(self, v: f32) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_metallic(self.ptr, v) }; self
    }
    /// Emissive colour + intensity multiplier.
    pub fn emissive(self, r: f32, g: f32, b: f32, intensity: f32) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_emissive(self.ptr, r, g, b, intensity) }; self
    }
    /// Opacity (0 = transparent, 1 = opaque).
    pub fn opacity(self, v: f32) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_opacity(self.ptr, v) }; self
    }
    /// Clearcoat layer (0–1) and its roughness.
    pub fn clearcoat(self, cc: f32, roughness: f32) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_clearcoat(self.ptr, cc, roughness) }; self
    }
    /// Sheen colour (0–1 per channel).
    pub fn sheen(self, r: f32, g: f32, b: f32) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_sheen(self.ptr, r, g, b) }; self
    }
    /// Specular reflectance at normal incidence (0–1).
    pub fn specular(self, v: f32) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_specular(self.ptr, v) }; self
    }
    /// Anisotropy level (0–1).
    pub fn anisotropy(self, v: f32) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_anisotropy(self.ptr, v) }; self
    }
    /// Apply a base-colour texture.
    pub fn base_color_texture(self, tex: &TextureResource) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_base_color_texture(self.ptr, tex.ptr) }; self
    }
    /// Apply a tangent-space normal map.
    pub fn normal_texture(self, tex: &TextureResource) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_normal_texture(self.ptr, tex.ptr) }; self
    }
    /// Apply a roughness map.
    pub fn roughness_texture(self, tex: &TextureResource) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_roughness_texture(self.ptr, tex.ptr) }; self
    }
    /// Apply a metallic map.
    pub fn metallic_texture(self, tex: &TextureResource) -> Self {
        unsafe { realitykit_sys::rk_material_pbr_set_metallic_texture(self.ptr, tex.ptr) }; self
    }

    /// Consume the builder and return the finished `Material`.
    pub fn build(self) -> Material {
        unsafe { realitykit_sys::rk_retain(self.ptr) };
        Material { ptr: self.ptr }
    }
}

impl From<MaterialBuilder> for Material {
    fn from(b: MaterialBuilder) -> Material { b.build() }
}

// ─── Additional material constructors ────────────────────────────────────────

impl Material {
    /// `VideoMaterial` — streams an `AVPlayer` video onto the entity's surface.
    ///
    /// `url` can be a file path or an `http(s)://` URL.  Call `video_play` /
    /// `video_pause` on the returned handle to control playback.
    pub fn video(url: &str) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_material_video(url.as_ptr(), url.len(), err.as_mut_ptr(), err.len())
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(Material { ptr: p })
        }
    }

    /// Start the underlying `AVPlayer` for a `VideoMaterial`.
    pub fn video_play(&self)  { unsafe { realitykit_sys::rk_material_video_play(self.ptr) } }

    /// Pause the underlying `AVPlayer` for a `VideoMaterial`.
    pub fn video_pause(&self) { unsafe { realitykit_sys::rk_material_video_pause(self.ptr) } }

    /// `PortalMaterial` — masks out depth for portal effects (macOS 15+ / iOS 18+;
    /// falls back to `OcclusionMaterial` on older systems).
    pub fn portal() -> Self {
        Material { ptr: unsafe { realitykit_sys::rk_material_portal_new() } }
    }

    /// Load a `ShaderGraphMaterial` by material name from a file.
    ///
    /// `material_name` is the material identifier inside the asset file,
    /// `file_path` is the path to the `.usda` / `.reality` file on disk.
    ///
    /// Requires macOS 15+ / iOS 18+ / visionOS 2+.
    pub fn shader_graph_from_file(material_name: &str, file_path: &str) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_material_shader_graph_from_file(
                material_name.as_ptr(), material_name.len(),
                file_path.as_ptr(), file_path.len(),
                err.as_mut_ptr(), err.len(),
            )
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(Material { ptr: p })
        }
    }
}


