//! visionOS-specific components: InputTargetComponent, HoverEffectComponent,
//! PortalComponent, WorldComponent, SurroundingsEffectComponent,
//! and visionOS-only anchor types (head, left/right hand).
//!
//! Enable with the `visionos` Cargo feature or target `xros`.

use crate::Entity;

impl Entity {
    // ── Interaction ───────────────────────────────────────────────────────

    /// Attach `InputTargetComponent` so the entity can receive tap / pinch events.
    ///
    /// - `direct`   — allow direct-touch (fingertip) interaction
    /// - `indirect` — allow indirect (look + pinch) interaction
    pub fn set_input_target(&self, direct: bool, indirect: bool) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_input_target(self.ptr, direct, indirect) }; self
    }
    pub fn remove_input_target(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_input_target(self.ptr) }; self
    }

    // ── Hover highlight ───────────────────────────────────────────────────

    /// Attach `HoverEffectComponent` with a highlight colour.
    /// On visionOS 1, colour is ignored (plain default hover effect used).
    /// On visionOS 2+ the highlight colour and strength are applied.
    ///
    /// `strength` is 0–1.
    pub fn set_hover_effect(&self, r: f32, g: f32, b: f32, strength: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_hover_effect(self.ptr, r, g, b, strength) }; self
    }
    pub fn remove_hover_effect(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_hover_effect(self.ptr) }; self
    }

    // ── Portal / World ────────────────────────────────────────────────────

    /// Mark this entity as a "world" — its children live in an isolated space
    /// rendered through portals.
    pub fn set_world_component(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_world_component(self.ptr) }; self
    }
    pub fn remove_world_component(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_world_component(self.ptr) }; self
    }

    // ── Passthrough tinting (visionOS 2+) ─────────────────────────────────

    /// Dim the user's passthrough view by `intensity` (0 = off, 1 = maximum dimming).
    /// Requires visionOS 2+; is a no-op on visionOS 1.
    pub fn set_surroundings_effect(&self, intensity: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_surroundings_effect(self.ptr, intensity) }; self
    }
    pub fn remove_surroundings_effect(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_surroundings_effect(self.ptr) }; self
    }
}
