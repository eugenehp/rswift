//! Pure-Rust unit tests for realitykit types that don't require the Swift bridge.

#[cfg(test)]
mod physics_tests {
    use crate::physics::{PhysicsBodyMode, CollisionMode};

    #[test]
    fn physics_body_mode_repr() {
        assert_eq!(PhysicsBodyMode::Static   as i32, 0);
        assert_eq!(PhysicsBodyMode::Dynamic  as i32, 1);
        assert_eq!(PhysicsBodyMode::Kinematic as i32, 2);
    }

    #[test]
    fn collision_mode_repr() {
        assert_eq!(CollisionMode::Default as i32, 0);
        assert_eq!(CollisionMode::Trigger as i32, 1);
    }

    #[test]
    fn physics_body_mode_debug() {
        assert_eq!(format!("{:?}", PhysicsBodyMode::Dynamic), "Dynamic");
    }
}

#[cfg(test)]
mod animation_tests {
    use crate::animation::TimingFunction;

    #[test]
    fn timing_function_repr() {
        assert_eq!(TimingFunction::Linear    as i32, 0);
        assert_eq!(TimingFunction::EaseIn    as i32, 1);
        assert_eq!(TimingFunction::EaseOut   as i32, 2);
        assert_eq!(TimingFunction::EaseInOut as i32, 3);
    }

    #[test]
    fn timing_function_copy() {
        let t = TimingFunction::EaseInOut;
        let t2 = t;  // Copy
        assert_eq!(t, t2);
    }
}

#[cfg(test)]
mod audio_tests {
    use crate::audio::AudioInputMode;

    #[test]
    fn audio_input_mode_repr() {
        assert_eq!(AudioInputMode::Spatial    as i32, 0);
        assert_eq!(AudioInputMode::NonSpatial as i32, 1);
    }
}

#[cfg(test)]
mod scene_tests {
    use crate::scene::RenderOptions;

    #[test]
    fn render_options_bits() {
        assert_eq!(RenderOptions::NONE.0, 0);
        assert_eq!(RenderOptions::DISABLE_MOTION_BLUR.0, 1);
        assert_eq!(RenderOptions::DISABLE_HDR.0, 4);
        assert_eq!(RenderOptions::DISABLE_CAMERA_GRAIN.0, 64);
    }

    #[test]
    fn render_options_bitor() {
        let opts = RenderOptions::DISABLE_HDR | RenderOptions::DISABLE_MOTION_BLUR;
        assert_eq!(opts.0, 5);
    }

    #[test]
    fn render_options_none_is_zero() {
        assert_eq!(RenderOptions::NONE | RenderOptions::DISABLE_DEPTH_OF_FIELD,
                   RenderOptions::DISABLE_DEPTH_OF_FIELD);
    }
}

#[cfg(test)]
mod particle_tests {
    use crate::particle::ParticleEmitterParams;

    #[test]
    fn params_defaults() {
        let p = ParticleEmitterParams::new();
        assert_eq!(p.birth_rate, 50.0);
        assert_eq!(p.speed,      1.0);
        assert_eq!(p.lifetime,   2.0);
        assert_eq!(p.size,       0.05);
        assert_eq!(p.color,      [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn params_builder() {
        let p = ParticleEmitterParams::new()
            .birth_rate(100.0)
            .speed(2.5)
            .lifetime(3.0)
            .size(0.1)
            .color(1.0, 0.5, 0.0, 0.8);
        assert_eq!(p.birth_rate, 100.0);
        assert_eq!(p.speed,      2.5);
        assert_eq!(p.lifetime,   3.0);
        assert_eq!(p.size,       0.1);
        assert_eq!(p.color,      [1.0, 0.5, 0.0, 0.8]);
    }

    #[test]
    fn params_default_trait() {
        let p: ParticleEmitterParams = Default::default();
        assert_eq!(p.birth_rate, 50.0);
    }

    #[test]
    fn params_is_copy() {
        let p = ParticleEmitterParams::new().birth_rate(99.0);
        let p2 = p;    // Copy
        assert_eq!(p.birth_rate, p2.birth_rate);
    }
}

#[cfg(test)]
mod anchor_type_tests {
    use crate::entity::{AnchorType, PlaneType};

    #[test]
    fn plane_type_debug() {
        assert_eq!(format!("{:?}", PlaneType::Horizontal), "Horizontal");
        assert_eq!(format!("{:?}", PlaneType::Vertical),   "Vertical");
        assert_eq!(format!("{:?}", PlaneType::Any),        "Any");
    }

    #[test]
    fn anchor_type_image_clone() {
        let a = AnchorType::Image {
            group: "MyGroup".into(),
            name:  "poster".into(),
        };
        let b = a.clone();
        if let AnchorType::Image { group, name } = b {
            assert_eq!(group, "MyGroup");
            assert_eq!(name,  "poster");
        } else {
            panic!("clone changed variant");
        }
    }
}
