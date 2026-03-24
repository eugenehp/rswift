//! Raw CoreGraphics C FFI — complete extern declarations.
//!
//! Auto-assisted from CoreGraphics headers, hand-verified for type correctness.
//! All functions link directly to the CoreGraphics framework — no bridge needed.

#![allow(non_snake_case, dead_code)]

use core::ffi::c_void;
use super::{CGFloat, Point, Size, Rect, AffineTransform};

// Opaque ref types
pub type CGContextRef = *mut c_void;
pub type CGColorRef = *mut c_void;
pub type CGColorSpaceRef = *mut c_void;
pub type CGImageRef = *mut c_void;
pub type CGPathRef = *const c_void;
pub type CGMutablePathRef = *mut c_void;
pub type CGGradientRef = *mut c_void;
pub type CGFontRef = *mut c_void;
pub type CGLayerRef = *mut c_void;
pub type CGPatternRef = *mut c_void;
pub type CGShadingRef = *mut c_void;
pub type CGDataProviderRef = *mut c_void;
pub type CGDataConsumerRef = *mut c_void;
pub type CGFunctionRef = *mut c_void;
pub type CGPDFDocumentRef = *mut c_void;
pub type CGPDFPageRef = *mut c_void;
pub type CFStringRef = *const c_void;
pub type CFDataRef = *const c_void;
pub type CFURLRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFArrayRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFTypeID = u64;

// ── CGAffineTransform (17 functions) ────────────────────────────────────────

unsafe extern "C" {
    pub fn CGAffineTransformMake(a: CGFloat, b: CGFloat, c: CGFloat, d: CGFloat, tx: CGFloat, ty: CGFloat) -> AffineTransform;
    pub fn CGAffineTransformMakeTranslation(tx: CGFloat, ty: CGFloat) -> AffineTransform;
    pub fn CGAffineTransformMakeScale(sx: CGFloat, sy: CGFloat) -> AffineTransform;
    pub fn CGAffineTransformMakeRotation(angle: CGFloat) -> AffineTransform;
    pub fn CGAffineTransformIsIdentity(t: AffineTransform) -> bool;
    pub fn CGAffineTransformTranslate(t: AffineTransform, tx: CGFloat, ty: CGFloat) -> AffineTransform;
    pub fn CGAffineTransformScale(t: AffineTransform, sx: CGFloat, sy: CGFloat) -> AffineTransform;
    pub fn CGAffineTransformRotate(t: AffineTransform, angle: CGFloat) -> AffineTransform;
    pub fn CGAffineTransformInvert(t: AffineTransform) -> AffineTransform;
    pub fn CGAffineTransformConcat(t1: AffineTransform, t2: AffineTransform) -> AffineTransform;
    pub fn CGAffineTransformEqualToTransform(t1: AffineTransform, t2: AffineTransform) -> bool;
    pub fn CGPointApplyAffineTransform(point: Point, t: AffineTransform) -> Point;
    pub fn CGSizeApplyAffineTransform(size: Size, t: AffineTransform) -> Size;
    pub fn CGRectApplyAffineTransform(rect: Rect, t: AffineTransform) -> Rect;
}

// ── CGGeometry (35 functions) ───────────────────────────────────────────────

