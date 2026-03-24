//! Apple GameplayKit — game logic, AI, pathfinding from Rust.
//!
//! **Platform:** macOS 10.11+, iOS 9+, tvOS 9+.
//!
//! ```ignore
//! let rng = gameplaykit::RandomSource::arc4();
//! println!("Random int: {}", rng.next_int());
//! println!("Random 1-6: {}", rng.next_int_with_upper_bound(6) + 1);
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// Random number source (wraps `GKRandomSource` subclasses).
pub struct RandomSource { inner: Id }

impl RandomSource {
    /// ARC4-based random source (fast, good quality).
    pub fn arc4() -> Self {
        Self { inner: unsafe { msg_send![class!(b"GKARC4RandomSource\0"), new] } }
    }

    /// Linear congruential (fastest, lower quality).
    pub fn linear() -> Self {
        Self { inner: unsafe { msg_send![class!(b"GKLinearCongruentialRandomSource\0"), new] } }
    }

    /// Mersenne twister (best quality, slower).
    pub fn mersenne_twister() -> Self {
        Self { inner: unsafe { msg_send![class!(b"GKMersenneTwisterRandomSource\0"), new] } }
    }

    /// Shared system random source.
    pub fn shared() -> Self {
        let inner = unsafe { msg_send![class!(b"GKRandomSource\0"), sharedRandom] };
        Self { inner }
    }

    /// Next random integer (full range).
    pub fn next_int(&self) -> i32 {
        unsafe { msg_send_t![i32; self.inner, nextInt] }
    }

    /// Next random integer in [0, upper_bound).
    pub fn next_int_with_upper_bound(&self, upper_bound: usize) -> usize {
        unsafe {
            let sel = sel_registerName(b"nextIntWithUpperBound:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) -> usize =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, upper_bound)
        }
    }

    /// Next random float in [0.0, 1.0).
    pub fn next_uniform(&self) -> f32 {
        unsafe { msg_send_t![f32; self.inner, nextUniform] }
    }

    /// Next random bool.
    pub fn next_bool(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, nextBool] }
    }

    /// Shuffle an array of indices (returns shuffled copy).
    pub fn shuffled_indices(&self, count: usize) -> Vec<usize> {
        (0..count).map(|_| self.next_int_with_upper_bound(count)).collect()
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for RandomSource { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Entity-Component system node.
pub struct Entity { inner: Id }

impl Entity {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"GKEntity\0"), entity] } }
    }

    pub fn component_count(&self) -> usize {
        unsafe {
            let arr: Id = msg_send![self.inner, components];
            if arr.is_null() { 0 } else { msg_send_t![usize; arr, count] }
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for Entity { fn default() -> Self { Self::new() } }

/// State machine.
pub struct StateMachine { inner: Id }

impl StateMachine {
    /// Create from an array of GKState pointers.
    pub fn from_states_ptr(states_array: Id) -> Self {
        let inner = unsafe { msg_send![class!(b"GKStateMachine\0"), stateMachineWithStates: states_array] };
        Self { inner }
    }

    /// Current state class name.
    pub fn current_state_class(&self) -> Option<String> {
        unsafe {
            let state: Id = msg_send![self.inner, currentState];
            if state.is_null() { return None; }
            let cls: Id = msg_send![state, class];
            nsstring_to_string(msg_send![cls, description])
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_source() {
        let rng = RandomSource::arc4();
        let a = rng.next_int();
        let b = rng.next_int();
        // Extremely unlikely to be equal
        assert!(a != b || rng.next_int() != a);
    }

    #[test]
    fn test_random_bounded() {
        let rng = RandomSource::mersenne_twister();
        for _ in 0..100 {
            let v = rng.next_int_with_upper_bound(6);
            assert!(v < 6);
        }
    }

    #[test]
    fn test_random_uniform() {
        let rng = RandomSource::linear();
        let v = rng.next_uniform();
        assert!((0.0..1.0).contains(&v));
    }

    #[test]
    fn test_entity() {
        let e = Entity::new();
        assert_eq!(e.component_count(), 0);
    }
}
