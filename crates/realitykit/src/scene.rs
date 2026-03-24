//! Scene / ARView management — ARView, raycasting, entity queries, event subscriptions.

use crate::Entity;
use core::ffi::c_void;

/// Bitmask of rendering features to disable on an `ARView`.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct RenderOptions(pub u32);

impl RenderOptions {
    pub const NONE:                     Self = Self(0);
    pub const DISABLE_MOTION_BLUR:       Self = Self(1);
    pub const DISABLE_DEPTH_OF_FIELD:    Self = Self(2);
    pub const DISABLE_HDR:               Self = Self(4);
    pub const DISABLE_GROUNDING_SHADOWS: Self = Self(8);
    pub const DISABLE_FACE_MESH:         Self = Self(16);
    pub const DISABLE_PERSON_OCCLUSION:  Self = Self(32);
    pub const DISABLE_CAMERA_GRAIN:      Self = Self(64);

    pub fn or(self, other: Self) -> Self { Self(self.0 | other.0) }
}

impl std::ops::BitOr for RenderOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

/// An `ARView` instance (macOS) or a null stub on other platforms.
/// Populate the scene by calling `add_anchor`.
pub struct Scene {
    ptr: *mut c_void,
}
unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}

impl Drop for Scene {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { realitykit_sys::rk_release(self.ptr) };
        }
    }
}

impl Scene {
    /// Create an `ARView` with the given pixel dimensions.
    ///
    /// On non-macOS platforms the view cannot be created through `@_cdecl`
    /// (use `RealityView` in SwiftUI instead); returns a null stub.
    pub fn new(width: f32, height: f32) -> Self {
        let raw = unsafe { realitykit_sys::rk_arview_new(width, height) };
        Scene { ptr: if raw.is_null() { std::ptr::null_mut() } else { raw } }
    }

    pub fn add_anchor(&self, anchor: &Entity) {
        if !self.ptr.is_null() {
            unsafe { realitykit_sys::rk_arview_scene_add_anchor(self.ptr, anchor.ptr) };
        }
    }
    pub fn remove_anchor(&self, anchor: &Entity) {
        if !self.ptr.is_null() {
            unsafe { realitykit_sys::rk_arview_scene_remove_anchor(self.ptr, anchor.ptr) };
        }
    }

    /// `true` = camera passthrough (AR), `false` = virtual-only.
    pub fn set_camera_mode(&self, ar: bool) {
        if !self.ptr.is_null() {
            unsafe { realitykit_sys::rk_arview_set_camera_mode(self.ptr, if ar { 0 } else { 1 }) };
        }
    }

    /// How strongly real-world lighting is reflected in PBR materials.
    pub fn set_environment_intensity(&self, v: f32) {
        if !self.ptr.is_null() {
            unsafe { realitykit_sys::rk_arview_set_environment_intensity(self.ptr, v) };
        }
    }

    /// Disable specific rendering features to improve performance.
    pub fn set_render_options(&self, opts: RenderOptions) {
        if !self.ptr.is_null() {
            unsafe { realitykit_sys::rk_arview_set_render_options(self.ptr, opts.0) };
        }
    }

    pub fn anchor_count(&self) -> usize {
        if self.ptr.is_null() { return 0; }
        unsafe { realitykit_sys::rk_arview_scene_anchor_count(self.ptr) }
    }

    pub fn anchor_at(&self, index: usize) -> Option<Entity> {
        if self.ptr.is_null() { return None; }
        let p = unsafe { realitykit_sys::rk_arview_scene_get_anchor(self.ptr, index) };
        if p.is_null() { None } else { Some(Entity { ptr: p }) }
    }

    /// Find any entity (anywhere in the scene hierarchy) by name.
    pub fn find_entity_named(&self, name: &str) -> Option<Entity> {
        if self.ptr.is_null() { return None; }
        let p = unsafe { realitykit_sys::rk_arview_find_entity_named(self.ptr, name.as_ptr(), name.len()) };
        if p.is_null() { None } else { Some(Entity { ptr: p }) }
    }