unsafe extern "C" {
    pub fn CGRectGetMinX(rect: Rect) -> CGFloat;
    pub fn CGRectGetMidX(rect: Rect) -> CGFloat;
    pub fn CGRectGetMaxX(rect: Rect) -> CGFloat;
    pub fn CGRectGetMinY(rect: Rect) -> CGFloat;
    pub fn CGRectGetMidY(rect: Rect) -> CGFloat;
    pub fn CGRectGetMaxY(rect: Rect) -> CGFloat;
    pub fn CGRectGetWidth(rect: Rect) -> CGFloat;
    pub fn CGRectGetHeight(rect: Rect) -> CGFloat;
    pub fn CGRectEqualToRect(r1: Rect, r2: Rect) -> bool;
    pub fn CGRectStandardize(rect: Rect) -> Rect;
    pub fn CGRectIsEmpty(rect: Rect) -> bool;
    pub fn CGRectIsNull(rect: Rect) -> bool;
    pub fn CGRectIsInfinite(rect: Rect) -> bool;
    pub fn CGRectInset(rect: Rect, dx: CGFloat, dy: CGFloat) -> Rect;
    pub fn CGRectIntegral(rect: Rect) -> Rect;
    pub fn CGRectUnion(r1: Rect, r2: Rect) -> Rect;
    pub fn CGRectIntersection(r1: Rect, r2: Rect) -> Rect;
    pub fn CGRectOffset(rect: Rect, dx: CGFloat, dy: CGFloat) -> Rect;
    pub fn CGRectContainsPoint(rect: Rect, point: Point) -> bool;
    pub fn CGRectContainsRect(r1: Rect, r2: Rect) -> bool;
    pub fn CGRectIntersectsRect(r1: Rect, r2: Rect) -> bool;
    pub fn CGPointEqualToPoint(p1: Point, p2: Point) -> bool;
    pub fn CGSizeEqualToSize(s1: Size, s2: Size) -> bool;
}

// ── CGContext (127 functions) ───────────────────────────────────────────────

