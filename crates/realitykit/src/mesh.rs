//! Mesh resource creation — all RealityKit primitive generators.


use core::ffi::c_void;

/// A retained handle to a `MeshResource` in Swift.
pub struct MeshResource {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for MeshResource {}
unsafe impl Sync for MeshResource {}
impl Clone for MeshResource {
    fn clone(&self) -> Self { unsafe { realitykit_sys::rk_retain(self.ptr) }; MeshResource { ptr: self.ptr } }
}
impl Drop for MeshResource {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

impl MeshResource {
    // ── Boxes ──────────────────────────────────────────────────────────────

    /// Axis-aligned box.
    pub fn box_(w: f32, h: f32, d: f32) -> Self {
        MeshResource { ptr: unsafe { realitykit_sys::rk_mesh_box(w, h, d) } }
    }
    /// Box with rounded corners (`corner_radius` in metres).
    pub fn box_chamfer(w: f32, h: f32, d: f32, corner_radius: f32) -> Self {
        MeshResource { ptr: unsafe { realitykit_sys::rk_mesh_box_chamfer(w, h, d, corner_radius) } }
    }
    /// Cube (equal sides).
    pub fn cube(size: f32) -> Self { Self::box_(size, size, size) }

    // ── Curved primitives ─────────────────────────────────────────────────

    /// UV sphere.
    pub fn sphere(radius: f32) -> Self {
        MeshResource { ptr: unsafe { realitykit_sys::rk_mesh_sphere(radius) } }
    }
    /// Flat plane in the XZ plane.
    pub fn plane(width: f32, depth: f32) -> Self {
        MeshResource { ptr: unsafe { realitykit_sys::rk_mesh_plane(width, depth) } }
    }
    /// Flat plane with rounded corners.
    pub fn plane_rounded(width: f32, depth: f32, corner_radius: f32) -> Self {
        MeshResource { ptr: unsafe { realitykit_sys::rk_mesh_plane_corner_radius(width, depth, corner_radius) } }
    }
    /// Cone (tip at top).
    pub fn cone(height: f32, radius: f32) -> Self {
        MeshResource { ptr: unsafe { realitykit_sys::rk_mesh_cone(height, radius) } }
    }
    /// Open cylinder (no caps).
    pub fn cylinder(height: f32, radius: f32) -> Self {
        MeshResource { ptr: unsafe { realitykit_sys::rk_mesh_cylinder(height, radius) } }
    }
    /// Capsule (cylinder + hemispherical end caps).
    pub fn capsule(height: f32, radius: f32) -> Self {
        MeshResource { ptr: unsafe { realitykit_sys::rk_mesh_capsule(height, radius) } }
    }
    /// Torus (donut). Available on macOS 13+/iOS 16+; falls back to sphere on older OS.
    pub fn torus(mean_radius: f32, tube_radius: f32) -> Self {
        MeshResource { ptr: unsafe { realitykit_sys::rk_mesh_torus(mean_radius, tube_radius) } }
    }

    // ── 3-D text ──────────────────────────────────────────────────────────

    /// Extruded 3-D text mesh.
    pub fn text(string: &str, extrusion_depth: f32, font_size: f32) -> Self {
        MeshResource {
            ptr: unsafe { realitykit_sys::rk_mesh_text(string.as_ptr(), string.len(), extrusion_depth, font_size) }
        }
    }

    // ── Custom / procedural ───────────────────────────────────────────────

    /// Build a mesh from raw vertex buffers.
    ///
    /// * `positions` — slice of `[x, y, z]` vertices (flat `f32` triples)
    /// * `normals`   — optional per-vertex normals (same count as positions)
    /// * `uvs`       — optional per-vertex texture coordinates (`[u, v]` pairs)
    /// * `indices`   — optional triangle index list (u32 triples; auto-indexed if `None`)
    ///
    /// Returns `Err` if RealityKit rejects the descriptor (e.g. degenerate mesh).
    ///
    /// # Example
    /// ```ignore
    /// let positions: &[f32] = &[0.,0.,0., 1.,0.,0., 0.,1.,0.];
    /// let indices:   &[u32] = &[0, 1, 2];
    /// let mesh = MeshResource::from_buffers(positions, None, None, Some(indices)).unwrap();
    /// ```
    pub fn from_buffers(
        positions: &[f32],
        normals:   Option<&[f32]>,
        uvs:       Option<&[f32]>,
        indices:   Option<&[u32]>,
    ) -> Result<Self, String> {
        assert_eq!(positions.len() % 3, 0, "positions must be f32 triples");
        if let Some(n) = normals  { assert_eq!(n.len() % 3, 0, "normals must be f32 triples"); }
        if let Some(u) = uvs      { assert_eq!(u.len() % 2, 0, "uvs must be f32 pairs"); }
        if let Some(i) = indices  { assert_eq!(i.len() % 3, 0, "indices must be u32 triples"); }

        let (np, nc) = normals.map_or((core::ptr::null(), 0), |n| (n.as_ptr(), n.len() / 3));
        let (up, uc) = uvs    .map_or((core::ptr::null(), 0), |u| (u.as_ptr(), u.len() / 2));
        let (ip, ic) = indices.map_or((core::ptr::null(), 0), |i| (i.as_ptr(), i.len()));

        let mut err = [0u8; 2048];
        let ptr = unsafe {
            realitykit_sys::rk_mesh_from_buffers(
                positions.as_ptr(), positions.len() / 3,
                np, nc,
                up, uc,
                ip, ic,
                err.as_mut_ptr(), err.len(),
            )
        };
        if ptr.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(MeshResource { ptr })
        }
    }
}
