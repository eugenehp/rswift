//! Apple Core Graphics (Quartz 2D) — 2D drawing from Rust.
//!
//! **Platform:** all Apple platforms.
//!
//! Contains a complete `ffi` module with ~350 raw C function declarations,
//! plus ergonomic Rust wrappers for the most common operations.
//!
//! ```ignore
//! use coregraphics::*;
//! let cs = ColorSpace::device_rgb();
//! let ctx = BitmapContext::new(100, 100, &cs);
//! ctx.set_fill_color_rgba(1.0, 0.0, 0.0, 1.0);
//! ctx.fill_rect(Rect::new(10.0, 10.0, 80.0, 80.0));
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

#![allow(non_snake_case, dead_code)]

use core::ffi::c_void;

/// Complete raw FFI to CoreGraphics C API (~350 functions).
/// Use these for anything the ergonomic wrappers don't cover.
pub mod ffi;

pub fn is_available() -> bool { true }

// ── Geometry types ──────────────────────────────────────────────────────────

pub type CGFloat = f64;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point { pub x: CGFloat, pub y: CGFloat }
impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn new(x: CGFloat, y: CGFloat) -> Self { Self { x, y } }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size { pub width: CGFloat, pub height: CGFloat }
impl Size {
    pub const ZERO: Self = Self { width: 0.0, height: 0.0 };
    pub fn new(w: CGFloat, h: CGFloat) -> Self { Self { width: w, height: h } }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect { pub origin: Point, pub size: Size }
impl Rect {
    pub const ZERO: Self = Self { origin: Point::ZERO, size: Size::ZERO };
    pub fn new(x: CGFloat, y: CGFloat, w: CGFloat, h: CGFloat) -> Self {
        Self { origin: Point { x, y }, size: Size { width: w, height: h } }
    }
    pub fn min_x(&self) -> CGFloat { unsafe { ffi::CGRectGetMinX(*self) } }
    pub fn mid_x(&self) -> CGFloat { unsafe { ffi::CGRectGetMidX(*self) } }
    pub fn max_x(&self) -> CGFloat { unsafe { ffi::CGRectGetMaxX(*self) } }
    pub fn min_y(&self) -> CGFloat { unsafe { ffi::CGRectGetMinY(*self) } }
    pub fn mid_y(&self) -> CGFloat { unsafe { ffi::CGRectGetMidY(*self) } }
    pub fn max_y(&self) -> CGFloat { unsafe { ffi::CGRectGetMaxY(*self) } }
    pub fn width(&self) -> CGFloat { unsafe { ffi::CGRectGetWidth(*self) } }
    pub fn height(&self) -> CGFloat { unsafe { ffi::CGRectGetHeight(*self) } }
    pub fn is_empty(&self) -> bool { unsafe { ffi::CGRectIsEmpty(*self) } }
    pub fn is_null(&self) -> bool { unsafe { ffi::CGRectIsNull(*self) } }
    pub fn is_infinite(&self) -> bool { unsafe { ffi::CGRectIsInfinite(*self) } }
    pub fn inset(&self, dx: CGFloat, dy: CGFloat) -> Rect { unsafe { ffi::CGRectInset(*self, dx, dy) } }
    pub fn offset(&self, dx: CGFloat, dy: CGFloat) -> Rect { unsafe { ffi::CGRectOffset(*self, dx, dy) } }
    pub fn integral(&self) -> Rect { unsafe { ffi::CGRectIntegral(*self) } }
    pub fn union(&self, other: Rect) -> Rect { unsafe { ffi::CGRectUnion(*self, other) } }
    pub fn intersection(&self, other: Rect) -> Rect { unsafe { ffi::CGRectIntersection(*self, other) } }
    pub fn contains_point(&self, p: Point) -> bool { unsafe { ffi::CGRectContainsPoint(*self, p) } }
    pub fn contains_rect(&self, other: Rect) -> bool { unsafe { ffi::CGRectContainsRect(*self, other) } }
    pub fn intersects(&self, other: Rect) -> bool { unsafe { ffi::CGRectIntersectsRect(*self, other) } }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AffineTransform {
    pub a: CGFloat, pub b: CGFloat,
    pub c: CGFloat, pub d: CGFloat,
    pub tx: CGFloat, pub ty: CGFloat,
}
impl AffineTransform {
    pub const IDENTITY: Self = Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 };
    pub fn translation(tx: CGFloat, ty: CGFloat) -> Self { unsafe { ffi::CGAffineTransformMakeTranslation(tx, ty) } }
    pub fn rotation(angle: CGFloat) -> Self { unsafe { ffi::CGAffineTransformMakeRotation(angle) } }
    pub fn scale(sx: CGFloat, sy: CGFloat) -> Self { unsafe { ffi::CGAffineTransformMakeScale(sx, sy) } }
    pub fn is_identity(&self) -> bool { unsafe { ffi::CGAffineTransformIsIdentity(*self) } }
    pub fn translate(self, tx: CGFloat, ty: CGFloat) -> Self { unsafe { ffi::CGAffineTransformTranslate(self, tx, ty) } }
    pub fn scaled(self, sx: CGFloat, sy: CGFloat) -> Self { unsafe { ffi::CGAffineTransformScale(self, sx, sy) } }
    pub fn rotated(self, angle: CGFloat) -> Self { unsafe { ffi::CGAffineTransformRotate(self, angle) } }
    pub fn invert(self) -> Self { unsafe { ffi::CGAffineTransformInvert(self) } }
    pub fn concat(self, other: Self) -> Self { unsafe { ffi::CGAffineTransformConcat(self, other) } }
    pub fn eq(&self, other: &Self) -> bool { unsafe { ffi::CGAffineTransformEqualToTransform(*self, *other) } }
    pub fn apply_to_point(self, p: Point) -> Point { unsafe { ffi::CGPointApplyAffineTransform(p, self) } }
    pub fn apply_to_size(self, s: Size) -> Size { unsafe { ffi::CGSizeApplyAffineTransform(s, self) } }
    pub fn apply_to_rect(self, r: Rect) -> Rect { unsafe { ffi::CGRectApplyAffineTransform(r, self) } }
}

// ── Blend modes ─────────────────────────────────────────────────────────────

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal = 0, Multiply = 1, Screen = 2, Overlay = 3,
    Darken = 4, Lighten = 5, ColorDodge = 6, ColorBurn = 7,
    SoftLight = 8, HardLight = 9, Difference = 10, Exclusion = 11,
    Hue = 12, Saturation = 13, Color = 14, Luminosity = 15,
    Clear = 16, Copy = 17, SourceIn = 18, SourceOut = 19,
    SourceAtop = 20, DestinationOver = 21, DestinationIn = 22,
    DestinationOut = 23, DestinationAtop = 24, Xor = 25,
    PlusDarker = 26, PlusLighter = 27,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap { Butt = 0, Round = 1, Square = 2 }

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin { Miter = 0, Round = 1, Bevel = 2 }

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathDrawingMode { Fill = 0, EOFill = 1, Stroke = 2, FillStroke = 3, EOFillStroke = 4 }

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationQuality { Default = 0, None = 1, Low = 2, Medium = 4, High = 3 }

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpaceModel { Unknown = -1, Monochrome = 0, RGB = 1, CMYK = 2, Lab = 3, DeviceN = 4, Indexed = 5, Pattern = 6 }

// ── ColorSpace ──────────────────────────────────────────────────────────────

pub struct ColorSpace(ffi::CGColorSpaceRef);
impl ColorSpace {
    pub fn device_rgb() -> Self { Self(unsafe { ffi::CGColorSpaceCreateDeviceRGB() }) }
    pub fn device_gray() -> Self { Self(unsafe { ffi::CGColorSpaceCreateDeviceGray() }) }
    pub fn device_cmyk() -> Self { Self(unsafe { ffi::CGColorSpaceCreateDeviceCMYK() }) }
    pub fn num_components(&self) -> usize { unsafe { ffi::CGColorSpaceGetNumberOfComponents(self.0) } }
    pub fn model(&self) -> ColorSpaceModel { unsafe { core::mem::transmute(ffi::CGColorSpaceGetModel(self.0)) } }
    pub fn is_wide_gamut(&self) -> bool { unsafe { ffi::CGColorSpaceIsWideGamutRGB(self.0) } }
    pub fn as_ptr(&self) -> *mut c_void { self.0 }
}
impl Drop for ColorSpace { fn drop(&mut self) { unsafe { ffi::CGColorSpaceRelease(self.0); } } }

// ── Color ───────────────────────────────────────────────────────────────────

pub struct Color(ffi::CGColorRef);
impl Color {
    pub fn srgb(r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> Self { Self(unsafe { ffi::CGColorCreateSRGB(r, g, b, a) }) }
    pub fn gray(gray: CGFloat, alpha: CGFloat) -> Self { Self(unsafe { ffi::CGColorCreateGenericGray(gray, alpha) }) }
    pub fn rgb(r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> Self { Self(unsafe { ffi::CGColorCreateGenericRGB(r, g, b, a) }) }
    pub fn alpha(&self) -> CGFloat { unsafe { ffi::CGColorGetAlpha(self.0) } }
    pub fn num_components(&self) -> usize { unsafe { ffi::CGColorGetNumberOfComponents(self.0) } }
    pub fn components(&self) -> &[CGFloat] {
        unsafe {
            let ptr = ffi::CGColorGetComponents(self.0);
            let n = self.num_components() + 1; // includes alpha
            core::slice::from_raw_parts(ptr, n)
        }
    }
    pub fn as_ptr(&self) -> *mut c_void { self.0 }
}
impl Clone for Color { fn clone(&self) -> Self { Self(unsafe { ffi::CGColorCreateCopy(self.0) }) } }
impl Drop for Color { fn drop(&mut self) { unsafe { ffi::CGColorRelease(self.0); } } }

// ── BitmapContext ───────────────────────────────────────────────────────────

pub struct BitmapContext(ffi::CGContextRef);
const RGBA_PREMUL: u32 = 1;

impl BitmapContext {
    pub fn new(width: usize, height: usize, color_space: &ColorSpace) -> Self {
        let ctx = unsafe { ffi::CGBitmapContextCreate(core::ptr::null_mut(), width, height, 8, width * 4, color_space.0, RGBA_PREMUL) };
        assert!(!ctx.is_null(), "Failed to create CGBitmapContext");
        Self(ctx)
    }
    pub fn width(&self) -> usize { unsafe { ffi::CGBitmapContextGetWidth(self.0) } }
    pub fn height(&self) -> usize { unsafe { ffi::CGBitmapContextGetHeight(self.0) } }
    pub fn bits_per_component(&self) -> usize { unsafe { ffi::CGBitmapContextGetBitsPerComponent(self.0) } }
    pub fn bytes_per_row(&self) -> usize { unsafe { ffi::CGBitmapContextGetBytesPerRow(self.0) } }
    pub fn data(&self) -> *mut c_void { unsafe { ffi::CGBitmapContextGetData(self.0) } }
    pub fn create_image(&self) -> Image { Image(unsafe { ffi::CGBitmapContextCreateImage(self.0) }) }

    // State
    pub fn save(&self) { unsafe { ffi::CGContextSaveGState(self.0); } }
    pub fn restore(&self) { unsafe { ffi::CGContextRestoreGState(self.0); } }
    pub fn flush(&self) { unsafe { ffi::CGContextFlush(self.0); } }

    // Transforms
    pub fn translate(&self, tx: CGFloat, ty: CGFloat) { unsafe { ffi::CGContextTranslateCTM(self.0, tx, ty); } }
    pub fn scale(&self, sx: CGFloat, sy: CGFloat) { unsafe { ffi::CGContextScaleCTM(self.0, sx, sy); } }
    pub fn rotate(&self, angle: CGFloat) { unsafe { ffi::CGContextRotateCTM(self.0, angle); } }
    pub fn concat(&self, t: AffineTransform) { unsafe { ffi::CGContextConcatCTM(self.0, t); } }
    pub fn ctm(&self) -> AffineTransform { unsafe { ffi::CGContextGetCTM(self.0) } }

    // Attributes
    pub fn set_alpha(&self, alpha: CGFloat) { unsafe { ffi::CGContextSetAlpha(self.0, alpha); } }
    pub fn set_line_width(&self, w: CGFloat) { unsafe { ffi::CGContextSetLineWidth(self.0, w); } }
    pub fn set_line_cap(&self, cap: LineCap) { unsafe { ffi::CGContextSetLineCap(self.0, cap as i32); } }
    pub fn set_line_join(&self, join: LineJoin) { unsafe { ffi::CGContextSetLineJoin(self.0, join as i32); } }
    pub fn set_miter_limit(&self, limit: CGFloat) { unsafe { ffi::CGContextSetMiterLimit(self.0, limit); } }
    pub fn set_flatness(&self, flatness: CGFloat) { unsafe { ffi::CGContextSetFlatness(self.0, flatness); } }
    pub fn set_blend_mode(&self, mode: BlendMode) { unsafe { ffi::CGContextSetBlendMode(self.0, mode as i32); } }
    pub fn set_should_antialias(&self, v: bool) { unsafe { ffi::CGContextSetShouldAntialias(self.0, v); } }
    pub fn set_interpolation_quality(&self, q: InterpolationQuality) { unsafe { ffi::CGContextSetInterpolationQuality(self.0, q as i32); } }

    // Colors
    pub fn set_fill_color_rgba(&self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) { unsafe { ffi::CGContextSetRGBFillColor(self.0, r, g, b, a); } }
    pub fn set_stroke_color_rgba(&self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) { unsafe { ffi::CGContextSetRGBStrokeColor(self.0, r, g, b, a); } }
    pub fn set_gray_fill(&self, gray: CGFloat, alpha: CGFloat) { unsafe { ffi::CGContextSetGrayFillColor(self.0, gray, alpha); } }
    pub fn set_gray_stroke(&self, gray: CGFloat, alpha: CGFloat) { unsafe { ffi::CGContextSetGrayStrokeColor(self.0, gray, alpha); } }
    pub fn set_fill_color(&self, color: &Color) { unsafe { ffi::CGContextSetFillColorWithColor(self.0, color.0); } }
    pub fn set_stroke_color(&self, color: &Color) { unsafe { ffi::CGContextSetStrokeColorWithColor(self.0, color.0); } }

    // Paths
    pub fn begin_path(&self) { unsafe { ffi::CGContextBeginPath(self.0); } }
    pub fn close_path(&self) { unsafe { ffi::CGContextClosePath(self.0); } }
    pub fn move_to(&self, x: CGFloat, y: CGFloat) { unsafe { ffi::CGContextMoveToPoint(self.0, x, y); } }
    pub fn line_to(&self, x: CGFloat, y: CGFloat) { unsafe { ffi::CGContextAddLineToPoint(self.0, x, y); } }
    pub fn curve_to(&self, cp1x: CGFloat, cp1y: CGFloat, cp2x: CGFloat, cp2y: CGFloat, x: CGFloat, y: CGFloat) { unsafe { ffi::CGContextAddCurveToPoint(self.0, cp1x, cp1y, cp2x, cp2y, x, y); } }
    pub fn quad_curve_to(&self, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat) { unsafe { ffi::CGContextAddQuadCurveToPoint(self.0, cpx, cpy, x, y); } }
    pub fn add_rect(&self, r: Rect) { unsafe { ffi::CGContextAddRect(self.0, r); } }
    pub fn add_rects(&self, rects: &[Rect]) { unsafe { ffi::CGContextAddRects(self.0, rects.as_ptr(), rects.len()); } }
    pub fn add_lines(&self, points: &[Point]) { unsafe { ffi::CGContextAddLines(self.0, points.as_ptr(), points.len()); } }
    pub fn add_ellipse(&self, r: Rect) { unsafe { ffi::CGContextAddEllipseInRect(self.0, r); } }
    pub fn add_arc(&self, x: CGFloat, y: CGFloat, radius: CGFloat, start: CGFloat, end: CGFloat, clockwise: bool) { unsafe { ffi::CGContextAddArc(self.0, x, y, radius, start, end, clockwise as i32); } }
    pub fn add_arc_to(&self, x1: CGFloat, y1: CGFloat, x2: CGFloat, y2: CGFloat, radius: CGFloat) { unsafe { ffi::CGContextAddArcToPoint(self.0, x1, y1, x2, y2, radius); } }
    pub fn add_path(&self, path: &Path) { unsafe { ffi::CGContextAddPath(self.0, path.0); } }
    pub fn is_path_empty(&self) -> bool { unsafe { ffi::CGContextIsPathEmpty(self.0) } }
    pub fn path_current_point(&self) -> Point { unsafe { ffi::CGContextGetPathCurrentPoint(self.0) } }
    pub fn path_bounding_box(&self) -> Rect { unsafe { ffi::CGContextGetPathBoundingBox(self.0) } }
    pub fn replace_path_with_stroked(&self) { unsafe { ffi::CGContextReplacePathWithStrokedPath(self.0); } }

    // Drawing
    pub fn draw_path(&self, mode: PathDrawingMode) { unsafe { ffi::CGContextDrawPath(self.0, mode as i32); } }
    pub fn fill_path(&self) { unsafe { ffi::CGContextFillPath(self.0); } }
    pub fn eo_fill_path(&self) { unsafe { ffi::CGContextEOFillPath(self.0); } }
    pub fn stroke_path(&self) { unsafe { ffi::CGContextStrokePath(self.0); } }
    pub fn fill_rect(&self, r: Rect) { unsafe { ffi::CGContextFillRect(self.0, r); } }
    pub fn fill_rects(&self, rects: &[Rect]) { unsafe { ffi::CGContextFillRects(self.0, rects.as_ptr(), rects.len()); } }
    pub fn stroke_rect(&self, r: Rect) { unsafe { ffi::CGContextStrokeRect(self.0, r); } }
    pub fn stroke_rect_with_width(&self, r: Rect, w: CGFloat) { unsafe { ffi::CGContextStrokeRectWithWidth(self.0, r, w); } }
    pub fn clear_rect(&self, r: Rect) { unsafe { ffi::CGContextClearRect(self.0, r); } }
    pub fn fill_ellipse(&self, r: Rect) { unsafe { ffi::CGContextFillEllipseInRect(self.0, r); } }
    pub fn stroke_ellipse(&self, r: Rect) { unsafe { ffi::CGContextStrokeEllipseInRect(self.0, r); } }
    pub fn stroke_line_segments(&self, points: &[Point]) { unsafe { ffi::CGContextStrokeLineSegments(self.0, points.as_ptr(), points.len()); } }

    // Clipping
    pub fn clip(&self) { unsafe { ffi::CGContextClip(self.0); } }
    pub fn eo_clip(&self) { unsafe { ffi::CGContextEOClip(self.0); } }
    pub fn clip_to_rect(&self, r: Rect) { unsafe { ffi::CGContextClipToRect(self.0, r); } }
    pub fn clip_to_rects(&self, rects: &[Rect]) { unsafe { ffi::CGContextClipToRects(self.0, rects.as_ptr(), rects.len()); } }
    pub fn clip_bounding_box(&self) -> Rect { unsafe { ffi::CGContextGetClipBoundingBox(self.0) } }

    // Shadow
    pub fn set_shadow(&self, offset: Size, blur: CGFloat) { unsafe { ffi::CGContextSetShadow(self.0, offset, blur); } }
    pub fn set_shadow_with_color(&self, offset: Size, blur: CGFloat, color: &Color) { unsafe { ffi::CGContextSetShadowWithColor(self.0, offset, blur, color.0); } }

    // Gradient
    pub fn draw_linear_gradient(&self, g: &Gradient, start: Point, end: Point, options: u32) { unsafe { ffi::CGContextDrawLinearGradient(self.0, g.0, start, end, options); } }
    pub fn draw_radial_gradient(&self, g: &Gradient, start: Point, sr: CGFloat, end: Point, er: CGFloat, options: u32) { unsafe { ffi::CGContextDrawRadialGradient(self.0, g.0, start, sr, end, er, options); } }

    // Images
    pub fn draw_image(&self, r: Rect, image: &Image) { unsafe { ffi::CGContextDrawImage(self.0, r, image.0); } }
    pub fn draw_tiled_image(&self, r: Rect, image: &Image) { unsafe { ffi::CGContextDrawTiledImage(self.0, r, image.0); } }

    // Transparency layers
    pub fn begin_transparency_layer(&self) { unsafe { ffi::CGContextBeginTransparencyLayer(self.0, core::ptr::null()); } }
    pub fn end_transparency_layer(&self) { unsafe { ffi::CGContextEndTransparencyLayer(self.0); } }

    // Text
    pub fn set_text_position(&self, x: CGFloat, y: CGFloat) { unsafe { ffi::CGContextSetTextPosition(self.0, x, y); } }
    pub fn text_position(&self) -> Point { unsafe { ffi::CGContextGetTextPosition(self.0) } }
    pub fn set_text_matrix(&self, t: AffineTransform) { unsafe { ffi::CGContextSetTextMatrix(self.0, t); } }
    pub fn set_font_size(&self, size: CGFloat) { unsafe { ffi::CGContextSetFontSize(self.0, size); } }
    pub fn set_character_spacing(&self, spacing: CGFloat) { unsafe { ffi::CGContextSetCharacterSpacing(self.0, spacing); } }

    // PDF
    pub fn draw_pdf_page(&self, page: *mut c_void) { unsafe { ffi::CGContextDrawPDFPage(self.0, page); } }

    // Coordinate conversion
    pub fn user_to_device_transform(&self) -> AffineTransform { unsafe { ffi::CGContextGetUserSpaceToDeviceSpaceTransform(self.0) } }
    pub fn point_to_device(&self, p: Point) -> Point { unsafe { ffi::CGContextConvertPointToDeviceSpace(self.0, p) } }
    pub fn point_to_user(&self, p: Point) -> Point { unsafe { ffi::CGContextConvertPointToUserSpace(self.0, p) } }
    pub fn rect_to_device(&self, r: Rect) -> Rect { unsafe { ffi::CGContextConvertRectToDeviceSpace(self.0, r) } }
    pub fn rect_to_user(&self, r: Rect) -> Rect { unsafe { ffi::CGContextConvertRectToUserSpace(self.0, r) } }

    pub fn as_ptr(&self) -> *mut c_void { self.0 }
}
impl Drop for BitmapContext { fn drop(&mut self) { unsafe { ffi::CGContextRelease(self.0); } } }

// ── Image ───────────────────────────────────────────────────────────────────

pub struct Image(ffi::CGImageRef);
impl Image {
    pub fn width(&self) -> usize { unsafe { ffi::CGImageGetWidth(self.0) } }
    pub fn height(&self) -> usize { unsafe { ffi::CGImageGetHeight(self.0) } }
    pub fn bits_per_component(&self) -> usize { unsafe { ffi::CGImageGetBitsPerComponent(self.0) } }
    pub fn bits_per_pixel(&self) -> usize { unsafe { ffi::CGImageGetBitsPerPixel(self.0) } }
    pub fn bytes_per_row(&self) -> usize { unsafe { ffi::CGImageGetBytesPerRow(self.0) } }
    pub fn is_mask(&self) -> bool { unsafe { ffi::CGImageIsMask(self.0) } }
    pub fn crop(&self, rect: Rect) -> Image { Image(unsafe { ffi::CGImageCreateWithImageInRect(self.0, rect) }) }
    pub fn as_ptr(&self) -> *mut c_void { self.0 }
}
impl Drop for Image { fn drop(&mut self) { if !self.0.is_null() { unsafe { ffi::CGImageRelease(self.0); } } } }

// ── Path ────────────────────────────────────────────────────────────────────

pub struct Path(ffi::CGMutablePathRef);
impl Path {
    pub fn new() -> Self { Self(unsafe { ffi::CGPathCreateMutable() }) }
    pub fn with_rect(rect: Rect) -> Self { Self(unsafe { ffi::CGPathCreateWithRect(rect, core::ptr::null()) as ffi::CGMutablePathRef }) }
    pub fn with_ellipse(rect: Rect) -> Self { Self(unsafe { ffi::CGPathCreateWithEllipseInRect(rect, core::ptr::null()) as ffi::CGMutablePathRef }) }
    pub fn with_rounded_rect(rect: Rect, cw: CGFloat, ch: CGFloat) -> Self { Self(unsafe { ffi::CGPathCreateWithRoundedRect(rect, cw, ch, core::ptr::null()) as ffi::CGMutablePathRef }) }

    pub fn move_to(&self, x: CGFloat, y: CGFloat) { unsafe { ffi::CGPathMoveToPoint(self.0, core::ptr::null(), x, y); } }
    pub fn line_to(&self, x: CGFloat, y: CGFloat) { unsafe { ffi::CGPathAddLineToPoint(self.0, core::ptr::null(), x, y); } }
    pub fn curve_to(&self, cp1x: CGFloat, cp1y: CGFloat, cp2x: CGFloat, cp2y: CGFloat, x: CGFloat, y: CGFloat) { unsafe { ffi::CGPathAddCurveToPoint(self.0, core::ptr::null(), cp1x, cp1y, cp2x, cp2y, x, y); } }
    pub fn quad_curve_to(&self, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat) { unsafe { ffi::CGPathAddQuadCurveToPoint(self.0, core::ptr::null(), cpx, cpy, x, y); } }
    pub fn close(&self) { unsafe { ffi::CGPathCloseSubpath(self.0); } }
    pub fn add_rect(&self, r: Rect) { unsafe { ffi::CGPathAddRect(self.0, core::ptr::null(), r); } }
    pub fn add_rects(&self, rects: &[Rect]) { unsafe { ffi::CGPathAddRects(self.0, core::ptr::null(), rects.as_ptr(), rects.len()); } }
    pub fn add_lines(&self, points: &[Point]) { unsafe { ffi::CGPathAddLines(self.0, core::ptr::null(), points.as_ptr(), points.len()); } }
    pub fn add_ellipse(&self, r: Rect) { unsafe { ffi::CGPathAddEllipseInRect(self.0, core::ptr::null(), r); } }
    pub fn add_arc(&self, x: CGFloat, y: CGFloat, radius: CGFloat, start: CGFloat, end: CGFloat, clockwise: bool) { unsafe { ffi::CGPathAddArc(self.0, core::ptr::null(), x, y, radius, start, end, clockwise); } }
    pub fn add_rounded_rect(&self, rect: Rect, cw: CGFloat, ch: CGFloat) { unsafe { ffi::CGPathAddRoundedRect(self.0, core::ptr::null(), rect, cw, ch); } }
    pub fn add_path(&self, other: &Path) { unsafe { ffi::CGPathAddPath(self.0, core::ptr::null(), other.0); } }

    pub fn is_empty(&self) -> bool { unsafe { ffi::CGPathIsEmpty(self.0) } }
    pub fn current_point(&self) -> Point { unsafe { ffi::CGPathGetCurrentPoint(self.0) } }
    pub fn bounding_box(&self) -> Rect { unsafe { ffi::CGPathGetBoundingBox(self.0) } }
    pub fn contains_point(&self, point: Point, eo_fill: bool) -> bool { unsafe { ffi::CGPathContainsPoint(self.0, core::ptr::null(), point, eo_fill) } }

    pub fn as_ptr(&self) -> *mut c_void { self.0 as *mut c_void }
}
impl Default for Path { fn default() -> Self { Self::new() } }
impl Drop for Path { fn drop(&mut self) { unsafe { ffi::CGPathRelease(self.0); } } }

// ── Gradient ────────────────────────────────────────────────────────────────

pub struct Gradient(ffi::CGGradientRef);
impl Gradient {
    /// Create from RGBA color components and locations (0.0–1.0).
    /// `components` is [r,g,b,a, r,g,b,a, ...], `locations` is [0.0, ..., 1.0].
    pub fn with_colors(color_space: &ColorSpace, components: &[CGFloat], locations: &[CGFloat]) -> Self {
        Self(unsafe { ffi::CGGradientCreateWithColorComponents(color_space.0, components.as_ptr(), locations.as_ptr(), locations.len()) })
    }
    pub fn as_ptr(&self) -> *mut c_void { self.0 }
}
impl Drop for Gradient { fn drop(&mut self) { unsafe { ffi::CGGradientRelease(self.0); } } }

/// Gradient drawing options.
pub mod gradient_options {
    pub const DRAWS_BEFORE_START: u32 = 1;
    pub const DRAWS_AFTER_END: u32 = 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_context() {
        let cs = ColorSpace::device_rgb();
        assert_eq!(cs.num_components(), 3);
        let ctx = BitmapContext::new(64, 64, &cs);
        assert_eq!(ctx.width(), 64);
        assert_eq!(ctx.height(), 64);
        assert_eq!(ctx.bits_per_component(), 8);
    }

    #[test]
    fn test_draw_and_capture() {
        let cs = ColorSpace::device_rgb();
        let ctx = BitmapContext::new(100, 100, &cs);
        ctx.set_fill_color_rgba(1.0, 0.0, 0.0, 1.0);
        ctx.fill_rect(Rect::new(0.0, 0.0, 100.0, 100.0));
        let img = ctx.create_image();
        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);
        assert!(!img.is_mask());
    }

    #[test]
    fn test_rect_ops() {
        let r = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(r.min_x(), 10.0);
        assert_eq!(r.max_x(), 110.0);
        assert_eq!(r.width(), 100.0);
        assert!(!r.is_empty());

        let inset = r.inset(5.0, 10.0);
        assert_eq!(inset.width(), 90.0);
        assert_eq!(inset.height(), 180.0);

        let r2 = Rect::new(50.0, 50.0, 200.0, 200.0);
        assert!(r.intersects(r2));
        assert!(r.union(r2).width() > 200.0);
    }

    #[test]
    fn test_path() {
        let p = Path::new();
        assert!(p.is_empty());
        p.move_to(0.0, 0.0);
        p.line_to(100.0, 0.0);
        p.line_to(100.0, 100.0);
        p.close();
        assert!(!p.is_empty());
        assert!(p.contains_point(Point::new(50.0, 50.0), false));
        assert!(!p.contains_point(Point::new(150.0, 50.0), false));
    }

    #[test]
    fn test_path_constructors() {
        let r = Path::with_rect(Rect::new(0.0, 0.0, 50.0, 50.0));
        assert!(!r.is_empty());
        let e = Path::with_ellipse(Rect::new(0.0, 0.0, 50.0, 50.0));
        assert!(!e.is_empty());
        let rr = Path::with_rounded_rect(Rect::new(0.0, 0.0, 50.0, 50.0), 5.0, 5.0);
        assert!(!rr.is_empty());
    }

    #[test]
    fn test_affine_transform() {
        let t = AffineTransform::translation(10.0, 20.0);
        assert!(!t.is_identity());
        let p = t.apply_to_point(Point::ZERO);
        assert!((p.x - 10.0).abs() < 0.001);
        assert!((p.y - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_color() {
        let c = Color::srgb(1.0, 0.0, 0.0, 1.0);
        // sRGB has 3 color components + 1 alpha = 4 total
        assert_eq!(c.num_components(), 4);
        assert!((c.alpha() - 1.0).abs() < 0.001);
        let comps = c.components();
        assert!((comps[0] - 1.0).abs() < 0.01); // red
        assert!((comps[1]).abs() < 0.01); // green
    }

    #[test]
    fn test_gradient() {
        let cs = ColorSpace::device_rgb();
        let colors = [1.0, 0.0, 0.0, 1.0,  0.0, 0.0, 1.0, 1.0]; // red → blue
        let locs = [0.0, 1.0];
        let g = Gradient::with_colors(&cs, &colors, &locs);
        assert!(!g.as_ptr().is_null());
    }

    #[test]
    fn test_full_drawing() {
        let cs = ColorSpace::device_rgb();
        let ctx = BitmapContext::new(200, 200, &cs);

        // Background
        ctx.set_fill_color_rgba(1.0, 1.0, 1.0, 1.0);
        ctx.fill_rect(Rect::new(0.0, 0.0, 200.0, 200.0));

        // Red circle
        ctx.set_fill_color_rgba(1.0, 0.0, 0.0, 1.0);
        ctx.fill_ellipse(Rect::new(25.0, 25.0, 150.0, 150.0));

        // Blue stroked rect
        ctx.set_stroke_color_rgba(0.0, 0.0, 1.0, 1.0);
        ctx.set_line_width(3.0);
        ctx.stroke_rect(Rect::new(10.0, 10.0, 180.0, 180.0));

        // Gradient
        let gradient = Gradient::with_colors(&cs,
            &[0.0, 1.0, 0.0, 0.5,  1.0, 1.0, 0.0, 0.5],
            &[0.0, 1.0]);
        ctx.save();
        ctx.clip_to_rect(Rect::new(50.0, 50.0, 100.0, 100.0));
        ctx.draw_linear_gradient(&gradient, Point::new(50.0, 50.0), Point::new(150.0, 150.0), 0);
        ctx.restore();

        // Path
        let p = Path::new();
        p.move_to(100.0, 10.0);
        p.line_to(190.0, 190.0);
        p.line_to(10.0, 190.0);
        p.close();
        ctx.set_stroke_color_rgba(0.0, 0.5, 0.0, 1.0);
        ctx.set_line_width(2.0);
        ctx.add_path(&p);
        ctx.stroke_path();

        let img = ctx.create_image();
        assert_eq!(img.width(), 200);
    }

    #[test]
    fn test_image_crop() {
        let cs = ColorSpace::device_rgb();
        let ctx = BitmapContext::new(100, 100, &cs);
        ctx.set_fill_color_rgba(1.0, 0.0, 0.0, 1.0);
        ctx.fill_rect(Rect::new(0.0, 0.0, 100.0, 100.0));
        let img = ctx.create_image();
        let cropped = img.crop(Rect::new(10.0, 10.0, 50.0, 50.0));
        assert_eq!(cropped.width(), 50);
        assert_eq!(cropped.height(), 50);
    }
}