unsafe extern "C" {
    pub fn CGContextGetTypeID() -> CFTypeID;
    pub fn CGContextSaveGState(c: CGContextRef);
    pub fn CGContextRestoreGState(c: CGContextRef);
    pub fn CGContextScaleCTM(c: CGContextRef, sx: CGFloat, sy: CGFloat);
    pub fn CGContextTranslateCTM(c: CGContextRef, tx: CGFloat, ty: CGFloat);
    pub fn CGContextRotateCTM(c: CGContextRef, angle: CGFloat);
    pub fn CGContextConcatCTM(c: CGContextRef, transform: AffineTransform);
    pub fn CGContextGetCTM(c: CGContextRef) -> AffineTransform;
    pub fn CGContextSetLineWidth(c: CGContextRef, width: CGFloat);
    pub fn CGContextSetLineCap(c: CGContextRef, cap: i32);
    pub fn CGContextSetLineJoin(c: CGContextRef, join: i32);
    pub fn CGContextSetMiterLimit(c: CGContextRef, limit: CGFloat);
    pub fn CGContextSetLineDash(c: CGContextRef, phase: CGFloat, lengths: *const CGFloat, count: usize);
    pub fn CGContextSetFlatness(c: CGContextRef, flatness: CGFloat);
    pub fn CGContextSetAlpha(c: CGContextRef, alpha: CGFloat);
    pub fn CGContextSetBlendMode(c: CGContextRef, mode: i32);
    pub fn CGContextBeginPath(c: CGContextRef);
    pub fn CGContextMoveToPoint(c: CGContextRef, x: CGFloat, y: CGFloat);
    pub fn CGContextAddLineToPoint(c: CGContextRef, x: CGFloat, y: CGFloat);
    pub fn CGContextAddCurveToPoint(c: CGContextRef, cp1x: CGFloat, cp1y: CGFloat, cp2x: CGFloat, cp2y: CGFloat, x: CGFloat, y: CGFloat);
    pub fn CGContextAddQuadCurveToPoint(c: CGContextRef, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat);
    pub fn CGContextClosePath(c: CGContextRef);
    pub fn CGContextAddRect(c: CGContextRef, rect: Rect);
    pub fn CGContextAddRects(c: CGContextRef, rects: *const Rect, count: usize);
    pub fn CGContextAddLines(c: CGContextRef, points: *const Point, count: usize);
    pub fn CGContextAddEllipseInRect(c: CGContextRef, rect: Rect);
    pub fn CGContextAddArc(c: CGContextRef, x: CGFloat, y: CGFloat, radius: CGFloat, startAngle: CGFloat, endAngle: CGFloat, clockwise: i32);
    pub fn CGContextAddArcToPoint(c: CGContextRef, x1: CGFloat, y1: CGFloat, x2: CGFloat, y2: CGFloat, radius: CGFloat);
    pub fn CGContextAddPath(c: CGContextRef, path: CGPathRef);
    pub fn CGContextReplacePathWithStrokedPath(c: CGContextRef);
    pub fn CGContextIsPathEmpty(c: CGContextRef) -> bool;
    pub fn CGContextGetPathCurrentPoint(c: CGContextRef) -> Point;
    pub fn CGContextGetPathBoundingBox(c: CGContextRef) -> Rect;
    pub fn CGContextCopyPath(c: CGContextRef) -> CGPathRef;
    pub fn CGContextPathContainsPoint(c: CGContextRef, point: Point, mode: i32) -> bool;
    pub fn CGContextDrawPath(c: CGContextRef, mode: i32);
    pub fn CGContextFillPath(c: CGContextRef);
    pub fn CGContextEOFillPath(c: CGContextRef);
    pub fn CGContextStrokePath(c: CGContextRef);
    pub fn CGContextFillRect(c: CGContextRef, rect: Rect);
    pub fn CGContextFillRects(c: CGContextRef, rects: *const Rect, count: usize);
    pub fn CGContextStrokeRect(c: CGContextRef, rect: Rect);
    pub fn CGContextStrokeRectWithWidth(c: CGContextRef, rect: Rect, width: CGFloat);
    pub fn CGContextClearRect(c: CGContextRef, rect: Rect);
    pub fn CGContextFillEllipseInRect(c: CGContextRef, rect: Rect);
    pub fn CGContextStrokeEllipseInRect(c: CGContextRef, rect: Rect);
    pub fn CGContextStrokeLineSegments(c: CGContextRef, points: *const Point, count: usize);
    pub fn CGContextClip(c: CGContextRef);
    pub fn CGContextEOClip(c: CGContextRef);
    pub fn CGContextClipToMask(c: CGContextRef, rect: Rect, mask: CGImageRef);
    pub fn CGContextGetClipBoundingBox(c: CGContextRef) -> Rect;
    pub fn CGContextClipToRect(c: CGContextRef, rect: Rect);
    pub fn CGContextClipToRects(c: CGContextRef, rects: *const Rect, count: usize);
    pub fn CGContextSetFillColorWithColor(c: CGContextRef, color: CGColorRef);
    pub fn CGContextSetStrokeColorWithColor(c: CGContextRef, color: CGColorRef);
    pub fn CGContextSetFillColorSpace(c: CGContextRef, space: CGColorSpaceRef);
    pub fn CGContextSetStrokeColorSpace(c: CGContextRef, space: CGColorSpaceRef);
    pub fn CGContextSetFillColor(c: CGContextRef, components: *const CGFloat);
    pub fn CGContextSetStrokeColor(c: CGContextRef, components: *const CGFloat);
    pub fn CGContextSetGrayFillColor(c: CGContextRef, gray: CGFloat, alpha: CGFloat);
    pub fn CGContextSetGrayStrokeColor(c: CGContextRef, gray: CGFloat, alpha: CGFloat);
    pub fn CGContextSetRGBFillColor(c: CGContextRef, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat);
    pub fn CGContextSetRGBStrokeColor(c: CGContextRef, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat);
    pub fn CGContextSetCMYKFillColor(c: CGContextRef, cyan: CGFloat, magenta: CGFloat, yellow: CGFloat, black: CGFloat, alpha: CGFloat);
    pub fn CGContextSetCMYKStrokeColor(c: CGContextRef, cyan: CGFloat, magenta: CGFloat, yellow: CGFloat, black: CGFloat, alpha: CGFloat);
    pub fn CGContextSetRenderingIntent(c: CGContextRef, intent: i32);
    pub fn CGContextDrawImage(c: CGContextRef, rect: Rect, image: CGImageRef);
    pub fn CGContextDrawTiledImage(c: CGContextRef, rect: Rect, image: CGImageRef);
    pub fn CGContextGetInterpolationQuality(c: CGContextRef) -> i32;
    pub fn CGContextSetInterpolationQuality(c: CGContextRef, quality: i32);
    pub fn CGContextSetShadowWithColor(c: CGContextRef, offset: Size, blur: CGFloat, color: CGColorRef);
    pub fn CGContextSetShadow(c: CGContextRef, offset: Size, blur: CGFloat);
    pub fn CGContextDrawLinearGradient(c: CGContextRef, gradient: CGGradientRef, startPoint: Point, endPoint: Point, options: u32);
    pub fn CGContextDrawRadialGradient(c: CGContextRef, gradient: CGGradientRef, startCenter: Point, startRadius: CGFloat, endCenter: Point, endRadius: CGFloat, options: u32);
    pub fn CGContextDrawShading(c: CGContextRef, shading: CGShadingRef);
    pub fn CGContextSetCharacterSpacing(c: CGContextRef, spacing: CGFloat);
    pub fn CGContextSetTextPosition(c: CGContextRef, x: CGFloat, y: CGFloat);
    pub fn CGContextGetTextPosition(c: CGContextRef) -> Point;
    pub fn CGContextSetTextMatrix(c: CGContextRef, t: AffineTransform);
    pub fn CGContextGetTextMatrix(c: CGContextRef) -> AffineTransform;
    pub fn CGContextSetTextDrawingMode(c: CGContextRef, mode: i32);
    pub fn CGContextSetFont(c: CGContextRef, font: CGFontRef);
    pub fn CGContextSetFontSize(c: CGContextRef, size: CGFloat);
    pub fn CGContextShowGlyphsAtPositions(c: CGContextRef, glyphs: *const u16, positions: *const Point, count: usize);
    pub fn CGContextDrawPDFPage(c: CGContextRef, page: CGPDFPageRef);
    pub fn CGContextBeginPage(c: CGContextRef, mediaBox: *const Rect);
    pub fn CGContextEndPage(c: CGContextRef);
    pub fn CGContextRetain(c: CGContextRef) -> CGContextRef;
    pub fn CGContextRelease(c: CGContextRef);
    pub fn CGContextFlush(c: CGContextRef);
    pub fn CGContextSynchronize(c: CGContextRef);
    pub fn CGContextSetShouldAntialias(c: CGContextRef, shouldAntialias: bool);
    pub fn CGContextSetAllowsAntialiasing(c: CGContextRef, allowsAntialiasing: bool);
    pub fn CGContextSetShouldSmoothFonts(c: CGContextRef, shouldSmoothFonts: bool);
    pub fn CGContextSetAllowsFontSmoothing(c: CGContextRef, allowsFontSmoothing: bool);
    pub fn CGContextSetShouldSubpixelPositionFonts(c: CGContextRef, shouldSubpixelPositionFonts: bool);
    pub fn CGContextSetAllowsFontSubpixelPositioning(c: CGContextRef, allowsFontSubpixelPositioning: bool);
    pub fn CGContextSetShouldSubpixelQuantizeFonts(c: CGContextRef, shouldSubpixelQuantizeFonts: bool);
    pub fn CGContextSetAllowsFontSubpixelQuantization(c: CGContextRef, allowsFontSubpixelQuantization: bool);
    pub fn CGContextBeginTransparencyLayer(c: CGContextRef, auxiliaryInfo: CFDictionaryRef);
    pub fn CGContextBeginTransparencyLayerWithRect(c: CGContextRef, rect: Rect, auxiliaryInfo: CFDictionaryRef);
    pub fn CGContextEndTransparencyLayer(c: CGContextRef);
    pub fn CGContextGetUserSpaceToDeviceSpaceTransform(c: CGContextRef) -> AffineTransform;
    pub fn CGContextConvertPointToDeviceSpace(c: CGContextRef, point: Point) -> Point;
    pub fn CGContextConvertPointToUserSpace(c: CGContextRef, point: Point) -> Point;
    pub fn CGContextConvertSizeToDeviceSpace(c: CGContextRef, size: Size) -> Size;
    pub fn CGContextConvertSizeToUserSpace(c: CGContextRef, size: Size) -> Size;
    pub fn CGContextConvertRectToDeviceSpace(c: CGContextRef, rect: Rect) -> Rect;
    pub fn CGContextConvertRectToUserSpace(c: CGContextRef, rect: Rect) -> Rect;
    pub fn CGContextDrawLayerInRect(c: CGContextRef, rect: Rect, layer: CGLayerRef);
    pub fn CGContextDrawLayerAtPoint(c: CGContextRef, point: Point, layer: CGLayerRef);
}

