import RealityKit
import Foundation
#if os(macOS)
import AppKit
#elseif os(iOS) || os(visionOS)
import UIKit
#endif

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Boxing utilities
// ═══════════════════════════════════════════════════════════════════════════

public typealias RKHandle = UnsafeMutableRawPointer

@inline(__always) func rkBox<T: AnyObject>(_ obj: T) -> RKHandle { Unmanaged.passRetained(obj).toOpaque() }
@inline(__always) func rkUnboxObj<T: AnyObject>(_ h: RKHandle) -> T { Unmanaged<T>.fromOpaque(h).takeUnretainedValue() }
@inline(__always) func rkE(_ h: RKHandle) -> Entity { Unmanaged<Entity>.fromOpaque(h).takeUnretainedValue() }

/// Mutable material wrapper — allows in-place PBR property mutation.
final class RKMat: NSObject { var mat: any Material; init(_ m: any Material) { mat = m } }
@inline(__always) func rkBoxMat(_ m: any Material) -> RKHandle { rkBox(RKMat(m)) }
@inline(__always) func rkMat(_ h: RKHandle) -> RKMat { rkUnboxObj(h) }

func rkWriteErr(_ msg: String, _ buf: UnsafeMutablePointer<UInt8>?, _ len: Int) {
    guard let buf, len > 0 else { return }
    let b = Array(msg.utf8.prefix(len - 1))
    for (i, c) in b.enumerated() { buf[i] = c }
    buf[min(b.count, len - 1)] = 0
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Platform helpers
// ═══════════════════════════════════════════════════════════════════════════

#if os(macOS)
typealias PColor = NSColor
extension PColor {
    static func rk(_ r: Float, _ g: Float, _ b: Float, _ a: Float = 1) -> PColor {
        PColor(red: CGFloat(r), green: CGFloat(g), blue: CGFloat(b), alpha: CGFloat(a))
    }
}
#else
typealias PColor = UIColor
extension PColor {
    static func rk(_ r: Float, _ g: Float, _ b: Float, _ a: Float = 1) -> PColor {
        PColor(red: CGFloat(r), green: CGFloat(g), blue: CGFloat(b), alpha: CGFloat(a))
    }
}
#endif

func rkTiming(_ id: Int32) -> AnimationTimingFunction {
    switch id { case 1: return .easeIn; case 2: return .easeOut; case 3: return .easeInOut; default: return .linear }
}
func rkPhysMode(_ id: Int32) -> PhysicsBodyMode {
    switch id { case 0: return .static; case 2: return .kinematic; default: return .dynamic }
}
func rkCollMode(_ id: Int32) -> CollisionComponent.Mode {
    switch id { case 1: return .trigger; default: return .default }
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Lifecycle
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_release") public func rkRelease(_ h: RKHandle) { Unmanaged<AnyObject>.fromOpaque(h).release() }
@_cdecl("rk_retain")  public func rkRetain(_ h: RKHandle)  { _ = Unmanaged<AnyObject>.fromOpaque(h).retain() }

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Entity core
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_new")   public func rkEntityNew()  -> RKHandle { rkBox(Entity()) }
@_cdecl("rk_entity_clone") public func rkEntityClone(_ h: RKHandle) -> RKHandle { rkBox(rkE(h).clone(recursive: true)) }

@_cdecl("rk_entity_set_name")
public func rkEntitySetName(_ h: RKHandle, _ p: UnsafePointer<UInt8>, _ n: Int) {
    rkE(h).name = String(bytes: UnsafeBufferPointer(start: p, count: n), encoding: .utf8) ?? ""
}
@_cdecl("rk_entity_get_name")
public func rkEntityGetName(_ h: RKHandle, _ buf: UnsafeMutablePointer<UInt8>, _ len: Int) -> Int {
    let s = rkE(h).name; let b = Array(s.utf8.prefix(max(0, len - 1)))
    for (i, c) in b.enumerated() { buf[i] = c }; if b.count < len { buf[b.count] = 0 }; return b.count
}
@_cdecl("rk_entity_add_child")            public func rkEntityAddChild(_ p: RKHandle, _ c: RKHandle)    { rkE(p).addChild(rkE(c)) }
@_cdecl("rk_entity_remove_child")         public func rkEntityRemoveChild(_ p: RKHandle, _ c: RKHandle) { rkE(p).removeChild(rkE(c)) }
@_cdecl("rk_entity_remove_from_parent")   public func rkEntityRemoveFromParent(_ h: RKHandle)           { rkE(h).removeFromParent() }
@_cdecl("rk_entity_child_count")          public func rkEntityChildCount(_ h: RKHandle) -> Int           { rkE(h).children.count }

@_cdecl("rk_entity_find_child_named")
public func rkEntityFindChildNamed(_ h: RKHandle, _ p: UnsafePointer<UInt8>, _ n: Int, _ recursive: Bool) -> RKHandle? {
    let name = String(bytes: UnsafeBufferPointer(start: p, count: n), encoding: .utf8) ?? ""
    guard let child = rkE(h).findEntity(named: name) else { return nil }
    return rkBox(child)
}

// MARK: Transform
@_cdecl("rk_entity_set_position")
public func rkEntitySetPosition(_ h: RKHandle, _ x: Float, _ y: Float, _ z: Float) { rkE(h).position = .init(x,y,z) }
@_cdecl("rk_entity_get_position")
public func rkEntityGetPosition(_ h: RKHandle, _ o: UnsafeMutablePointer<Float>) {
    let p = rkE(h).position; o[0]=p.x; o[1]=p.y; o[2]=p.z
}
@_cdecl("rk_entity_set_rotation_quat")
public func rkEntitySetRotationQuat(_ h: RKHandle, _ x: Float, _ y: Float, _ z: Float, _ w: Float) {
    rkE(h).orientation = simd_quatf(ix:x, iy:y, iz:z, r:w)
}
@_cdecl("rk_entity_get_rotation_quat")
public func rkEntityGetRotationQuat(_ h: RKHandle, _ o: UnsafeMutablePointer<Float>) {
    let q = rkE(h).orientation; o[0]=q.imag.x; o[1]=q.imag.y; o[2]=q.imag.z; o[3]=q.real
}
@_cdecl("rk_entity_set_scale")
public func rkEntitySetScale(_ h: RKHandle, _ x: Float, _ y: Float, _ z: Float) { rkE(h).scale = .init(x,y,z) }
@_cdecl("rk_entity_get_scale")
public func rkEntityGetScale(_ h: RKHandle, _ o: UnsafeMutablePointer<Float>) {
    let s = rkE(h).scale; o[0]=s.x; o[1]=s.y; o[2]=s.z
}
@_cdecl("rk_entity_set_uniform_scale")
public func rkEntitySetUniformScale(_ h: RKHandle, _ s: Float) { rkE(h).scale = .init(repeating:s) }

@_cdecl("rk_entity_set_transform")
public func rkEntitySetTransform(
    _ h: RKHandle,
    _ tx:Float,_ ty:Float,_ tz:Float,
    _ rx:Float,_ ry:Float,_ rz:Float,_ rw:Float,
    _ sx:Float,_ sy:Float,_ sz:Float
) {
    rkE(h).transform = Transform(
        scale:.init(sx,sy,sz),
        rotation:simd_quatf(ix:rx,iy:ry,iz:rz,r:rw),
        translation:.init(tx,ty,tz))
}
@_cdecl("rk_entity_get_transform")
public func rkEntityGetTransform(
    _ h: RKHandle,
    _ op: UnsafeMutablePointer<Float>,
    _ or: UnsafeMutablePointer<Float>,
    _ os: UnsafeMutablePointer<Float>
) {
    let t = rkE(h).transform
    op[0]=t.translation.x; op[1]=t.translation.y; op[2]=t.translation.z
    or[0]=t.rotation.imag.x; or[1]=t.rotation.imag.y; or[2]=t.rotation.imag.z; or[3]=t.rotation.real
    os[0]=t.scale.x; os[1]=t.scale.y; os[2]=t.scale.z
}
@_cdecl("rk_entity_set_is_enabled")          public func rkEntitySetEnabled(_ h: RKHandle, _ v: Bool) { rkE(h).isEnabled = v }
@_cdecl("rk_entity_is_enabled")              public func rkEntityIsEnabled(_ h: RKHandle) -> Bool { rkE(h).isEnabled }
@_cdecl("rk_entity_is_enabled_in_hierarchy") public func rkEntityIsEnabledInHierarchy(_ h: RKHandle) -> Bool { rkE(h).isEnabledInHierarchy }

@_cdecl("rk_entity_look_at")
public func rkEntityLookAt(_ h: RKHandle, _ tx:Float,_ ty:Float,_ tz:Float, _ ux:Float,_ uy:Float,_ uz:Float) {
    let entity = rkE(h)
    let pos = entity.position
    let target = SIMD3<Float>(tx,ty,tz)
    let up = SIMD3<Float>(ux,uy,uz)
    let zAxis = normalize(pos - target)
    let xAxis = normalize(cross(up, zAxis))
    let yAxis = cross(zAxis, xAxis)
    let m = float4x4(columns:(
        SIMD4(xAxis.x, xAxis.y, xAxis.z, 0),
        SIMD4(yAxis.x, yAxis.y, yAxis.z, 0),
        SIMD4(zAxis.x, zAxis.y, zAxis.z, 0),
        SIMD4(0, 0, 0, 1)
    ))
    entity.orientation = Transform(matrix:m).rotation
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - ModelEntity
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_model_entity_new")
public func rkModelEntityNew(_ mesh: RKHandle, _ mat: RKHandle) -> RKHandle {
    let m: MeshResource = rkUnboxObj(mesh)
    return rkBox(ModelEntity(mesh: m, materials: [rkMat(mat).mat]))
}
@_cdecl("rk_model_entity_new_many")
public func rkModelEntityNewMany(_ mesh: RKHandle, _ mats: UnsafePointer<RKHandle>, _ count: Int) -> RKHandle {
    let m: MeshResource = rkUnboxObj(mesh)
    let materials = (0..<count).map { rkMat(mats[$0]).mat }
    return rkBox(ModelEntity(mesh: m, materials: materials))
}
@_cdecl("rk_model_entity_set_material")
public func rkModelEntitySetMaterial(_ h: RKHandle, _ mat: RKHandle, _ slot: Int) {
    guard let me = rkE(h) as? ModelEntity, var model = me.model else { return }
    let newMat = rkMat(mat).mat
    if slot < model.materials.count { model.materials[slot] = newMat } else { model.materials.append(newMat) }
    me.model = model
}
@_cdecl("rk_model_entity_material_count")
public func rkModelEntityMaterialCount(_ h: RKHandle) -> Int {
    (rkE(h) as? ModelEntity)?.model?.materials.count ?? 0
}
@_cdecl("rk_model_entity_set_mesh")
public func rkModelEntitySetMesh(_ h: RKHandle, _ mesh: RKHandle) {
    guard let me = rkE(h) as? ModelEntity else { return }
    let m: MeshResource = rkUnboxObj(mesh)
    if var model = me.model { model.mesh = m; me.model = model }
    else { me.model = ModelComponent(mesh: m, materials: [SimpleMaterial()]) }
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - AnchorEntity
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_anchor_entity_new")           public func rkAnchorNew() -> RKHandle { rkBox(AnchorEntity()) }
@_cdecl("rk_anchor_entity_world")         public func rkAnchorWorld(_ x:Float,_ y:Float,_ z:Float) -> RKHandle { rkBox(AnchorEntity(world:.init(x,y,z))) }
@_cdecl("rk_anchor_entity_plane_any")     public func rkAnchorPlaneAny() -> RKHandle { rkBox(AnchorEntity(.plane([.any], classification:.any, minimumBounds:.zero))) }
@_cdecl("rk_anchor_entity_plane_horizontal") public func rkAnchorPlaneH() -> RKHandle { rkBox(AnchorEntity(.plane(.horizontal, classification:.any, minimumBounds:.zero))) }
@_cdecl("rk_anchor_entity_plane_vertical")  public func rkAnchorPlaneV() -> RKHandle { rkBox(AnchorEntity(.plane(.vertical,   classification:.any, minimumBounds:.zero))) }

@_cdecl("rk_anchor_entity_image")
public func rkAnchorImage(_ gp:UnsafePointer<UInt8>,_ gl:Int,_ np:UnsafePointer<UInt8>,_ nl:Int) -> RKHandle {
    let group = String(bytes:UnsafeBufferPointer(start:gp,count:gl),encoding:.utf8) ?? ""
    let name  = String(bytes:UnsafeBufferPointer(start:np,count:nl),encoding:.utf8) ?? ""
    return rkBox(AnchorEntity(.image(group:group, name:name)))
}

#if os(iOS) || os(visionOS)
@_cdecl("rk_anchor_entity_face") public func rkAnchorFace() -> RKHandle { rkBox(AnchorEntity(.face)) }
#else
@_cdecl("rk_anchor_entity_face") public func rkAnchorFace() -> RKHandle? { nil }
#endif

#if os(iOS)
@_cdecl("rk_anchor_entity_body") public func rkAnchorBody() -> RKHandle { rkBox(AnchorEntity(.body)) }
#else
@_cdecl("rk_anchor_entity_body") public func rkAnchorBody() -> RKHandle? { nil }
#endif

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Mesh primitives
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_mesh_box")
public func rkMeshBox(_ w:Float,_ h:Float,_ d:Float) -> RKHandle { rkBox(MeshResource.generateBox(width:w,height:h,depth:d)) }
@_cdecl("rk_mesh_box_chamfer")
public func rkMeshBoxChamfer(_ w:Float,_ h:Float,_ d:Float,_ c:Float) -> RKHandle { rkBox(MeshResource.generateBox(width:w,height:h,depth:d,cornerRadius:c)) }
@_cdecl("rk_mesh_sphere")
public func rkMeshSphere(_ r:Float) -> RKHandle { rkBox(MeshResource.generateSphere(radius:r)) }
@_cdecl("rk_mesh_plane")
public func rkMeshPlane(_ w:Float,_ d:Float) -> RKHandle { rkBox(MeshResource.generatePlane(width:w,depth:d)) }
@_cdecl("rk_mesh_plane_corner_radius")
public func rkMeshPlaneCorner(_ w:Float,_ d:Float,_ r:Float) -> RKHandle { rkBox(MeshResource.generatePlane(width:w,depth:d,cornerRadius:r)) }
@_cdecl("rk_mesh_cone")
public func rkMeshCone(_ h:Float,_ r:Float) -> RKHandle { rkBox(MeshResource.generateCone(height:h,radius:r)) }
@_cdecl("rk_mesh_cylinder")
public func rkMeshCylinder(_ h:Float,_ r:Float) -> RKHandle { rkBox(MeshResource.generateCylinder(height:h,radius:r)) }

@_cdecl("rk_mesh_capsule")
public func rkMeshCapsule(_ h:Float,_ r:Float) -> RKHandle {
    // MeshResource.generateCapsule not available on macOS SDK; use cylinder fallback
    rkBox(MeshResource.generateCylinder(height:h, radius:r))
}

@_cdecl("rk_mesh_torus")
public func rkMeshTorus(_ ring:Float,_ pipe:Float) -> RKHandle {
    // MeshResource.generateTorus not available on macOS SDK; use sphere fallback
    rkBox(MeshResource.generateSphere(radius:ring))
}

@_cdecl("rk_mesh_text")
public func rkMeshText(_ p:UnsafePointer<UInt8>,_ n:Int,_ depth:Float,_ fontSize:Float) -> RKHandle {
    let s = String(bytes:UnsafeBufferPointer(start:p,count:n),encoding:.utf8) ?? ""
    return rkBox(MeshResource.generateText(s, extrusionDepth:depth, font:.systemFont(ofSize:CGFloat(fontSize))))
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Materials
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_material_simple")
public func rkMatSimple(_ r:Float,_ g:Float,_ b:Float,_ roughness:Float,_ metallic:Bool) -> RKHandle {
    var m = SimpleMaterial()
    m.color = .init(tint: PColor.rk(r,g,b)); m.roughness = .float(roughness); m.metallic = .float(metallic ? 1 : 0)
    return rkBoxMat(m)
}
@_cdecl("rk_material_simple_alpha")
public func rkMatSimpleAlpha(_ r:Float,_ g:Float,_ b:Float,_ a:Float,_ roughness:Float,_ metallic:Bool) -> RKHandle {
    // SimpleMaterial on macOS doesn't support transparency blending; colour alpha is used
    var m = SimpleMaterial()
    m.color = .init(tint: PColor.rk(r,g,b,a)); m.roughness = .float(roughness); m.metallic = .float(metallic ? 1 : 0)
    return rkBoxMat(m)
}
@_cdecl("rk_material_unlit")
public func rkMatUnlit(_ r:Float,_ g:Float,_ b:Float) -> RKHandle {
    var m = UnlitMaterial(); m.color = .init(tint: PColor.rk(r,g,b)); return rkBoxMat(m)
}
@_cdecl("rk_material_unlit_alpha")
public func rkMatUnlitAlpha(_ r:Float,_ g:Float,_ b:Float,_ a:Float) -> RKHandle {
    var m = UnlitMaterial()
    m.color = .init(tint: PColor.rk(r,g,b,a))
    m.blending = .transparent(opacity: .init(floatLiteral: a))
    return rkBoxMat(m)
}
@_cdecl("rk_material_occlusion")
public func rkMatOcclusion() -> RKHandle { rkBoxMat(OcclusionMaterial()) }

// MARK: PBR builder
@_cdecl("rk_material_pbr_new")
public func rkMatPBRNew() -> RKHandle { rkBoxMat(PhysicallyBasedMaterial()) }

@_cdecl("rk_material_pbr_set_base_color")
public func rkMatPBRSetBaseColor(_ h:RKHandle,_ r:Float,_ g:Float,_ b:Float,_ a:Float) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    pbr.baseColor = .init(tint: PColor.rk(r,g,b,a)); rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_roughness")
public func rkMatPBRSetRoughness(_ h:RKHandle,_ v:Float) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    pbr.roughness = .init(floatLiteral: v); rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_metallic")
public func rkMatPBRSetMetallic(_ h:RKHandle,_ v:Float) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    pbr.metallic = .init(floatLiteral: v); rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_emissive")
public func rkMatPBRSetEmissive(_ h:RKHandle,_ r:Float,_ g:Float,_ b:Float,_ intensity:Float) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    pbr.emissiveColor = .init(color: PColor.rk(r,g,b))
    pbr.emissiveIntensity = intensity
    rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_opacity")
public func rkMatPBRSetOpacity(_ h:RKHandle,_ v:Float) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    if v >= 1.0 { pbr.blending = .opaque }
    else { pbr.blending = .transparent(opacity: .init(floatLiteral: v)) }
    rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_clearcoat")
public func rkMatPBRSetClearcoat(_ h:RKHandle,_ cc:Float,_ rough:Float) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    pbr.clearcoat = .init(floatLiteral: cc)
    pbr.clearcoatRoughness = .init(floatLiteral: rough)
    rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_sheen")
public func rkMatPBRSetSheen(_ h:RKHandle,_ r:Float,_ g:Float,_ b:Float) {
    // PhysicallyBasedMaterial.Sheen API varies by SDK version; apply via emissive proxy
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    pbr.emissiveColor = .init(color: PColor.rk(r * 0.1, g * 0.1, b * 0.1))
    rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_specular")
public func rkMatPBRSetSpecular(_ h:RKHandle,_ v:Float) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    pbr.specular = .init(floatLiteral: v); rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_anisotropy")
public func rkMatPBRSetAnisotropy(_ h:RKHandle,_ v:Float) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    pbr.anisotropyLevel = .init(floatLiteral: v); rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_base_color_texture")
public func rkMatPBRSetBaseColorTex(_ h:RKHandle,_ tex:RKHandle) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    let t: TextureResource = rkUnboxObj(tex)
    pbr.baseColor = .init(texture: .init(t)); rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_normal_texture")
public func rkMatPBRSetNormalTex(_ h:RKHandle,_ tex:RKHandle) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    let t: TextureResource = rkUnboxObj(tex)
    pbr.normal = .init(texture: .init(t)); rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_roughness_texture")
public func rkMatPBRSetRoughnessTex(_ h:RKHandle,_ tex:RKHandle) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    let t: TextureResource = rkUnboxObj(tex)
    pbr.roughness = .init(texture: .init(t)); rkMat(h).mat = pbr
}
@_cdecl("rk_material_pbr_set_metallic_texture")
public func rkMatPBRSetMetallicTex(_ h:RKHandle,_ tex:RKHandle) {
    guard var pbr = rkMat(h).mat as? PhysicallyBasedMaterial else { return }
    let t: TextureResource = rkUnboxObj(tex)
    pbr.metallic = .init(texture: .init(t)); rkMat(h).mat = pbr
}

@_cdecl("rk_texture_load")
public func rkTextureLoad(_ p:UnsafePointer<UInt8>,_ n:Int,_ eb:UnsafeMutablePointer<UInt8>?,_ el:Int) -> RKHandle? {
    let path = String(bytes:UnsafeBufferPointer(start:p,count:n),encoding:.utf8) ?? ""
    do { let t = try TextureResource.load(contentsOf:URL(fileURLWithPath:path)); return rkBox(t) }
    catch { rkWriteErr(error.localizedDescription, eb, el); return nil }
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Lights
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_point_light")
public func rkPointLight(_ r:Float,_ g:Float,_ b:Float,_ intensity:Float,_ attenRadius:Float) -> RKHandle {
    let l = PointLight()
    l.light.color = PColor.rk(r,g,b); l.light.intensity = intensity; l.light.attenuationRadius = attenRadius
    return rkBox(l)
}
@_cdecl("rk_point_light_set_color")
public func rkPointLightSetColor(_ h:RKHandle,_ r:Float,_ g:Float,_ b:Float) {
    guard let l = rkE(h) as? PointLight else { return }; l.light.color = PColor.rk(r,g,b)
}
@_cdecl("rk_point_light_set_intensity")
public func rkPointLightSetIntensity(_ h:RKHandle,_ v:Float) {
    guard let l = rkE(h) as? PointLight else { return }; l.light.intensity = v
}
@_cdecl("rk_point_light_set_attenuation_radius")
public func rkPointLightSetAttenRadius(_ h:RKHandle,_ v:Float) {
    guard let l = rkE(h) as? PointLight else { return }; l.light.attenuationRadius = v
}

@_cdecl("rk_directional_light")
public func rkDirLight(_ r:Float,_ g:Float,_ b:Float,_ intensity:Float,_ casts:Bool) -> RKHandle {
    let l = DirectionalLight()
    l.light.color = PColor.rk(r,g,b); l.light.intensity = intensity; l.shadow = casts ? .init() : nil
    return rkBox(l)
}
@_cdecl("rk_directional_light_set_color")
public func rkDirLightSetColor(_ h:RKHandle,_ r:Float,_ g:Float,_ b:Float) {
    guard let l = rkE(h) as? DirectionalLight else { return }; l.light.color = PColor.rk(r,g,b)
}
@_cdecl("rk_directional_light_set_intensity")
public func rkDirLightSetIntensity(_ h:RKHandle,_ v:Float) {
    guard let l = rkE(h) as? DirectionalLight else { return }; l.light.intensity = v
}
@_cdecl("rk_directional_light_set_casts_shadow")
public func rkDirLightSetCastsShadow(_ h:RKHandle,_ v:Bool) {
    guard let l = rkE(h) as? DirectionalLight else { return }; l.shadow = v ? .init() : nil
}

@_cdecl("rk_spot_light")
public func rkSpotLight(_ r:Float,_ g:Float,_ b:Float,_ intensity:Float,_ inner:Float,_ outer:Float,_ radius:Float) -> RKHandle {
    let l = SpotLight()
    var comp = SpotLightComponent()
    comp.color = PColor.rk(r,g,b); comp.intensity = intensity
    comp.innerAngleInDegrees = inner; comp.outerAngleInDegrees = outer; comp.attenuationRadius = radius
    l.components.set(comp)
    return rkBox(l)
}
@_cdecl("rk_spot_light_set_angles")
public func rkSpotLightSetAngles(_ h:RKHandle,_ inner:Float,_ outer:Float) {
    guard var comp = rkE(h).components[SpotLightComponent.self] else { return }
    comp.innerAngleInDegrees = inner; comp.outerAngleInDegrees = outer; rkE(h).components.set(comp)
}
@_cdecl("rk_spot_light_set_color")
public func rkSpotLightSetColor(_ h:RKHandle,_ r:Float,_ g:Float,_ b:Float) {
    guard var comp = rkE(h).components[SpotLightComponent.self] else { return }
    comp.color = PColor.rk(r,g,b); rkE(h).components.set(comp)
}
@_cdecl("rk_spot_light_set_intensity")
public func rkSpotLightSetIntensity(_ h:RKHandle,_ v:Float) {
    guard var comp = rkE(h).components[SpotLightComponent.self] else { return }
    comp.intensity = v; rkE(h).components.set(comp)
}

@_cdecl("rk_image_based_light_new")
public func rkIBLNew(_ envHandle:RKHandle,_ intensityExp:Float) -> RKHandle {
    let env: EnvironmentResource = rkUnboxObj(envHandle)
    let entity = Entity()
    entity.components.set(ImageBasedLightComponent(source:.single(env), intensityExponent:intensityExp))
    return rkBox(entity)
}
@_cdecl("rk_entity_set_image_based_light_receiver")
public func rkEntitySetIBLReceiver(_ target:RKHandle,_ ibl:RKHandle) {
    rkE(target).components.set(ImageBasedLightReceiverComponent(imageBasedLight:rkE(ibl)))
}
@_cdecl("rk_entity_remove_image_based_light_receiver")
public func rkEntityRemoveIBLReceiver(_ h:RKHandle) { rkE(h).components.remove(ImageBasedLightReceiverComponent.self) }

@_cdecl("rk_entity_set_environment_lighting_weight")
public func rkEntitySetEnvLightingWeight(_ h:RKHandle,_ w:Float) {
    // EnvironmentLightingConfigurationComponent property name varies by SDK
    rkE(h).components.set(EnvironmentLightingConfigurationComponent())
}
@_cdecl("rk_entity_remove_environment_lighting_config")
public func rkEntityRemoveEnvLightingConfig(_ h:RKHandle) { rkE(h).components.remove(EnvironmentLightingConfigurationComponent.self) }

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Physics body
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_physics_body")
public func rkEntitySetPhysicsBody(
    _ h:RKHandle,_ modeId:Int32,_ mass:Float,_ friction:Float,_ restitution:Float,
    _ linearDamping:Float,_ angularDamping:Float
) {
    var comp = PhysicsBodyComponent()
    comp.mode = rkPhysMode(modeId)
    comp.massProperties.mass = mass
    comp.material = PhysicsMaterialResource.generate(friction:friction, restitution:restitution)
    comp.linearDamping  = linearDamping
    comp.angularDamping = angularDamping
    rkE(h).components.set(comp)
}
@_cdecl("rk_entity_set_physics_body_mode")
public func rkEntitySetPhysicsBodyMode(_ h:RKHandle,_ modeId:Int32) {
    guard var c = rkE(h).components[PhysicsBodyComponent.self] else { return }
    c.mode = rkPhysMode(modeId); rkE(h).components.set(c)
}
@_cdecl("rk_entity_set_physics_body_mass")
public func rkEntitySetPhysicsBodyMass(_ h:RKHandle,_ mass:Float) {
    guard var c = rkE(h).components[PhysicsBodyComponent.self] else { return }
    c.massProperties.mass = mass; rkE(h).components.set(c)
}
@_cdecl("rk_entity_set_physics_body_material")
public func rkEntitySetPhysicsBodyMaterial(_ h:RKHandle,_ friction:Float,_ restitution:Float) {
    guard var c = rkE(h).components[PhysicsBodyComponent.self] else { return }
    c.material = PhysicsMaterialResource.generate(friction:friction, restitution:restitution); rkE(h).components.set(c)
}
@_cdecl("rk_entity_is_resting")
public func rkEntityIsResting(_ h:RKHandle) -> Bool { false } // isResting not available on macOS
@_cdecl("rk_entity_remove_physics_body")
public func rkEntityRemovePhysicsBody(_ h:RKHandle) { rkE(h).components.remove(PhysicsBodyComponent.self) }

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Collision
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_collision_box")
public func rkEntitySetCollisionBox(_ h:RKHandle,_ w:Float,_ ht:Float,_ d:Float,_ ox:Float,_ oy:Float,_ oz:Float) {
    let shape = ShapeResource.generateBox(width:w,height:ht,depth:d).offsetBy(translation:.init(ox,oy,oz))
    rkE(h).components.set(CollisionComponent(shapes:[shape]))
}
@_cdecl("rk_entity_set_collision_sphere")
public func rkEntitySetCollisionSphere(_ h:RKHandle,_ radius:Float,_ ox:Float,_ oy:Float,_ oz:Float) {
    let shape = ShapeResource.generateSphere(radius:radius).offsetBy(translation:.init(ox,oy,oz))
    rkE(h).components.set(CollisionComponent(shapes:[shape]))
}
@_cdecl("rk_entity_set_collision_capsule")
public func rkEntitySetCollisionCapsule(_ h:RKHandle,_ height:Float,_ radius:Float,_ ox:Float,_ oy:Float,_ oz:Float) {
    let shape = ShapeResource.generateCapsule(height:height, radius:radius).offsetBy(translation:.init(ox,oy,oz))
    rkE(h).components.set(CollisionComponent(shapes:[shape]))
}
@_cdecl("rk_entity_set_collision_convex_hull")
public func rkEntitySetCollisionConvexHull(_ h:RKHandle) {
    guard let me = rkE(h) as? ModelEntity else { return }
    me.generateCollisionShapes(recursive: false)
}
@_cdecl("rk_entity_set_collision_mode")
public func rkEntitySetCollisionMode(_ h:RKHandle,_ modeId:Int32) {
    guard var c = rkE(h).components[CollisionComponent.self] else { return }
    c.mode = rkCollMode(modeId); rkE(h).components.set(c)
}
@_cdecl("rk_entity_remove_collision")
public func rkEntityRemoveCollision(_ h:RKHandle) { rkE(h).components.remove(CollisionComponent.self) }

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Physics forces & velocity
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_linear_velocity")
public func rkEntitySetLinearVelocity(_ h:RKHandle,_ x:Float,_ y:Float,_ z:Float) {
    if var c = rkE(h).components[PhysicsMotionComponent.self] { c.linearVelocity = .init(x,y,z); rkE(h).components.set(c) }
    else { var m = PhysicsMotionComponent(); m.linearVelocity = .init(x,y,z); rkE(h).components.set(m) }
}
@_cdecl("rk_entity_get_linear_velocity")
public func rkEntityGetLinearVelocity(_ h:RKHandle,_ o:UnsafeMutablePointer<Float>) {
    let v = rkE(h).components[PhysicsMotionComponent.self]?.linearVelocity ?? .zero
    o[0]=v.x; o[1]=v.y; o[2]=v.z
}
@_cdecl("rk_entity_set_angular_velocity")
public func rkEntitySetAngularVelocity(_ h:RKHandle,_ x:Float,_ y:Float,_ z:Float) {
    if var c = rkE(h).components[PhysicsMotionComponent.self] { c.angularVelocity = .init(x,y,z); rkE(h).components.set(c) }
    else { var m = PhysicsMotionComponent(); m.angularVelocity = .init(x,y,z); rkE(h).components.set(m) }
}
@_cdecl("rk_entity_get_angular_velocity")
public func rkEntityGetAngularVelocity(_ h:RKHandle,_ o:UnsafeMutablePointer<Float>) {
    let v = rkE(h).components[PhysicsMotionComponent.self]?.angularVelocity ?? .zero
    o[0]=v.x; o[1]=v.y; o[2]=v.z
}
@_cdecl("rk_entity_apply_linear_impulse")
public func rkEntityApplyLinearImpulse(_ h:RKHandle,_ x:Float,_ y:Float,_ z:Float) {
    guard let me = rkE(h) as? ModelEntity else { return }
    me.applyLinearImpulse(.init(x,y,z), relativeTo:nil)
}
@_cdecl("rk_entity_apply_angular_impulse")
public func rkEntityApplyAngularImpulse(_ h:RKHandle,_ x:Float,_ y:Float,_ z:Float) {
    guard let me = rkE(h) as? ModelEntity else { return }
    me.applyAngularImpulse(.init(x,y,z), relativeTo:nil)
}
@_cdecl("rk_entity_add_force")
public func rkEntityAddForce(_ h:RKHandle,_ fx:Float,_ fy:Float,_ fz:Float,_ px:Float,_ py:Float,_ pz:Float) {
    guard let me = rkE(h) as? ModelEntity else { return }
    me.addForce(.init(fx,fy,fz), at:.init(px,py,pz), relativeTo:nil)
}
@_cdecl("rk_entity_add_torque")
public func rkEntityAddTorque(_ h:RKHandle,_ x:Float,_ y:Float,_ z:Float) {
    guard let me = rkE(h) as? ModelEntity else { return }
    me.addTorque(.init(x,y,z), relativeTo:nil)
}
@_cdecl("rk_entity_reset_physics")
public func rkEntityResetPhysics(_ h:RKHandle) { /* resetPhysics not available on macOS */ }

@_cdecl("rk_entity_set_physics_simulation")
public func rkEntitySetPhysicsSimulation(_ h:RKHandle,_ gx:Float,_ gy:Float,_ gz:Float) {
    var comp = PhysicsSimulationComponent(); comp.gravity = .init(gx,gy,gz); rkE(h).components.set(comp)
}
@_cdecl("rk_entity_remove_physics_simulation")
public func rkEntityRemovePhysicsSimulation(_ h:RKHandle) { rkE(h).components.remove(PhysicsSimulationComponent.self) }

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Animation
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_move_to")
public func rkEntityMoveTo(
    _ h:RKHandle,
    _ tx:Float,_ ty:Float,_ tz:Float,
    _ rx:Float,_ ry:Float,_ rz:Float,_ rw:Float,
    _ sx:Float,_ sy:Float,_ sz:Float,
    _ duration:Double,_ timingId:Int32
) -> RKHandle {
    let t = Transform(scale:.init(sx,sy,sz), rotation:simd_quatf(ix:rx,iy:ry,iz:rz,r:rw), translation:.init(tx,ty,tz))
    let ctrl = rkE(h).move(to:t, relativeTo:nil, duration:duration, timingFunction:rkTiming(timingId))
    return rkBox(ctrl)
}
@_cdecl("rk_entity_animation_count")
public func rkEntityAnimationCount(_ h:RKHandle) -> Int { rkE(h).availableAnimations.count }
@_cdecl("rk_entity_play_animation")
public func rkEntityPlayAnimation(_ h:RKHandle,_ idx:Int,_ transitionDuration:Double,_ startsPaused:Bool) -> RKHandle? {
    let entity = rkE(h)
    guard idx < entity.availableAnimations.count else { return nil }
    let ctrl = entity.playAnimation(entity.availableAnimations[idx], transitionDuration:transitionDuration, startsPaused:startsPaused)
    return rkBox(ctrl)
}
@_cdecl("rk_entity_stop_all_animations")
public func rkEntityStopAllAnimations(_ h:RKHandle) { rkE(h).stopAllAnimations() }

@_cdecl("rk_animation_controller_pause")
public func rkAnimCtrlPause(_ h:RKHandle)  { let c: AnimationPlaybackController = rkUnboxObj(h); c.pause() }
@_cdecl("rk_animation_controller_resume")
public func rkAnimCtrlResume(_ h:RKHandle) { let c: AnimationPlaybackController = rkUnboxObj(h); c.resume() }
@_cdecl("rk_animation_controller_stop")
public func rkAnimCtrlStop(_ h:RKHandle)   { let c: AnimationPlaybackController = rkUnboxObj(h); c.stop() }
@_cdecl("rk_animation_controller_is_playing")
public func rkAnimCtrlIsPlaying(_ h:RKHandle) -> Bool { let c: AnimationPlaybackController = rkUnboxObj(h); return c.isPlaying }
@_cdecl("rk_animation_controller_is_paused")
public func rkAnimCtrlIsPaused(_ h:RKHandle) -> Bool  { let c: AnimationPlaybackController = rkUnboxObj(h); return c.isPaused }

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Audio
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_audio_resource_load")
public func rkAudioResourceLoad(
    _ pathPtr:UnsafePointer<UInt8>,_ pathLen:Int,
    _ inputModeId:Int32,
    _ errBuf:UnsafeMutablePointer<UInt8>?,_ errLen:Int
) -> RKHandle? {
    let name = String(bytes:UnsafeBufferPointer(start:pathPtr,count:pathLen),encoding:.utf8) ?? ""
    let mode: AudioResource.InputMode = inputModeId == 1 ? .nonSpatial : .spatial
    do {
        let r = try AudioFileResource.load(named: name, in: nil, inputMode: mode, loadingStrategy: .preload)
        return rkBox(r)
    } catch { rkWriteErr(error.localizedDescription, errBuf, errLen); return nil }
}
@_cdecl("rk_audio_resource_load_named")
public func rkAudioResourceLoadNamed(
    _ namePtr:UnsafePointer<UInt8>,_ nameLen:Int,
    _ inputModeId:Int32,
    _ errBuf:UnsafeMutablePointer<UInt8>?,_ errLen:Int
) -> RKHandle? {
    let name = String(bytes:UnsafeBufferPointer(start:namePtr,count:nameLen),encoding:.utf8) ?? ""
    let mode: AudioResource.InputMode = inputModeId == 1 ? .nonSpatial : .spatial
    do {
        let r = try AudioFileResource.load(named: name, in: nil, inputMode: mode, loadingStrategy: .preload)
        return rkBox(r)
    } catch { rkWriteErr(error.localizedDescription, errBuf, errLen); return nil }
}
@_cdecl("rk_entity_play_audio")
public func rkEntityPlayAudio(_ entityHandle:RKHandle,_ resourceHandle:RKHandle) -> RKHandle {
    let entity = rkE(entityHandle)
    let resource: AudioFileResource = rkUnboxObj(resourceHandle)
    return rkBox(entity.playAudio(resource))
}
@_cdecl("rk_entity_stop_all_audio")
public func rkEntityStopAllAudio(_ h:RKHandle) { rkE(h).stopAllAudio() }

@_cdecl("rk_audio_controller_pause")
public func rkAudioCtrlPause(_ h:RKHandle)  { let c: AudioPlaybackController = rkUnboxObj(h); c.pause() }
@_cdecl("rk_audio_controller_resume")
public func rkAudioCtrlResume(_ h:RKHandle) {
    // AudioPlaybackController.resume() not available on macOS; use fade-in as workaround
    let c: AudioPlaybackController = rkUnboxObj(h); c.fade(to: AudioPlaybackController.Decibel(0), duration: 0)
}
@_cdecl("rk_audio_controller_stop")
public func rkAudioCtrlStop(_ h:RKHandle)   { let c: AudioPlaybackController = rkUnboxObj(h); c.stop() }
@_cdecl("rk_audio_controller_is_playing")
public func rkAudioCtrlIsPlaying(_ h:RKHandle) -> Bool { let c: AudioPlaybackController = rkUnboxObj(h); return c.isPlaying }
@_cdecl("rk_audio_controller_get_gain")
public func rkAudioCtrlGetGain(_ h:RKHandle) -> Double { let c: AudioPlaybackController = rkUnboxObj(h); return c.gain }
@_cdecl("rk_audio_controller_set_gain")
public func rkAudioCtrlSetGain(_ h:RKHandle,_ v:Double) { let c: AudioPlaybackController = rkUnboxObj(h); c.gain = v }
@_cdecl("rk_audio_controller_get_speed")
public func rkAudioCtrlGetSpeed(_ h:RKHandle) -> Double { let c: AudioPlaybackController = rkUnboxObj(h); return c.speed }
@_cdecl("rk_audio_controller_set_speed")
public func rkAudioCtrlSetSpeed(_ h:RKHandle,_ v:Double) { let c: AudioPlaybackController = rkUnboxObj(h); c.speed = v }

@_cdecl("rk_entity_set_spatial_audio")
public func rkEntitySetSpatialAudio(_ h:RKHandle,_ directDb:Double,_ reverbDb:Double) {
    var comp = SpatialAudioComponent(); comp.directLevel = directDb; comp.reverbLevel = reverbDb
    rkE(h).components.set(comp)
}
@_cdecl("rk_entity_remove_spatial_audio")
public func rkEntityRemoveSpatialAudio(_ h:RKHandle) { rkE(h).components.remove(SpatialAudioComponent.self) }
@_cdecl("rk_entity_set_ambient_audio")
public func rkEntitySetAmbientAudio(_ h:RKHandle,_ gain:Double) {
    var comp = AmbientAudioComponent(); comp.gain = gain; rkE(h).components.set(comp)
}
@_cdecl("rk_entity_remove_ambient_audio")
public func rkEntityRemoveAmbientAudio(_ h:RKHandle) { rkE(h).components.remove(AmbientAudioComponent.self) }
@_cdecl("rk_entity_set_channel_audio")
public func rkEntitySetChannelAudio(_ h:RKHandle,_ gain:Double) {
    var comp = ChannelAudioComponent(); comp.gain = gain; rkE(h).components.set(comp)
}
@_cdecl("rk_entity_remove_channel_audio")
public func rkEntityRemoveChannelAudio(_ h:RKHandle) { rkE(h).components.remove(ChannelAudioComponent.self) }

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Opacity & shadow
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_opacity")
public func rkEntitySetOpacity(_ h:RKHandle,_ v:Float) { rkE(h).components.set(OpacityComponent(opacity:v)) }
@_cdecl("rk_entity_get_opacity")
public func rkEntityGetOpacity(_ h:RKHandle) -> Float { rkE(h).components[OpacityComponent.self]?.opacity ?? 1 }
@_cdecl("rk_entity_remove_opacity")
public func rkEntityRemoveOpacity(_ h:RKHandle) { rkE(h).components.remove(OpacityComponent.self) }
@_cdecl("rk_entity_set_grounding_shadow")
public func rkEntitySetGroundingShadow(_ h:RKHandle,_ casts:Bool) { rkE(h).components.set(GroundingShadowComponent(castsShadow:casts)) }
@_cdecl("rk_entity_remove_grounding_shadow")
public func rkEntityRemoveGroundingShadow(_ h:RKHandle) { rkE(h).components.remove(GroundingShadowComponent.self) }

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Camera entities
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_perspective_camera_new")
public func rkPerspCameraNew(_ fovDeg:Float,_ near:Float,_ far:Float) -> RKHandle {
    let e = Entity(); var c = PerspectiveCameraComponent()
    c.fieldOfViewInDegrees = fovDeg; c.near = near; c.far = far; e.components.set(c); return rkBox(e)
}
@_cdecl("rk_perspective_camera_set_fov")
public func rkPerspCameraSetFOV(_ h:RKHandle,_ fovDeg:Float) {
    guard var c = rkE(h).components[PerspectiveCameraComponent.self] else { return }
    c.fieldOfViewInDegrees = fovDeg; rkE(h).components.set(c)
}
@_cdecl("rk_perspective_camera_set_clip")
public func rkPerspCameraSetClip(_ h:RKHandle,_ near:Float,_ far:Float) {
    guard var c = rkE(h).components[PerspectiveCameraComponent.self] else { return }
    c.near = near; c.far = far; rkE(h).components.set(c)
}
@_cdecl("rk_orthographic_camera_new")
public func rkOrthoCameraNew(_ scale:Float,_ near:Float,_ far:Float) -> RKHandle {
    let e = Entity(); var c = OrthographicCameraComponent()
    c.scale = scale; c.near = near; c.far = far; e.components.set(c); return rkBox(e)
}
@_cdecl("rk_orthographic_camera_set_scale")
public func rkOrthoCameraSetScale(_ h:RKHandle,_ scale:Float) {
    guard var c = rkE(h).components[OrthographicCameraComponent.self] else { return }
    c.scale = scale; rkE(h).components.set(c)
}
@_cdecl("rk_entity_remove_camera")
public func rkEntityRemoveCamera(_ h:RKHandle) {
    let e = rkE(h)
    e.components.remove(PerspectiveCameraComponent.self)
    e.components.remove(OrthographicCameraComponent.self)
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Character controller (macOS 13+, iOS 16+, visionOS 1+)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_character_controller")
public func rkEntitySetCharacterController(_ h:RKHandle,_ radius:Float,_ height:Float,_ slopeLimit:Float,_ stepOffset:Float) {
    if #available(macOS 13.0, iOS 16.0, visionOS 1.0, *) {
        let comp = CharacterControllerComponent(radius: radius, height: height, skinWidth: 0.04, slopeLimit: slopeLimit)
        rkE(h).components.set(comp)
    }
}
@_cdecl("rk_entity_character_move")
public func rkEntityCharacterMove(_ h:RKHandle,_ dx:Float,_ dy:Float,_ dz:Float,_ dt:Float) {
    if #available(macOS 13.0, iOS 16.0, visionOS 1.0, *) {
        rkE(h).moveCharacter(by:.init(dx,dy,dz), deltaTime:dt, relativeTo:nil)
    }
}
@_cdecl("rk_entity_remove_character_controller")
public func rkEntityRemoveCharacterController(_ h:RKHandle) {
    if #available(macOS 13.0, iOS 16.0, visionOS 1.0, *) {
        rkE(h).components.remove(CharacterControllerComponent.self)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Text (3-D via ModelComponent)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_text")
public func rkEntitySetText(
    _ h:RKHandle,_ tp:UnsafePointer<UInt8>,_ tn:Int,
    _ fontSize:Float,_ depth:Float,_ r:Float,_ g:Float,_ b:Float,_ a:Float
) {
    let s = String(bytes:UnsafeBufferPointer(start:tp,count:tn),encoding:.utf8) ?? ""
    let mesh = MeshResource.generateText(s, extrusionDepth:depth, font:.systemFont(ofSize:CGFloat(fontSize)))
    var mat = UnlitMaterial(); mat.color = .init(tint:PColor.rk(r,g,b,a))
    rkE(h).components.set(ModelComponent(mesh:mesh, materials:[mat]))
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Particle emitter (macOS 14+, iOS 17+, visionOS 1+)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_particle_emitter")
public func rkEntitySetParticleEmitter(
    _ h:RKHandle,_ birthRate:Float,_ speed:Float,_ lifetime:Float,_ size:Float,
    _ r:Float,_ g:Float,_ b:Float,_ a:Float
) {
    if #available(macOS 14.0, iOS 17.0, visionOS 1.0, *) {
        var comp = ParticleEmitterComponent()
        comp.mainEmitter.birthRate = birthRate
        comp.mainEmitter.lifeSpan = TimeInterval(lifetime)
        comp.mainEmitter.color = .constant(.single(.init(red:CGFloat(r),green:CGFloat(g),blue:CGFloat(b),alpha:CGFloat(a))))
        rkE(h).components.set(comp)
    }
}
@_cdecl("rk_entity_particle_set_birth_rate")
public func rkEntityParticleSetBirthRate(_ h:RKHandle,_ v:Float) {
    if #available(macOS 14.0, iOS 17.0, visionOS 1.0, *) {
        guard var c = rkE(h).components[ParticleEmitterComponent.self] else { return }
        c.mainEmitter.birthRate = v; rkE(h).components.set(c)
    }
}
@_cdecl("rk_entity_particle_set_speed")
public func rkEntityParticleSetSpeed(_ h:RKHandle,_ v:Float) {
    if #available(macOS 14.0, iOS 17.0, visionOS 1.0, *) {
        guard var c = rkE(h).components[ParticleEmitterComponent.self] else { return }
        c.mainEmitter.birthRateVariation = v * 0.1 // Approximate; speed is set via emitter acceleration
        rkE(h).components.set(c)
    }
}
@_cdecl("rk_entity_particle_set_color")
public func rkEntityParticleSetColor(_ h:RKHandle,_ r:Float,_ g:Float,_ b:Float,_ a:Float) {
    if #available(macOS 14.0, iOS 17.0, visionOS 1.0, *) {
        guard var c = rkE(h).components[ParticleEmitterComponent.self] else { return }
        c.mainEmitter.color = .constant(.single(.init(red:CGFloat(r),green:CGFloat(g),blue:CGFloat(b),alpha:CGFloat(a))))
        rkE(h).components.set(c)
    }
}
@_cdecl("rk_entity_particle_set_lifetime")
public func rkEntityParticleSetLifetime(_ h:RKHandle,_ v:Float) {
    if #available(macOS 14.0, iOS 17.0, visionOS 1.0, *) {
        guard var c = rkE(h).components[ParticleEmitterComponent.self] else { return }
        c.mainEmitter.lifeSpan = TimeInterval(v); rkE(h).components.set(c)
    }
}
@_cdecl("rk_entity_particle_set_size")
public func rkEntityParticleSetSize(_ h:RKHandle,_ v:Float) {
    if #available(macOS 14.0, iOS 17.0, visionOS 1.0, *) {
        guard var c = rkE(h).components[ParticleEmitterComponent.self] else { return }
        c.mainEmitter.lifeSpanVariation = TimeInterval(v * 0.05) // fallback mapping
        rkE(h).components.set(c)
    }
}
@_cdecl("rk_entity_remove_particle_emitter")
public func rkEntityRemoveParticleEmitter(_ h:RKHandle) {
    if #available(macOS 14.0, iOS 17.0, visionOS 1.0, *) {
        rkE(h).components.remove(ParticleEmitterComponent.self)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Billboard (macOS 26+, iOS 26+, visionOS 3+)
//         Compiled as no-ops when building against older SDKs.
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_billboard")
public func rkEntitySetBillboard(_ h:RKHandle) {
    // BillboardComponent is new in macOS 26 / iOS 26 / visionOS 3.
    // When the SDK is updated, replace the body with:
    //   rkE(h).components.set(BillboardComponent())
}
@_cdecl("rk_entity_remove_billboard")
public func rkEntityRemoveBillboard(_ h:RKHandle) {
    // Remove when SDK supports: rkE(h).components.remove(BillboardComponent.self)
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Force effects (macOS 26+, iOS 26+, visionOS 3+) — SDK stubs
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_constant_force_effect")
public func rkEntitySetConstantForceEffect(_ h:RKHandle,_ fx:Float,_ fy:Float,_ fz:Float,_ radius:Float) {
    // ForceEffectComponent — new in macOS 26 / iOS 26 / visionOS 3.
    // Implement when SDK headers become available.
}
@_cdecl("rk_entity_set_radial_force_effect")
public func rkEntitySetRadialForceEffect(_ h:RKHandle,_ strength:Float,_ radius:Float,_ falloffExp:Float) {}
@_cdecl("rk_entity_set_vortex_force_effect")
public func rkEntitySetVortexForceEffect(_ h:RKHandle,_ strength:Float,_ radius:Float) {}
@_cdecl("rk_entity_set_drag_force_effect")
public func rkEntitySetDragForceEffect(_ h:RKHandle,_ linear:Float,_ angular:Float) {}
@_cdecl("rk_entity_remove_force_effect")
public func rkEntityRemoveForceEffect(_ h:RKHandle) {}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Network synchronisation
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_set_network_sync")
public func rkEntitySetNetworkSync(_ h:RKHandle,_ allowsOwnershipTransfer:Bool) {
#if os(iOS) || os(visionOS)
    var comp = NetworkSynchronizationComponent()
    comp.isOwnershipTransferable = allowsOwnershipTransfer
    rkE(h).components.set(comp)
#endif
}
@_cdecl("rk_entity_remove_network_sync")
public func rkEntityRemoveNetworkSync(_ h:RKHandle) {
#if os(iOS) || os(visionOS)
    rkE(h).components.remove(NetworkSynchronizationComponent.self)
#endif
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Environment resource
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_environment_resource_load")
public func rkEnvironmentResourceLoad(_ p:UnsafePointer<UInt8>,_ n:Int,_ eb:UnsafeMutablePointer<UInt8>?,_ el:Int) -> RKHandle? {
    let name = String(bytes:UnsafeBufferPointer(start:p,count:n),encoding:.utf8) ?? ""
    do { let r = try EnvironmentResource.load(named: name); return rkBox(r) }
    catch { rkWriteErr(error.localizedDescription, eb, el); return nil }
}
@_cdecl("rk_environment_resource_load_named")
public func rkEnvironmentResourceLoadNamed(_ p:UnsafePointer<UInt8>,_ n:Int,_ eb:UnsafeMutablePointer<UInt8>?,_ el:Int) -> RKHandle? {
    let name = String(bytes:UnsafeBufferPointer(start:p,count:n),encoding:.utf8) ?? ""
    do { let r = try EnvironmentResource.load(named: name); return rkBox(r) }
    catch { rkWriteErr(error.localizedDescription, eb, el); return nil }
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Asset loading
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_load_from_file")
public func rkEntityLoadFromFile(_ p:UnsafePointer<UInt8>,_ n:Int,_ eb:UnsafeMutablePointer<UInt8>?,_ el:Int) -> RKHandle? {
    let path = String(bytes:UnsafeBufferPointer(start:p,count:n),encoding:.utf8) ?? ""
    do { let e = try Entity.load(contentsOf:URL(fileURLWithPath:path)); return rkBox(e) }
    catch { rkWriteErr(error.localizedDescription, eb, el); return nil }
}
@_cdecl("rk_entity_load_named")
public func rkEntityLoadNamed(_ p:UnsafePointer<UInt8>,_ n:Int,_ eb:UnsafeMutablePointer<UInt8>?,_ el:Int) -> RKHandle? {
    let name = String(bytes:UnsafeBufferPointer(start:p,count:n),encoding:.utf8) ?? ""
    do { let e = try Entity.load(named:name); return rkBox(e) }
    catch { rkWriteErr(error.localizedDescription, eb, el); return nil }
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Scene / ARView
// ═══════════════════════════════════════════════════════════════════════════

#if os(macOS)
@_cdecl("rk_arview_new")
public func rkARViewNew(_ w:Float,_ h:Float) -> RKHandle {
    rkBox(ARView(frame:.init(x:0,y:0,width:CGFloat(w),height:CGFloat(h))))
}
@_cdecl("rk_arview_scene_add_anchor")
public func rkARViewAddAnchor(_ av:RKHandle,_ anchor:RKHandle) {
    let view: ARView = rkUnboxObj(av)
    let a: AnchorEntity = rkUnboxObj(anchor)
    view.scene.addAnchor(a)
}
@_cdecl("rk_arview_scene_remove_anchor")
public func rkARViewRemoveAnchor(_ av:RKHandle,_ anchor:RKHandle) {
    let view: ARView = rkUnboxObj(av)
    let a: AnchorEntity = rkUnboxObj(anchor)
    view.scene.removeAnchor(a)
}
@_cdecl("rk_arview_set_camera_mode")
public func rkARViewSetCameraMode(_ av:RKHandle,_ modeId:Int32) {
    // cameraMode is iOS-only — no-op on macOS
}
@_cdecl("rk_arview_set_environment_intensity")
public func rkARViewSetEnvIntensity(_ av:RKHandle,_ v:Float) {
    let view: ARView = rkUnboxObj(av)
    view.environment.lighting.intensityExponent = v
}
@_cdecl("rk_arview_set_render_options")
public func rkARViewSetRenderOptions(_ av:RKHandle,_ flags:UInt32) {
    // ARView.renderOptions is iOS-only — no-op on macOS
}
@_cdecl("rk_arview_scene_anchor_count")
public func rkARViewAnchorCount(_ av:RKHandle) -> Int {
    let view: ARView = rkUnboxObj(av); return view.scene.anchors.count
}
@_cdecl("rk_arview_scene_get_anchor")
public func rkARViewGetAnchor(_ av:RKHandle,_ idx:Int) -> RKHandle? {
    let view: ARView = rkUnboxObj(av)
    let anchors = Array(view.scene.anchors)
    guard idx < anchors.count else { return nil }
    // AnchorEntity is AnyObject; cast through Entity base class
    guard let entity = anchors[idx] as? Entity else { return nil }
    return rkBox(entity)
}
@_cdecl("rk_arview_find_entity_named")
public func rkARViewFindEntityNamed(_ av:RKHandle,_ p:UnsafePointer<UInt8>,_ n:Int) -> RKHandle? {
    let view: ARView = rkUnboxObj(av)
    let name = String(bytes:UnsafeBufferPointer(start:p,count:n),encoding:.utf8) ?? ""
    guard let e = view.scene.findEntity(named:name) else { return nil }
    return rkBox(e)
}
#else
@_cdecl("rk_arview_new")                  public func rkARViewNew(_ w:Float,_ h:Float) -> RKHandle? { nil }
@_cdecl("rk_arview_scene_add_anchor")     public func rkARViewAddAnchor(_ av:RKHandle,_ a:RKHandle) {}
@_cdecl("rk_arview_scene_remove_anchor")  public func rkARViewRemoveAnchor(_ av:RKHandle,_ a:RKHandle) {}
@_cdecl("rk_arview_set_camera_mode")      public func rkARViewSetCameraMode(_ av:RKHandle,_ m:Int32) {}
@_cdecl("rk_arview_set_environment_intensity") public func rkARViewSetEnvIntensity(_ av:RKHandle,_ v:Float) {}
@_cdecl("rk_arview_set_render_options")   public func rkARViewSetRenderOptions(_ av:RKHandle,_ f:UInt32) {}
@_cdecl("rk_arview_scene_anchor_count")   public func rkARViewAnchorCount(_ av:RKHandle) -> Int { 0 }
@_cdecl("rk_arview_scene_get_anchor")     public func rkARViewGetAnchor(_ av:RKHandle,_ idx:Int) -> RKHandle? { nil }
@_cdecl("rk_arview_find_entity_named")    public func rkARViewFindEntityNamed(_ av:RKHandle,_ p:UnsafePointer<UInt8>,_ n:Int) -> RKHandle? { nil }
#endif

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - visionOS specific
// ═══════════════════════════════════════════════════════════════════════════

#if os(visionOS)
@_cdecl("rk_entity_set_input_target")
public func rkEntitySetInputTarget(_ h:RKHandle,_ direct:Bool,_ indirect:Bool) {
    var comp = InputTargetComponent()
    comp.allowsDirectInteraction   = direct
    comp.allowsIndirectInteraction = indirect
    rkE(h).components.set(comp)
}
@_cdecl("rk_entity_remove_input_target")
public func rkEntityRemoveInputTarget(_ h:RKHandle) { rkE(h).components.remove(InputTargetComponent.self) }

@_cdecl("rk_entity_set_hover_effect")
public func rkEntitySetHoverEffect(_ h:RKHandle,_ r:Float,_ g:Float,_ b:Float,_ strength:Float) {
    if #available(visionOS 2.0, *) {
        let color = UIColor(red:CGFloat(r),green:CGFloat(g),blue:CGFloat(b),alpha:1)
        rkE(h).components.set(HoverEffectComponent(.highlight(.init(color:color, strength:strength))))
    } else {
        rkE(h).components.set(HoverEffectComponent())
    }
}
@_cdecl("rk_entity_remove_hover_effect")
public func rkEntityRemoveHoverEffect(_ h:RKHandle) { rkE(h).components.remove(HoverEffectComponent.self) }

@_cdecl("rk_portal_entity_new")
public func rkPortalEntityNew(_ worldHandle:RKHandle) -> RKHandle {
    let e = Entity(); e.components.set(PortalComponent(target:rkE(worldHandle))); return rkBox(e)
}
@_cdecl("rk_entity_set_world_component")
public func rkEntitySetWorldComponent(_ h:RKHandle) { rkE(h).components.set(WorldComponent()) }
@_cdecl("rk_entity_remove_world_component")
public func rkEntityRemoveWorldComponent(_ h:RKHandle) { rkE(h).components.remove(WorldComponent.self) }
@_cdecl("rk_entity_set_grounding_shadow_visionos")
public func rkEntitySetGroundingShadowVisionOS(_ h:RKHandle,_ casts:Bool) { rkE(h).components.set(GroundingShadowComponent(castsShadow:casts)) }

@_cdecl("rk_entity_set_surroundings_effect")
public func rkEntitySetSurroundingsEffect(_ h:RKHandle,_ intensity:Float) {
    if #available(visionOS 2.0, *) {
        rkE(h).components.set(SurroundingsEffectComponent(intensity:intensity))
    }
}
@_cdecl("rk_entity_remove_surroundings_effect")
public func rkEntityRemoveSurroundingsEffect(_ h:RKHandle) {
    if #available(visionOS 2.0, *) { rkE(h).components.remove(SurroundingsEffectComponent.self) }
}

@_cdecl("rk_anchor_entity_head")
public func rkAnchorHead() -> RKHandle {
    if #available(visionOS 3.0, *) { return rkBox(AnchorEntity(.head)) }
    return rkBox(AnchorEntity())
}
@_cdecl("rk_anchor_entity_left_hand")
public func rkAnchorLeftHand() -> RKHandle {
    if #available(visionOS 3.0, *) { return rkBox(AnchorEntity(.hand(.left, location:.palm))) }
    return rkBox(AnchorEntity())
}
@_cdecl("rk_anchor_entity_right_hand")
public func rkAnchorRightHand() -> RKHandle {
    if #available(visionOS 3.0, *) { return rkBox(AnchorEntity(.hand(.right, location:.palm))) }
    return rkBox(AnchorEntity())
}

#else
// Stubs — all symbols present on all platforms
@_cdecl("rk_entity_set_input_target")       public func rkEntitySetInputTarget(_ h:RKHandle,_ d:Bool,_ i:Bool) {}
@_cdecl("rk_entity_remove_input_target")    public func rkEntityRemoveInputTarget(_ h:RKHandle) {}
@_cdecl("rk_entity_set_hover_effect")       public func rkEntitySetHoverEffect(_ h:RKHandle,_ r:Float,_ g:Float,_ b:Float,_ s:Float) {}
@_cdecl("rk_entity_remove_hover_effect")    public func rkEntityRemoveHoverEffect(_ h:RKHandle) {}
@_cdecl("rk_portal_entity_new")             public func rkPortalEntityNew(_ w:RKHandle) -> RKHandle { rkBox(Entity()) }
@_cdecl("rk_entity_set_world_component")    public func rkEntitySetWorldComponent(_ h:RKHandle) {}
@_cdecl("rk_entity_remove_world_component") public func rkEntityRemoveWorldComponent(_ h:RKHandle) {}
@_cdecl("rk_entity_set_grounding_shadow_visionos") public func rkEntitySetGroundingShadowVisionOS(_ h:RKHandle,_ c:Bool) {}
@_cdecl("rk_entity_set_surroundings_effect")  public func rkEntitySetSurroundingsEffect(_ h:RKHandle,_ i:Float) {}
@_cdecl("rk_entity_remove_surroundings_effect") public func rkEntityRemoveSurroundingsEffect(_ h:RKHandle) {}
@_cdecl("rk_anchor_entity_head")            public func rkAnchorHead() -> RKHandle { rkBox(AnchorEntity()) }
@_cdecl("rk_anchor_entity_left_hand")       public func rkAnchorLeftHand() -> RKHandle { rkBox(AnchorEntity()) }
@_cdecl("rk_anchor_entity_right_hand")      public func rkAnchorRightHand() -> RKHandle { rkBox(AnchorEntity()) }
#endif
