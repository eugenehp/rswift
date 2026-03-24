//! Apple SpriteKit — 2D game engine from Rust.
//!
//! **Platform:** macOS 10.9+, iOS 7+, tvOS 9+, watchOS 3+.
//!
//! ```ignore
//! let scene = spritekit::Scene::with_size(800.0, 600.0);
//! let sprite = spritekit::SpriteNode::with_color(1.0, 0.0, 0.0, 1.0, 50.0, 50.0);
//! sprite.set_position(400.0, 300.0);
//! scene.add_child(&sprite);
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


// ── Scene ───────────────────────────────────────────────────────────────────

pub struct Scene { inner: Id }

impl Scene {
    pub fn with_size(width: f64, height: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"sceneWithSize:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f64;2]) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let s = f(class!(b"SKScene\0") as Id, sel, [width, height]);
            CFRetain(s as CFTypeRef);
            Self { inner: s }
        }
    }

    pub fn add_child(&self, node: &impl AsNode) {
        unsafe { msg_send_void![self.inner, addChildNode: node.as_node_ptr()]; }
    }

    /// Set the scale mode (0=fill, 1=aspectFill, 2=aspectFit, 3=resizeFill).
    pub fn set_scale_mode(&self, mode: isize) {
        unsafe {
            let sel = sel_registerName(b"setScaleMode:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, mode);
        }
    }

    /// Set background color (RGBA).
    pub fn set_background_color(&self, r: f64, g: f64, b: f64, a: f64) {
        unsafe {
            let sel = sel_registerName(b"colorWithRed:green:blue:alpha:\0".as_ptr());
            let cf: unsafe extern "C" fn(Id, Sel, f64, f64, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            #[cfg(target_os = "macos")]
            let color = cf(class!(b"NSColor\0") as Id, sel, r, g, b, a);
            #[cfg(not(target_os = "macos"))]
            let color = cf(class!(b"UIColor\0") as Id, sel, r, g, b, a);
            msg_send_void![self.inner, setBackgroundColor: color];
        }
    }

    /// Enable/disable gravity. Default gravity is (0, -9.8).
    pub fn set_gravity(&self, dx: f32, dy: f32) {
        unsafe {
            let world: Id = msg_send![self.inner, physicsWorld];
            let sel = sel_registerName(b"setGravity:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f32;2]) =
                core::mem::transmute(objc_msgSend as *const ());
            f(world, sel, [dx, dy]);
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Scene { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Trait for types that can act as scene nodes.
pub trait AsNode { fn as_node_ptr(&self) -> Id; }

// ── SpriteNode ──────────────────────────────────────────────────────────────

pub struct SpriteNode { inner: Id }

impl SpriteNode {
    /// Create a colored rectangle sprite.
    pub fn with_color(r: f64, g: f64, b: f64, a: f64, width: f64, height: f64) -> Self {
        unsafe {
            let sel_c = sel_registerName(b"colorWithRed:green:blue:alpha:\0".as_ptr());
            let cf: unsafe extern "C" fn(Id, Sel, f64, f64, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            #[cfg(target_os = "macos")]
            let color = cf(class!(b"NSColor\0") as Id, sel_c, r, g, b, a);
            #[cfg(not(target_os = "macos"))]
            let color = cf(class!(b"UIColor\0") as Id, sel_c, r, g, b, a);

            let sel = sel_registerName(b"spriteNodeWithColor:size:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, [f64;2]) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let s = f(class!(b"SKSpriteNode\0") as Id, sel, color, [width, height]);
            CFRetain(s as CFTypeRef);
            Self { inner: s }
        }
    }

    /// Create from an image file name (in bundle).
    pub fn with_image(named: &str) -> Self {
        unsafe {
            let ns = nsstring(named);
            let s: Id = msg_send![class!(b"SKSpriteNode\0"), spriteNodeWithImageNamed: ns];
            CFRelease(ns as CFTypeRef);
            CFRetain(s as CFTypeRef);
            Self { inner: s }
        }
    }

    pub fn set_position(&self, x: f64, y: f64) {
        unsafe {
            let sel = sel_registerName(b"setPosition:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f64;2]) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, [x, y]);
        }
    }

    pub fn set_z_rotation(&self, radians: f64) {
        unsafe {
            let sel = sel_registerName(b"setZRotation:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, radians);
        }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
            let ns = nsstring(name);
            msg_send_void![self.inner, setName: ns];
            CFRelease(ns as CFTypeRef);
        }
    }

    pub fn set_alpha(&self, alpha: f64) {
        unsafe {
            let sel = sel_registerName(b"setAlpha:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, alpha);
        }
    }

    /// Add a circular physics body.
    pub fn set_physics_circle(&self, radius: f64) {
        unsafe {
            let sel = sel_registerName(b"bodyWithCircleOfRadius:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let body = f(class!(b"SKPhysicsBody\0") as Id, sel, radius);
            msg_send_void![self.inner, setPhysicsBody: body];
        }
    }

    /// Add a rectangular physics body matching the sprite size.
    pub fn set_physics_rect(&self, width: f64, height: f64) {
        unsafe {
            let sel = sel_registerName(b"bodyWithRectangleOfSize:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f64;2]) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let body = f(class!(b"SKPhysicsBody\0") as Id, sel, [width, height]);
            msg_send_void![self.inner, setPhysicsBody: body];
        }
    }

    pub fn add_child(&self, node: &impl AsNode) {
        unsafe { msg_send_void![self.inner, addChildNode: node.as_node_ptr()]; }
    }

    pub fn remove_from_parent(&self) {
        unsafe { msg_send_void![self.inner, removeFromParent]; }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl AsNode for SpriteNode { fn as_node_ptr(&self) -> Id { self.inner } }
impl Drop for SpriteNode { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

// ── LabelNode ───────────────────────────────────────────────────────────────

pub struct LabelNode { inner: Id }

impl LabelNode {
    pub fn new(text: &str) -> Self {
        unsafe {
            let ns = nsstring(text);
            let l: Id = msg_send![class!(b"SKLabelNode\0"), labelNodeWithText: ns];
            CFRelease(ns as CFTypeRef);
            CFRetain(l as CFTypeRef);
            Self { inner: l }
        }
    }

    pub fn set_text(&self, text: &str) {
        unsafe {
            let ns = nsstring(text);
            msg_send_void![self.inner, setText: ns];
            CFRelease(ns as CFTypeRef);
        }
    }

    pub fn set_font_size(&self, size: f64) {
        unsafe {
            let sel = sel_registerName(b"setFontSize:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, size);
        }
    }

    pub fn set_font_name(&self, name: &str) {
        unsafe {
            let ns = nsstring(name);
            msg_send_void![self.inner, setFontName: ns];
            CFRelease(ns as CFTypeRef);
        }
    }

    pub fn set_position(&self, x: f64, y: f64) {
        unsafe {
            let sel = sel_registerName(b"setPosition:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f64;2]) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, [x, y]);
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl AsNode for LabelNode { fn as_node_ptr(&self) -> Id { self.inner } }
impl Drop for LabelNode { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

// ── ShapeNode ───────────────────────────────────────────────────────────────

pub struct ShapeNode { inner: Id }

impl ShapeNode {
    /// Create a circle shape.
    pub fn circle(radius: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"shapeNodeWithCircleOfRadius:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let s = f(class!(b"SKShapeNode\0") as Id, sel, radius);
            CFRetain(s as CFTypeRef);
            Self { inner: s }
        }
    }

    /// Create a rectangle shape.
    pub fn rect(width: f64, height: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"shapeNodeWithRectOfSize:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f64;2]) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let s = f(class!(b"SKShapeNode\0") as Id, sel, [width, height]);
            CFRetain(s as CFTypeRef);
            Self { inner: s }
        }
    }

    pub fn set_fill_color(&self, r: f64, g: f64, b: f64, a: f64) {
        unsafe {
            let sel_c = sel_registerName(b"colorWithRed:green:blue:alpha:\0".as_ptr());
            let cf: unsafe extern "C" fn(Id, Sel, f64, f64, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            #[cfg(target_os = "macos")]
            let color = cf(class!(b"NSColor\0") as Id, sel_c, r, g, b, a);
            #[cfg(not(target_os = "macos"))]
            let color = cf(class!(b"UIColor\0") as Id, sel_c, r, g, b, a);
            msg_send_void![self.inner, setFillColor: color];
        }
    }

    pub fn set_stroke_color(&self, r: f64, g: f64, b: f64, a: f64) {
        unsafe {
            let sel_c = sel_registerName(b"colorWithRed:green:blue:alpha:\0".as_ptr());
            let cf: unsafe extern "C" fn(Id, Sel, f64, f64, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            #[cfg(target_os = "macos")]
            let color = cf(class!(b"NSColor\0") as Id, sel_c, r, g, b, a);
            #[cfg(not(target_os = "macos"))]
            let color = cf(class!(b"UIColor\0") as Id, sel_c, r, g, b, a);
            msg_send_void![self.inner, setStrokeColor: color];
        }
    }

    pub fn set_line_width(&self, width: f64) {
        unsafe {
            let sel = sel_registerName(b"setLineWidth:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, width);
        }
    }

    pub fn set_position(&self, x: f64, y: f64) {
        unsafe {
            let sel = sel_registerName(b"setPosition:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f64;2]) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, [x, y]);
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl AsNode for ShapeNode { fn as_node_ptr(&self) -> Id { self.inner } }
impl Drop for ShapeNode { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

// ── Actions ─────────────────────────────────────────────────────────────────

/// Pre-built animation actions.
pub mod actions {
    use super::*;

    /// Create a move-to action.
    pub fn move_to(x: f64, y: f64, duration: f64) -> Id {
        unsafe {
            let sel = sel_registerName(b"moveToX:y:duration:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            f(class!(b"SKAction\0") as Id, sel, x, y, duration)
        }
    }

    /// Create a rotate-by action (radians).
    pub fn rotate_by(radians: f64, duration: f64) -> Id {
        unsafe {
            let sel = sel_registerName(b"rotateByAngle:duration:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            f(class!(b"SKAction\0") as Id, sel, radians, duration)
        }
    }

    /// Create a scale-to action.
    pub fn scale_to(scale: f64, duration: f64) -> Id {
        unsafe {
            let sel = sel_registerName(b"scaleTo:duration:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            f(class!(b"SKAction\0") as Id, sel, scale, duration)
        }
    }

    /// Create a fade-out action.
    pub fn fade_out(duration: f64) -> Id {
        unsafe {
            let sel = sel_registerName(b"fadeOutWithDuration:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            f(class!(b"SKAction\0") as Id, sel, duration)
        }
    }

    /// Create a fade-in action.
    pub fn fade_in(duration: f64) -> Id {
        unsafe {
            let sel = sel_registerName(b"fadeInWithDuration:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            f(class!(b"SKAction\0") as Id, sel, duration)
        }
    }

    /// Create a wait action.
    pub fn wait(duration: f64) -> Id {
        unsafe {
            let sel = sel_registerName(b"waitForDuration:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            f(class!(b"SKAction\0") as Id, sel, duration)
        }
    }

    /// Create a remove-from-parent action.
    pub fn remove_from_parent() -> Id {
        unsafe { msg_send![class!(b"SKAction\0"), removeFromParent] }
    }

    /// Sequence multiple actions.
    pub fn sequence(actions: &[Id]) -> Id {
        unsafe {
            let sel = sel_registerName(b"arrayWithObjects:count:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, *const Id, usize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let arr = f(class!(b"NSArray\0") as Id, sel, actions.as_ptr(), actions.len());
            msg_send![class!(b"SKAction\0"), sequence: arr]
        }
    }

    /// Repeat an action forever.
    pub fn repeat_forever(action: Id) -> Id {
        unsafe { msg_send![class!(b"SKAction\0"), repeatActionForever: action] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene() {
        let scene = Scene::with_size(800.0, 600.0);
        scene.set_gravity(0.0, -9.8);
    }

    #[test]
    fn test_label() {
        let label = LabelNode::new("Hello");
        label.set_font_size(24.0);
        label.set_position(100.0, 200.0);
    }

    #[test]
    fn test_shape() {
        let circle = ShapeNode::circle(50.0);
        circle.set_fill_color(1.0, 0.0, 0.0, 1.0);
        circle.set_line_width(2.0);
    }
}
