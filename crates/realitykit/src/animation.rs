//! Animation playback controller, timing functions, and animation builders.


use core::ffi::c_void;

/// Controls a running animation.  Obtained from `Entity::move_to` or
/// `Entity::play_animation`.
pub struct AnimationController {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for AnimationController {}
unsafe impl Sync for AnimationController {}
impl Drop for AnimationController {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

impl AnimationController {
    pub fn pause(&self)  { unsafe { realitykit_sys::rk_animation_controller_pause(self.ptr) }; }
    pub fn resume(&self) { unsafe { realitykit_sys::rk_animation_controller_resume(self.ptr) }; }
    pub fn stop(&self)   { unsafe { realitykit_sys::rk_animation_controller_stop(self.ptr) }; }
    pub fn is_playing(&self) -> bool { unsafe { realitykit_sys::rk_animation_controller_is_playing(self.ptr) } }
    pub fn is_paused(&self)  -> bool { unsafe { realitykit_sys::rk_animation_controller_is_paused(self.ptr) } }
    /// Returns `true` when the animation has completed (not playing and not paused).
    pub fn is_complete(&self) -> bool { !self.is_playing() && !self.is_paused() }
}

/// Easing curve for `Entity::move_to` and similar timed transitions.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimingFunction {
    Linear    = 0,
    EaseIn    = 1,
    EaseOut   = 2,
    EaseInOut = 3,
}

/// How an animation repeats after it completes.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AnimationRepeatMode {
    /// Play once then stop.
    #[default]
    None        = 0,
    /// Loop from the beginning indefinitely.
    Repeat      = 1,
    /// Accumulate the end value on each loop (useful for orbit animations).
    Cumulative  = 2,
    /// Play forward then backward alternately.
    AutoReverse = 3,
}

/// An opaque handle to a compiled `AnimationResource` or an `AnimationDefinition`
/// that can be used with [`AnimationDefinition`] builders and
/// [`Entity::play_animation_resource`].
pub struct AnimationResource {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for AnimationResource {}
unsafe impl Sync for AnimationResource {}
impl Clone for AnimationResource {
    fn clone(&self) -> Self { unsafe { realitykit_sys::rk_retain(self.ptr) }; AnimationResource { ptr: self.ptr } }
}
impl Drop for AnimationResource {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

/// An opaque handle to an uncompiled `AnimationDefinition`
/// (e.g. `OrbitAnimation`, `FromToByAnimation<Transform>`).
/// Compile it via [`AnimationDefinition::build`] or combine multiple
/// definitions into a group via [`AnimationResource::group`].
pub struct AnimationDefinition {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for AnimationDefinition {}
unsafe impl Sync for AnimationDefinition {}
impl Clone for AnimationDefinition {
    fn clone(&self) -> Self { unsafe { realitykit_sys::rk_retain(self.ptr) }; AnimationDefinition { ptr: self.ptr } }
}
impl Drop for AnimationDefinition {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

impl AnimationDefinition {
    /// Compile this definition into an `AnimationResource` ready for playback.
    pub fn build(self) -> Result<AnimationResource, String> {
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_animation_resource_from_def(self.ptr, err.as_mut_ptr(), err.len())
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(AnimationResource { ptr: p })
        }
    }

    // ── Definition constructors ───────────────────────────────────────────

    /// `OrbitAnimation` — rotates the entity around `axis` by `rotation_count`
    /// full revolutions over `duration` seconds.
    ///
    /// # Example
    /// ```ignore
    /// let def = AnimationDefinition::orbit([0., 1., 0.], 1.0, true, 2.0, AnimationRepeatMode::Repeat, true);
    /// let resource = def.build()?;
    /// entity.play_animation_resource(&resource, 0.0, false);
    /// ```
    pub fn orbit(
        axis: [f32; 3],
        rotation_count: f32,
        clockwise: bool,
        duration: f64,
        repeat_mode: AnimationRepeatMode,
        is_additive: bool,
    ) -> Self {
        AnimationDefinition {
            ptr: unsafe {
                realitykit_sys::rk_animdef_orbit(
                    axis[0], axis[1], axis[2],
                    rotation_count, clockwise, false,
                    duration, repeat_mode as i32, is_additive,
                )
            },
        }
    }

    /// `FromToByAnimation<Transform>` — animates between two explicit transforms.
    ///
    /// `from` / `to`: `[tx, ty, tz, rx, ry, rz, rw, sx, sy, sz]`
    /// (position, quaternion rotation, scale).
    pub fn from_to_transform(
        from: [f32; 10],
        to:   [f32; 10],
        duration: f64,
        timing: TimingFunction,
        repeat_mode: AnimationRepeatMode,
        is_additive: bool,
    ) -> Self {
        AnimationDefinition {
            ptr: unsafe {
                realitykit_sys::rk_animdef_from_to_transform(
                    from[0], from[1], from[2],
                    from[3], from[4], from[5], from[6],
                    from[7], from[8], from[9],
                    to[0], to[1], to[2],
                    to[3], to[4], to[5], to[6],
                    to[7], to[8], to[9],
                    duration, timing as i32, repeat_mode as i32, is_additive,
                )
            },
        }
    }
}

impl AnimationResource {
    /// Compile multiple [`AnimationDefinition`]s into a parallel `AnimationGroup`
    /// resource that plays all animations simultaneously.
    ///
    /// `speed` — playback rate multiplier (1.0 = normal).
    pub fn group(defs: &[AnimationDefinition], speed: f32) -> Result<Self, String> {
        let handles: Vec<*mut c_void> = defs.iter().map(|d| d.ptr).collect();
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_animation_resource_group(
                handles.as_ptr() as *const _,
                handles.len(),
                speed,
                err.as_mut_ptr(), err.len(),
            )
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(AnimationResource { ptr: p })
        }
    }
}
