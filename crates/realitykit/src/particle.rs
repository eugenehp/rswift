//! Particle emitter helpers (macOS 14+, iOS 17+, visionOS 1+).

use crate::Entity;

/// Parameters for [`Entity::set_particle_emitter`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParticleEmitterParams {
    /// Particles spawned per second.
    pub birth_rate: f32,
    /// Initial speed in m/s.
    pub speed: f32,
    /// Particle lifespan in seconds.
    pub lifetime: f32,
    /// Particle diameter in metres.
    pub size: f32,
    /// RGBA colour (0–1 each).
    pub color: [f32; 4],
}

impl ParticleEmitterParams {
    /// Sensible defaults: 50/s, 1 m/s, 2 s, 0.05 m, white opaque.
    pub fn new() -> Self {
        Self { birth_rate: 50.0, speed: 1.0, lifetime: 2.0, size: 0.05, color: [1.0, 1.0, 1.0, 1.0] }
    }
    pub fn birth_rate(mut self, v: f32) -> Self { self.birth_rate = v; self }
    pub fn speed(mut self, v: f32) -> Self { self.speed = v; self }
    pub fn lifetime(mut self, v: f32) -> Self { self.lifetime = v; self }
    pub fn size(mut self, v: f32) -> Self { self.size = v; self }
    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self { self.color = [r, g, b, a]; self }
}

impl Default for ParticleEmitterParams {
    fn default() -> Self { Self::new() }
}

impl Entity {
    /// Attach a `ParticleEmitterComponent` (macOS 14+, iOS 17+, visionOS 1+).
    pub fn set_particle_emitter(&self, p: ParticleEmitterParams) -> &Self {
        let [r, g, b, a] = p.color;
        unsafe {
            realitykit_sys::rk_entity_set_particle_emitter(
                self.ptr, p.birth_rate, p.speed, p.lifetime, p.size, r, g, b, a,
            )
        }; self
    }
    pub fn particle_set_birth_rate(&self, rate: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_particle_set_birth_rate(self.ptr, rate) }; self
    }
    pub fn particle_set_speed(&self, speed: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_particle_set_speed(self.ptr, speed) }; self
    }
    pub fn particle_set_color(&self, r: f32, g: f32, b: f32, a: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_particle_set_color(self.ptr, r, g, b, a) }; self
    }
    pub fn particle_set_lifetime(&self, secs: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_particle_set_lifetime(self.ptr, secs) }; self
    }
    pub fn particle_set_size(&self, metres: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_particle_set_size(self.ptr, metres) }; self
    }
    pub fn remove_particle_emitter(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_particle_emitter(self.ptr) }; self
    }
}