// ── CGBitmapContext (14 functions) ──────────────────────────────────────────

unsafe extern "C" {
    pub fn CGBitmapContextCreate(data: *mut c_void, width: usize, height: usize, bitsPerComponent: usize, bytesPerRow: usize, space: CGColorSpaceRef, bitmapInfo: u32) -> CGContextRef;
    pub fn CGBitmapContextCreateImage(c: CGContextRef) -> CGImageRef;
    pub fn CGBitmapContextGetData(c: CGContextRef) -> *mut c_void;
    pub fn CGBitmapContextGetWidth(c: CGContextRef) -> usize;
    pub fn CGBitmapContextGetHeight(c: CGContextRef) -> usize;
    pub fn CGBitmapContextGetBitsPerComponent(c: CGContextRef) -> usize;
    pub fn CGBitmapContextGetBitsPerPixel(c: CGContextRef) -> usize;
    pub fn CGBitmapContextGetBytesPerRow(c: CGContextRef) -> usize;
    pub fn CGBitmapContextGetColorSpace(c: CGContextRef) -> CGColorSpaceRef;
    pub fn CGBitmapContextGetAlphaInfo(c: CGContextRef) -> u32;
    pub fn CGBitmapContextGetBitmapInfo(c: CGContextRef) -> u32;
}