    pub fn raw(&self) -> *mut c_void { self.ptr }

    // ── Raycasting ────────────────────────────────────────────────────────

    /// Cast a ray and return up to `max_hits` results, sorted by distance.
    ///
    /// `query_mode`: 0 = nearest, 1 = all, 2 = any.
    ///
    /// On non-macOS platforms always returns an empty vec.
    pub fn raycast(
        &self,
        origin:     [f32; 3],
        direction:  [f32; 3],
        length:     f32,
        query_mode: i32,
        max_hits:   usize,
    ) -> Vec<RaycastHit> {
        if self.ptr.is_null() || max_hits == 0 { return Vec::new(); }
        let mut entities  = vec![core::ptr::null_mut::<c_void>(); max_hits];
        let mut positions = vec![0f32; max_hits * 3];
        let mut normals   = vec![0f32; max_hits * 3];
        let mut distances = vec![0f32; max_hits];

        let n = unsafe {
            realitykit_sys::rk_scene_raycast(
                self.ptr,
                origin[0], origin[1], origin[2],
                direction[0], direction[1], direction[2],
                length, query_mode,
                entities.as_mut_ptr()  as *mut *mut c_void,
                positions.as_mut_ptr(),
                normals.as_mut_ptr(),
                distances.as_mut_ptr(),
                max_hits,
            )
        };

        (0..n).map(|i| RaycastHit {
            entity:   Entity { ptr: entities[i] },
            position: [positions[i*3], positions[i*3+1], positions[i*3+2]],
            normal:   [normals[i*3],   normals[i*3+1],   normals[i*3+2]],
            distance: distances[i],
        }).collect()
    }

    /// Cast a convex shape through the scene. Returns up to `max_hits` results.
    ///
    /// `shape` — a `ShapeResource` handle (box, sphere, capsule, etc.).
    pub fn convex_cast(
        &self,
        from:     [f32; 3],
        to:       [f32; 3],
        shape:    *mut c_void,
        max_hits: usize,
    ) -> Vec<RaycastHit> {
        if self.ptr.is_null() || max_hits == 0 { return Vec::new(); }
        let mut entities  = vec![core::ptr::null_mut::<c_void>(); max_hits];
        let mut positions = vec![0f32; max_hits * 3];
        let mut normals   = vec![0f32; max_hits * 3];
        let mut distances = vec![0f32; max_hits];

        let n = unsafe {
            realitykit_sys::rk_scene_convex_cast(
                self.ptr,
                from[0], from[1], from[2],
                to[0],   to[1],   to[2],
                shape,
                entities.as_mut_ptr()  as *mut *mut c_void,
                positions.as_mut_ptr(),
                normals.as_mut_ptr(),
                distances.as_mut_ptr(),
                max_hits,
            )
        };

        (0..n).map(|i| RaycastHit {
            entity:   Entity { ptr: entities[i] },
            position: [positions[i*3], positions[i*3+1], positions[i*3+2]],
            normal:   [normals[i*3],   normals[i*3+1],   normals[i*3+2]],
            distance: distances[i],
        }).collect()
    }

    // ── Entity queries ────────────────────────────────────────────────────

    /// Return all entities in the scene that have a `ModelComponent`.
    pub fn query_model_entities(&self, max: usize)     -> Vec<Entity> { self.query_by_id(0, max) }
    /// Return all entities that have a `CollisionComponent`.
    pub fn query_collision_entities(&self, max: usize) -> Vec<Entity> { self.query_by_id(1, max) }
    /// Return all entities that have a `PhysicsBodyComponent`.
    pub fn query_physics_entities(&self, max: usize)   -> Vec<Entity> { self.query_by_id(2, max) }

    fn query_by_id(&self, component_id: i32, max: usize) -> Vec<Entity> {
        if self.ptr.is_null() || max == 0 { return Vec::new(); }
        let mut out = vec![core::ptr::null_mut::<c_void>(); max];
        let n = unsafe {
            realitykit_sys::rk_scene_query_entities(
                self.ptr, component_id,
                out.as_mut_ptr() as *mut *mut c_void, max,
            )
        };
        out[..n].iter().map(|&p| Entity { ptr: p }).collect()
    }

