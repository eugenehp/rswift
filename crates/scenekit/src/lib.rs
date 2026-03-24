//! Apple SceneKit — 3D scene rendering from Rust.
//!
//! **Platform:** macOS 10.8+, iOS 8+, tvOS 9+, visionOS 1+, watchOS 3+.
//!
//! ```ignore
//! let scene = scenekit::Scene::new();
//! let box_geo = scenekit::Geometry::box_geo(1.0, 1.0, 1.0, 0.0);
//! let node = scenekit::Node::with_geometry(&box_geo);
//! node.set_position(0.0, 1.0, 0.0);
//! scene.root_node().add_child(&node);
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


// ── Scene ───────────────────────────────────────────────────────────────────

/// A 3D scene (wraps `SCNScene`).
pub struct Scene { inner: Id }

impl Scene {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"SCNScene\0"), scene] } }
    }

    /// Load a scene from a file path.
    pub fn from_file(path: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(path);
            let url: Id = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let s: Id = msg_send![class!(b"SCNScene\0"), sceneWithURL: url, options: NIL, error: NIL];
            if s.is_null() { None } else { CFRetain(s as CFTypeRef); Some(Self { inner: s }) }
        }
    }

    /// The root node of the scene.
    pub fn root_node(&self) -> Node {
        Node { inner: unsafe { msg_send![self.inner, rootNode] }, owned: false }
    }

    /// Set the scene background color (RGBA).
    pub fn set_background_color(&self, r: f64, g: f64, b: f64, a: f64) {
        unsafe {
            let sel = sel_registerName(b"colorWithRed:green:blue:alpha:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            // Use platform-appropriate color class
            #[cfg(target_os = "macos")]
            let color = f(class!(b"NSColor\0") as Id, sel, r, g, b, a);
            #[cfg(not(target_os = "macos"))]
            let color = f(class!(b"UIColor\0") as Id, sel, r, g, b, a);
            let bg: Id = msg_send![self.inner, background];
            msg_send_void![bg, setContents: color];
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for Scene { fn default() -> Self { Self::new() } }
impl Drop for Scene { fn drop(&mut self) { /* SCNScene is autoreleased from +scene */ } }

// ── Node ────────────────────────────────────────────────────────────────────

/// A node in the scene graph (wraps `SCNNode`).
pub struct Node { inner: Id, owned: bool }

impl Node {
    /// Create an empty node.
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"SCNNode\0"), node] }, owned: true }
    }

    /// Create a node with geometry attached.
    pub fn with_geometry(geo: &Geometry) -> Self {
        let inner = unsafe { msg_send![class!(b"SCNNode\0"), nodeWithGeometry: geo.inner] };
        Self { inner, owned: true }
    }

    /// Set position (x, y, z).
    pub fn set_position(&self, x: f32, y: f32, z: f32) {
        unsafe {
            // SCNVector3 is { float x, y, z } on both platforms
            let sel = sel_registerName(b"setPosition:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f32; 3]) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, [x, y, z]);
        }
    }

    /// Get position (x, y, z).
    pub fn position(&self) -> (f32, f32, f32) {
        unsafe {
            let sel = sel_registerName(b"position\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> [f32; 3] =
                core::mem::transmute(objc_msgSend as *const ());
            let p = f(self.inner, sel);
            (p[0], p[1], p[2])
        }
    }

    /// Set Euler rotation angles (radians).
    pub fn set_euler_angles(&self, x: f32, y: f32, z: f32) {
        unsafe {
            let sel = sel_registerName(b"setEulerAngles:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f32; 3]) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, [x, y, z]);
        }
    }

    /// Set scale (x, y, z).
    pub fn set_scale(&self, x: f32, y: f32, z: f32) {
        unsafe {
            let sel = sel_registerName(b"setScale:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, [f32; 3]) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, [x, y, z]);
        }
    }

    /// Set opacity (0.0 – 1.0).
    pub fn set_opacity(&self, opacity: f64) {
        unsafe {
            let sel = sel_registerName(b"setOpacity:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, opacity);
        }
    }

    /// Node name.
    pub fn name(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send![self.inner, name]) }
    }

    /// Set node name.
    pub fn set_name(&self, name: &str) {
        unsafe {
            let ns = nsstring(name);
            msg_send_void![self.inner, setName: ns];
            CFRelease(ns as CFTypeRef);
        }
    }

    /// Add a child node.
    pub fn add_child(&self, child: &Node) {
        unsafe { msg_send_void![self.inner, addChildNode: child.inner]; }
    }

    /// Remove from parent node.
    pub fn remove_from_parent(&self) {
        unsafe { msg_send_void![self.inner, removeFromParentNode]; }
    }

    /// Number of child nodes.
    pub fn child_count(&self) -> usize {
        unsafe {
            let arr: Id = msg_send![self.inner, childNodes];
            msg_send_t![usize; arr, count]
        }
    }

    /// Find child node by name (recursive).
    pub fn child_with_name(&self, name: &str, recursive: bool) -> Option<Node> {
        unsafe {
            let ns = nsstring(name);
            let n: Id = msg_send![self.inner, childNodeWithName: ns, recursively: recursive as u8];
            CFRelease(ns as CFTypeRef);
            if n.is_null() { None } else { Some(Node { inner: n, owned: false }) }
        }
    }

    /// Whether this node is hidden.
    pub fn is_hidden(&self) -> bool { unsafe { msg_send_t![bool; self.inner, isHidden] } }
    pub fn set_hidden(&self, hidden: bool) { unsafe { msg_send_void![self.inner, setHidden: hidden as u8]; } }

    /// Set the geometry for this node.
    pub fn set_geometry(&self, geo: &Geometry) {
        unsafe { msg_send_void![self.inner, setGeometry: geo.inner]; }
    }

    /// Set a light on this node.
    pub fn set_light(&self, light: &Light) {
        unsafe { msg_send_void![self.inner, setLight: light.inner]; }
    }

    /// Set a camera on this node.
    pub fn set_camera(&self, camera: &Camera) {
        unsafe { msg_send_void![self.inner, setCamera: camera.inner]; }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for Node { fn default() -> Self { Self::new() } }
impl Drop for Node {
    fn drop(&mut self) { if self.owned { unsafe { CFRelease(self.inner as CFTypeRef); } } }
}

// ── Geometry ────────────────────────────────────────────────────────────────

/// A 3D geometry (wraps `SCNGeometry` subclasses).
pub struct Geometry { inner: Id }

impl Geometry {
    /// Create a box geometry.
    pub fn box_geo(width: f64, height: f64, length: f64, chamfer: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"boxWithWidth:height:length:chamferRadius:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { inner: f(class!(b"SCNBox\0") as Id, sel, width, height, length, chamfer) }
        }
    }

    /// Create a sphere geometry.
    pub fn sphere(radius: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"sphereWithRadius:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { inner: f(class!(b"SCNSphere\0") as Id, sel, radius) }
        }
    }

    /// Create a plane geometry.
    pub fn plane(width: f64, height: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"planeWithWidth:height:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { inner: f(class!(b"SCNPlane\0") as Id, sel, width, height) }
        }
    }

    /// Create a cylinder.
    pub fn cylinder(radius: f64, height: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"cylinderWithRadius:height:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { inner: f(class!(b"SCNCylinder\0") as Id, sel, radius, height) }
        }
    }

    /// Create a cone.
    pub fn cone(top_radius: f64, bottom_radius: f64, height: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"coneWithTopRadius:bottomRadius:height:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { inner: f(class!(b"SCNCone\0") as Id, sel, top_radius, bottom_radius, height) }
        }
    }

    /// Create a torus.
    pub fn torus(ring_radius: f64, pipe_radius: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"torusWithRingRadius:pipeRadius:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { inner: f(class!(b"SCNTorus\0") as Id, sel, ring_radius, pipe_radius) }
        }
    }

    /// Create a capsule.
    pub fn capsule(cap_radius: f64, height: f64) -> Self {
        unsafe {
            let sel = sel_registerName(b"capsuleWithCapRadius:height:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { inner: f(class!(b"SCNCapsule\0") as Id, sel, cap_radius, height) }
        }
    }

    /// Create 3D text.
    pub fn text(string: &str, extrusion_depth: f64) -> Self {
        unsafe {
            let ns = nsstring(string);
            let sel = sel_registerName(b"textWithString:extrusionDepth:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let g = f(class!(b"SCNText\0") as Id, sel, ns, extrusion_depth);
            CFRelease(ns as CFTypeRef);
            Self { inner: g }
        }
    }

    /// Set the first material's diffuse color (RGBA).
    pub fn set_color(&self, r: f64, g: f64, b: f64, a: f64) {
        unsafe {
            let sel = sel_registerName(b"colorWithRed:green:blue:alpha:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, f64, f64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            #[cfg(target_os = "macos")]
            let color = f(class!(b"NSColor\0") as Id, sel, r, g, b, a);
            #[cfg(not(target_os = "macos"))]
            let color = f(class!(b"UIColor\0") as Id, sel, r, g, b, a);
            let mat: Id = msg_send![self.inner, firstMaterial];
            let diffuse: Id = msg_send![mat, diffuse];
            msg_send_void![diffuse, setContents: color];
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

// ── Light ───────────────────────────────────────────────────────────────────

pub struct Light { inner: Id }

impl Light {
    pub fn new() -> Self { Self { inner: unsafe { msg_send![class!(b"SCNLight\0"), light] } } }

    /// Set light type: "omni", "directional", "spot", "ambient", "IES", "probe".
    pub fn set_type(&self, light_type: &str) {
        unsafe {
            let ns = nsstring(light_type);
            msg_send_void![self.inner, setType: ns];
            CFRelease(ns as CFTypeRef);
        }
    }

    pub fn set_intensity(&self, intensity: f64) {
        unsafe {
            let sel = sel_registerName(b"setIntensity:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, intensity);
        }
    }

    pub fn set_temperature(&self, kelvin: f64) {
        unsafe {
            let sel = sel_registerName(b"setTemperature:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, kelvin);
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}
impl Default for Light { fn default() -> Self { Self::new() } }

// ── Camera ──────────────────────────────────────────────────────────────────

pub struct Camera { inner: Id }

impl Camera {
    pub fn new() -> Self { Self { inner: unsafe { msg_send![class!(b"SCNCamera\0"), camera] } } }

    pub fn set_field_of_view(&self, fov: f64) {
        unsafe {
            let sel = sel_registerName(b"setFieldOfView:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, fov);
        }
    }

    pub fn set_z_near(&self, near: f64) {
        unsafe {
            let sel = sel_registerName(b"setZNear:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, near);
        }
    }

    pub fn set_z_far(&self, far: f64) {
        unsafe {
            let sel = sel_registerName(b"setZFar:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, far);
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}
impl Default for Camera { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_root() {
        let scene = Scene::new();
        let root = scene.root_node();
        assert_eq!(root.child_count(), 0);
    }

    #[test]
    #[ignore] // SCNNode tree ops can segfault without graphics context
    fn test_node_hierarchy() {
        let parent = Node::new();
        parent.set_name("parent");
        let child = Node::new();
        child.set_name("child");
        parent.add_child(&child);
        assert_eq!(parent.child_count(), 1);
        assert!(parent.child_with_name("child", false).is_some());
    }

    #[test]
    fn test_geometries() {
        let _ = Geometry::box_geo(1.0, 1.0, 1.0, 0.0);
        let _ = Geometry::sphere(0.5);
        let _ = Geometry::plane(2.0, 2.0);
        let _ = Geometry::cylinder(0.5, 2.0);
        let _ = Geometry::cone(0.0, 0.5, 1.0);
        let _ = Geometry::torus(1.0, 0.3);
        let _ = Geometry::capsule(0.3, 1.5);
        let _ = Geometry::text("Hello", 0.1);
    }

    #[test]
    #[ignore] // Needs graphics context
    fn test_node_with_geometry() {
        let geo = Geometry::sphere(1.0);
        geo.set_color(1.0, 0.0, 0.0, 1.0);
        let node = Node::with_geometry(&geo);
        node.set_position(1.0, 2.0, 3.0);
        let (x, y, z) = node.position();
        assert!((x - 1.0).abs() < 0.01);
        assert!((y - 2.0).abs() < 0.01);
        assert!((z - 3.0).abs() < 0.01);
    }

    #[test]
    #[ignore] // SCNLight/SCNCamera need a graphics context
    fn test_light_camera() {
        let light = Light::new();
        light.set_type("omni");
        light.set_intensity(1000.0);
        let camera = Camera::new();
        camera.set_field_of_view(60.0);
        let node = Node::new();
        node.set_light(&light);
        let cam_node = Node::new();
        cam_node.set_camera(&camera);
    }
}