// ── CGColor (25 functions) ─────────────────────────────────────────────────

unsafe extern "C" {
    pub fn CGColorCreate(space: CGColorSpaceRef, components: *const CGFloat) -> CGColorRef;
    pub fn CGColorCreateGenericGray(gray: CGFloat, alpha: CGFloat) -> CGColorRef;
    pub fn CGColorCreateGenericRGB(r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> CGColorRef;
    pub fn CGColorCreateGenericCMYK(c: CGFloat, m: CGFloat, y: CGFloat, k: CGFloat, a: CGFloat) -> CGColorRef;
    pub fn CGColorCreateSRGB(r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> CGColorRef;
    pub fn CGColorCreateCopy(color: CGColorRef) -> CGColorRef;
    pub fn CGColorCreateCopyWithAlpha(color: CGColorRef, alpha: CGFloat) -> CGColorRef;
    pub fn CGColorRetain(color: CGColorRef) -> CGColorRef;
    pub fn CGColorRelease(color: CGColorRef);
    pub fn CGColorEqualToColor(a: CGColorRef, b: CGColorRef) -> bool;
    pub fn CGColorGetNumberOfComponents(color: CGColorRef) -> usize;
    pub fn CGColorGetComponents(color: CGColorRef) -> *const CGFloat;
    pub fn CGColorGetAlpha(color: CGColorRef) -> CGFloat;
    pub fn CGColorGetColorSpace(color: CGColorRef) -> CGColorSpaceRef;
    pub fn CGColorGetTypeID() -> CFTypeID;
}

// ── CGColorSpace (key functions from 80) ───────────────────────────────────

unsafe extern "C" {
    pub fn CGColorSpaceCreateDeviceGray() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateDeviceRGB() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateDeviceCMYK() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateWithName(name: CFStringRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreatePattern(baseSpace: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceRetain(space: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceRelease(space: CGColorSpaceRef);
    pub fn CGColorSpaceGetTypeID() -> CFTypeID;
    pub fn CGColorSpaceGetNumberOfComponents(space: CGColorSpaceRef) -> usize;
    pub fn CGColorSpaceGetModel(space: CGColorSpaceRef) -> i32;
    pub fn CGColorSpaceGetBaseColorSpace(space: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCopyName(space: CGColorSpaceRef) -> CFStringRef;
    pub fn CGColorSpaceIsWideGamutRGB(space: CGColorSpaceRef) -> bool;
    pub fn CGColorSpaceUsesExtendedRange(space: CGColorSpaceRef) -> bool;
    pub fn CGColorSpaceSupportsOutput(space: CGColorSpaceRef) -> bool;
}

// ── CGImage (39 functions) ─────────────────────────────────────────────────

unsafe extern "C" {
    pub fn CGImageCreate(width: usize, height: usize, bitsPerComponent: usize, bitsPerPixel: usize, bytesPerRow: usize, space: CGColorSpaceRef, bitmapInfo: u32, provider: CGDataProviderRef, decode: *const CGFloat, shouldInterpolate: bool, intent: i32) -> CGImageRef;
    pub fn CGImageRetain(image: CGImageRef) -> CGImageRef;
    pub fn CGImageRelease(image: CGImageRef);
    pub fn CGImageGetTypeID() -> CFTypeID;
    pub fn CGImageGetWidth(image: CGImageRef) -> usize;
    pub fn CGImageGetHeight(image: CGImageRef) -> usize;
    pub fn CGImageGetBitsPerComponent(image: CGImageRef) -> usize;
    pub fn CGImageGetBitsPerPixel(image: CGImageRef) -> usize;
    pub fn CGImageGetBytesPerRow(image: CGImageRef) -> usize;
    pub fn CGImageGetColorSpace(image: CGImageRef) -> CGColorSpaceRef;
    pub fn CGImageGetAlphaInfo(image: CGImageRef) -> u32;
    pub fn CGImageGetBitmapInfo(image: CGImageRef) -> u32;
    pub fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;
    pub fn CGImageGetDecode(image: CGImageRef) -> *const CGFloat;
    pub fn CGImageGetShouldInterpolate(image: CGImageRef) -> bool;
    pub fn CGImageGetRenderingIntent(image: CGImageRef) -> i32;
    pub fn CGImageIsMask(image: CGImageRef) -> bool;
    pub fn CGImageCreateCopyWithColorSpace(image: CGImageRef, space: CGColorSpaceRef) -> CGImageRef;
    pub fn CGImageCreateWithImageInRect(image: CGImageRef, rect: Rect) -> CGImageRef;
    pub fn CGImageCreateWithMask(image: CGImageRef, mask: CGImageRef) -> CGImageRef;
    pub fn CGImageCreateWithMaskingColors(image: CGImageRef, components: *const CGFloat) -> CGImageRef;
}

// ── CGPath (46 functions) ──────────────────────────────────────────────────

unsafe extern "C" {
    pub fn CGPathCreateMutable() -> CGMutablePathRef;
    pub fn CGPathCreateCopy(path: CGPathRef) -> CGPathRef;
    pub fn CGPathCreateMutableCopy(path: CGPathRef) -> CGMutablePathRef;
    pub fn CGPathCreateWithRect(rect: Rect, transform: *const AffineTransform) -> CGPathRef;
    pub fn CGPathCreateWithEllipseInRect(rect: Rect, transform: *const AffineTransform) -> CGPathRef;
    pub fn CGPathCreateWithRoundedRect(rect: Rect, cornerWidth: CGFloat, cornerHeight: CGFloat, transform: *const AffineTransform) -> CGPathRef;
    pub fn CGPathRetain(path: CGPathRef) -> CGPathRef;
    pub fn CGPathRelease(path: CGPathRef);
    pub fn CGPathEqualToPath(p1: CGPathRef, p2: CGPathRef) -> bool;
    pub fn CGPathMoveToPoint(path: CGMutablePathRef, m: *const AffineTransform, x: CGFloat, y: CGFloat);
    pub fn CGPathAddLineToPoint(path: CGMutablePathRef, m: *const AffineTransform, x: CGFloat, y: CGFloat);
    pub fn CGPathAddQuadCurveToPoint(path: CGMutablePathRef, m: *const AffineTransform, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat);
    pub fn CGPathAddCurveToPoint(path: CGMutablePathRef, m: *const AffineTransform, cp1x: CGFloat, cp1y: CGFloat, cp2x: CGFloat, cp2y: CGFloat, x: CGFloat, y: CGFloat);
    pub fn CGPathCloseSubpath(path: CGMutablePathRef);
    pub fn CGPathAddRect(path: CGMutablePathRef, m: *const AffineTransform, rect: Rect);
    pub fn CGPathAddRects(path: CGMutablePathRef, m: *const AffineTransform, rects: *const Rect, count: usize);
    pub fn CGPathAddLines(path: CGMutablePathRef, m: *const AffineTransform, points: *const Point, count: usize);
    pub fn CGPathAddEllipseInRect(path: CGMutablePathRef, m: *const AffineTransform, rect: Rect);
    pub fn CGPathAddRelativeArc(path: CGMutablePathRef, m: *const AffineTransform, x: CGFloat, y: CGFloat, radius: CGFloat, startAngle: CGFloat, delta: CGFloat);
    pub fn CGPathAddArc(path: CGMutablePathRef, m: *const AffineTransform, x: CGFloat, y: CGFloat, radius: CGFloat, startAngle: CGFloat, endAngle: CGFloat, clockwise: bool);
    pub fn CGPathAddArcToPoint(path: CGMutablePathRef, m: *const AffineTransform, x1: CGFloat, y1: CGFloat, x2: CGFloat, y2: CGFloat, radius: CGFloat);
    pub fn CGPathAddPath(p1: CGMutablePathRef, m: *const AffineTransform, p2: CGPathRef);
    pub fn CGPathAddRoundedRect(path: CGMutablePathRef, m: *const AffineTransform, rect: Rect, cornerWidth: CGFloat, cornerHeight: CGFloat);
    pub fn CGPathIsEmpty(path: CGPathRef) -> bool;
    pub fn CGPathIsRect(path: CGPathRef, rect: *mut Rect) -> bool;
    pub fn CGPathGetCurrentPoint(path: CGPathRef) -> Point;
    pub fn CGPathGetBoundingBox(path: CGPathRef) -> Rect;
    pub fn CGPathGetPathBoundingBox(path: CGPathRef) -> Rect;
    pub fn CGPathContainsPoint(path: CGPathRef, m: *const AffineTransform, point: Point, eoFill: bool) -> bool;
    pub fn CGPathCreateCopyByTransformingPath(path: CGPathRef, transform: *const AffineTransform) -> CGPathRef;
    pub fn CGPathCreateCopyByDashingPath(path: CGPathRef, transform: *const AffineTransform, phase: CGFloat, lengths: *const CGFloat, count: usize) -> CGPathRef;
    pub fn CGPathCreateCopyByStrokingPath(path: CGPathRef, transform: *const AffineTransform, lineWidth: CGFloat, lineCap: i32, lineJoin: i32, miterLimit: CGFloat) -> CGPathRef;
    pub fn CGPathGetTypeID() -> CFTypeID;
}

// ── CGGradient (7 functions) ───────────────────────────────────────────────

unsafe extern "C" {
    pub fn CGGradientCreateWithColorComponents(space: CGColorSpaceRef, components: *const CGFloat, locations: *const CGFloat, count: usize) -> CGGradientRef;
    pub fn CGGradientCreateWithColors(space: CGColorSpaceRef, colors: CFArrayRef, locations: *const CGFloat) -> CGGradientRef;
    pub fn CGGradientRetain(gradient: CGGradientRef) -> CGGradientRef;
    pub fn CGGradientRelease(gradient: CGGradientRef);
    pub fn CGGradientGetTypeID() -> CFTypeID;
}

// ── CGFont (key functions from 34) ─────────────────────────────────────────

unsafe extern "C" {
    pub fn CGFontCreateWithFontName(name: CFStringRef) -> CGFontRef;
    pub fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
    pub fn CGFontRetain(font: CGFontRef) -> CGFontRef;
    pub fn CGFontRelease(font: CGFontRef);
    pub fn CGFontGetNumberOfGlyphs(font: CGFontRef) -> usize;
    pub fn CGFontGetUnitsPerEm(font: CGFontRef) -> i32;
    pub fn CGFontCopyPostScriptName(font: CGFontRef) -> CFStringRef;
    pub fn CGFontCopyFullName(font: CGFontRef) -> CFStringRef;
    pub fn CGFontGetAscent(font: CGFontRef) -> i32;
    pub fn CGFontGetDescent(font: CGFontRef) -> i32;
    pub fn CGFontGetLeading(font: CGFontRef) -> i32;
    pub fn CGFontGetCapHeight(font: CGFontRef) -> i32;
    pub fn CGFontGetXHeight(font: CGFontRef) -> i32;
    pub fn CGFontGetItalicAngle(font: CGFontRef) -> CGFloat;
    pub fn CGFontGetStemV(font: CGFontRef) -> CGFloat;
    pub fn CGFontGetTypeID() -> CFTypeID;
}

// ── CGDataProvider (key functions) ─────────────────────────────────────────

unsafe extern "C" {
    pub fn CGDataProviderCreateWithData(info: *mut c_void, data: *const c_void, size: usize, releaseData: *const c_void) -> CGDataProviderRef;
    pub fn CGDataProviderCreateWithCFData(data: CFDataRef) -> CGDataProviderRef;
    pub fn CGDataProviderCreateWithURL(url: CFURLRef) -> CGDataProviderRef;
    pub fn CGDataProviderCreateWithFilename(filename: *const u8) -> CGDataProviderRef;
    pub fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
    pub fn CGDataProviderRelease(provider: CGDataProviderRef);
    pub fn CGDataProviderCopyData(provider: CGDataProviderRef) -> CFDataRef;
    pub fn CGDataProviderGetTypeID() -> CFTypeID;
}

// ── CGLayer (8 functions) ──────────────────────────────────────────────────

unsafe extern "C" {
    pub fn CGLayerCreateWithContext(c: CGContextRef, size: Size, auxiliaryInfo: CFDictionaryRef) -> CGLayerRef;
    pub fn CGLayerGetSize(layer: CGLayerRef) -> Size;
    pub fn CGLayerGetContext(layer: CGLayerRef) -> CGContextRef;
    pub fn CGLayerRetain(layer: CGLayerRef) -> CGLayerRef;
    pub fn CGLayerRelease(layer: CGLayerRef);
    pub fn CGLayerGetTypeID() -> CFTypeID;
}

// ── CGPDFDocument (key functions from 28) ──────────────────────────────────

unsafe extern "C" {
    pub fn CGPDFDocumentCreateWithURL(url: CFURLRef) -> CGPDFDocumentRef;
    pub fn CGPDFDocumentCreateWithProvider(provider: CGDataProviderRef) -> CGPDFDocumentRef;
    pub fn CGPDFDocumentRetain(doc: CGPDFDocumentRef) -> CGPDFDocumentRef;
    pub fn CGPDFDocumentRelease(doc: CGPDFDocumentRef);
    pub fn CGPDFDocumentGetNumberOfPages(doc: CGPDFDocumentRef) -> usize;
    pub fn CGPDFDocumentGetPage(doc: CGPDFDocumentRef, pageNumber: usize) -> CGPDFPageRef;
    pub fn CGPDFDocumentIsEncrypted(doc: CGPDFDocumentRef) -> bool;
    pub fn CGPDFDocumentIsUnlocked(doc: CGPDFDocumentRef) -> bool;
    pub fn CGPDFDocumentUnlockWithPassword(doc: CGPDFDocumentRef, password: *const u8) -> bool;
    pub fn CGPDFDocumentAllowsPrinting(doc: CGPDFDocumentRef) -> bool;
    pub fn CGPDFDocumentAllowsCopying(doc: CGPDFDocumentRef) -> bool;
    pub fn CGPDFDocumentGetVersion(doc: CGPDFDocumentRef, majorVersion: *mut i32, minorVersion: *mut i32);
    pub fn CGPDFDocumentGetTypeID() -> CFTypeID;
}

// ── CGPDFPage (9 functions) ────────────────────────────────────────────────

unsafe extern "C" {
    pub fn CGPDFPageRetain(page: CGPDFPageRef) -> CGPDFPageRef;
    pub fn CGPDFPageRelease(page: CGPDFPageRef);
    pub fn CGPDFPageGetDocument(page: CGPDFPageRef) -> CGPDFDocumentRef;
    pub fn CGPDFPageGetPageNumber(page: CGPDFPageRef) -> usize;
    pub fn CGPDFPageGetBoxRect(page: CGPDFPageRef, box_: i32) -> Rect;
    pub fn CGPDFPageGetRotationAngle(page: CGPDFPageRef) -> i32;
    pub fn CGPDFPageGetDrawingTransform(page: CGPDFPageRef, box_: i32, rect: Rect, rotate: i32, preserveAspectRatio: bool) -> AffineTransform;
    pub fn CGPDFPageGetTypeID() -> CFTypeID;
}