    // ── Event subscriptions ───────────────────────────────────────────────

    /// Subscribe to per-frame scene update events.
    ///
    /// The closure receives the delta time (seconds since last frame).
    /// Returns a [`Subscription`] — drop it to unsubscribe.
    ///
    /// **Thread safety**: the callback is invoked on RealityKit's render thread.
    /// Ensure any shared state is properly synchronized.
    pub fn on_update<F>(&self, callback: F) -> Subscription
    where F: Fn(f32) + Send + 'static
    {
        let boxed: Box<dyn Fn(f32) + Send + 'static> = Box::new(callback);
        let ud = Box::into_raw(Box::new(boxed)) as *mut c_void;
        let h = unsafe {
            realitykit_sys::rk_scene_subscribe_update(self.ptr, scene_update_trampoline, ud)
        };
        Subscription { handle: h, userdata: ud, drop_ud: drop_update_ud }
    }

    /// Subscribe to collision-began events between two entities.
    ///
    /// Callback receives `(entity_a, entity_b, impulse)`.
    pub fn on_collision_began<F>(&self, callback: F) -> Subscription
    where F: Fn(Entity, Entity, f32) + Send + 'static
    {
        let boxed: Box<dyn Fn(Entity, Entity, f32) + Send + 'static> = Box::new(callback);
        let ud = Box::into_raw(Box::new(boxed)) as *mut c_void;
        let h = unsafe {
            realitykit_sys::rk_scene_subscribe_collision_began(self.ptr, collision_began_trampoline, ud)
        };
        Subscription { handle: h, userdata: ud, drop_ud: drop_collision_began_ud }
    }

    /// Subscribe to collision-ended events.
    pub fn on_collision_ended<F>(&self, callback: F) -> Subscription
    where F: Fn(Entity, Entity) + Send + 'static
    {
        let boxed: Box<dyn Fn(Entity, Entity) + Send + 'static> = Box::new(callback);
        let ud = Box::into_raw(Box::new(boxed)) as *mut c_void;
        let h = unsafe {
            realitykit_sys::rk_scene_subscribe_collision_ended(self.ptr, collision_ended_trampoline, ud)
        };
        Subscription { handle: h, userdata: ud, drop_ud: drop_collision_ended_ud }
    }

    /// Subscribe to animation-playback-completed events.
    ///
    /// Callback receives an `Option<Entity>` (the entity whose animation completed,
    /// if available).
    pub fn on_animation_completed<F>(&self, callback: F) -> Subscription
    where F: Fn(Option<Entity>) + Send + 'static
    {
        let boxed: Box<dyn Fn(Option<Entity>) + Send + 'static> = Box::new(callback);
        let ud = Box::into_raw(Box::new(boxed)) as *mut c_void;
        let h = unsafe {
            realitykit_sys::rk_scene_subscribe_animation_completed(self.ptr, anim_completed_trampoline, ud)
        };
        Subscription { handle: h, userdata: ud, drop_ud: drop_opt_entity_ud }
    }

    /// Subscribe to audio-playback-completed events.
    pub fn on_audio_completed<F>(&self, callback: F) -> Subscription
    where F: Fn(Option<Entity>) + Send + 'static
    {
        let boxed: Box<dyn Fn(Option<Entity>) + Send + 'static> = Box::new(callback);
        let ud = Box::into_raw(Box::new(boxed)) as *mut c_void;
        let h = unsafe {
            realitykit_sys::rk_scene_subscribe_audio_completed(self.ptr, audio_completed_trampoline, ud)
        };
        Subscription { handle: h, userdata: ud, drop_ud: drop_opt_entity_ud }
    }
}

// ─── RaycastHit ──────────────────────────────────────────────────────────────

/// A single result from [`Scene::raycast`] or [`Scene::convex_cast`].
pub struct RaycastHit {
    /// The entity that was hit.
    pub entity:   Entity,
    /// World-space position of the intersection point.
    pub position: [f32; 3],
    /// World-space surface normal at the intersection point.
    pub normal:   [f32; 3],
    /// Distance from the ray origin to the hit point.
    pub distance: f32,
}

// ─── Subscription ─────────────────────────────────────────────────────────────

/// An active event subscription returned by `Scene::on_*` methods.
///
/// The subscription is **automatically cancelled** when the `Subscription` is
/// dropped.  Keep it alive for as long as you need the callback to fire.
///
/// ```ignore
/// let _update_sub = scene.on_update(|dt| println!("frame dt={dt}"));
/// // subscription cancelled when _update_sub goes out of scope
/// ```
pub struct Subscription {
    handle:   *mut c_void,
    userdata: *mut c_void,
    drop_ud:  unsafe fn(*mut c_void),
}
unsafe impl Send for Subscription {}
unsafe impl Sync for Subscription {}

impl Drop for Subscription {
    fn drop(&mut self) {
        // Releasing the handle cancels the Swift RKEventSubscription (via deinit → token.cancel())
        if !self.handle.is_null() {
            unsafe { realitykit_sys::rk_release(self.handle) };
        }
        // Free the Rust closure box
        unsafe { (self.drop_ud)(self.userdata) };
    }
}

// ─── Trampolines ─────────────────────────────────────────────────────────────

unsafe extern "C" fn scene_update_trampoline(dt: f32, ud: *mut c_void) {
    let cb = unsafe { &*(ud as *const Box<dyn Fn(f32) + Send + 'static>) };
    cb(dt);
}

unsafe extern "C" fn collision_began_trampoline(a: *mut c_void, b: *mut c_void, impulse: f32, ud: *mut c_void) {
    let cb = unsafe { &*(ud as *const Box<dyn Fn(Entity, Entity, f32) + Send + 'static>) };
    // Entities arrive with an extra retain from Swift — we adopt them here
    cb(Entity { ptr: a }, Entity { ptr: b }, impulse);
}

unsafe extern "C" fn collision_ended_trampoline(a: *mut c_void, b: *mut c_void, ud: *mut c_void) {
    let cb = unsafe { &*(ud as *const Box<dyn Fn(Entity, Entity) + Send + 'static>) };
    cb(Entity { ptr: a }, Entity { ptr: b });
}

unsafe extern "C" fn anim_completed_trampoline(entity_or_null: *mut c_void, ud: *mut c_void) {
    let cb = unsafe { &*(ud as *const Box<dyn Fn(Option<Entity>) + Send + 'static>) };
    let e = if entity_or_null.is_null() { None } else { Some(Entity { ptr: entity_or_null }) };
    cb(e);
}

unsafe extern "C" fn audio_completed_trampoline(entity_or_null: *mut c_void, ud: *mut c_void) {
    let cb = unsafe { &*(ud as *const Box<dyn Fn(Option<Entity>) + Send + 'static>) };
    let e = if entity_or_null.is_null() { None } else { Some(Entity { ptr: entity_or_null }) };
    cb(e);
}

unsafe fn drop_update_ud(ud: *mut c_void) {
    unsafe { drop(Box::from_raw(ud as *mut Box<dyn Fn(f32) + Send + 'static>)) };
}
unsafe fn drop_collision_began_ud(ud: *mut c_void) {
    unsafe { drop(Box::from_raw(ud as *mut Box<dyn Fn(Entity, Entity, f32) + Send + 'static>)) };
}
unsafe fn drop_collision_ended_ud(ud: *mut c_void) {
    unsafe { drop(Box::from_raw(ud as *mut Box<dyn Fn(Entity, Entity) + Send + 'static>)) };
}
unsafe fn drop_opt_entity_ud(ud: *mut c_void) {
    unsafe { drop(Box::from_raw(ud as *mut Box<dyn Fn(Option<Entity>) + Send + 'static>)) };
}
