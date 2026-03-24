#!/usr/bin/env python3
"""Generate Rust extern "C" blocks from CoreGraphics headers.

Usage: python3 tools/gen_cg_ffi.py > crates/coregraphics/src/ffi.rs
"""

import re, subprocess, os

SDK = subprocess.check_output(
    ["xcrun", "--sdk", "macosx", "--show-sdk-path"],
    text=True
).strip()

HDIR = f"{SDK}/System/Library/Frameworks/CoreGraphics.framework/Headers"

# Map C types to Rust types
TYPE_MAP = {
    "void": "()",
    "bool": "bool",
    "int": "i32",
    "int32_t": "i32",
    "uint32_t": "u32",
    "int64_t": "i64",
    "uint64_t": "u64",
    "size_t": "usize",
    "float": "f32",
    "double": "f64",
    "CGFloat": "f64",
    "CFTypeID": "u64",
    "CFIndex": "isize",
    "CGRect": "CGRect",
    "CGPoint": "CGPoint",
    "CGSize": "CGSize",
    "CGAffineTransform": "CGAffineTransform",
    "CGContextRef": "*mut c_void",
    "CGColorRef": "*mut c_void",
    "CGColorSpaceRef": "*mut c_void",
    "CGImageRef": "*mut c_void",
    "CGPathRef": "*const c_void",
    "CGMutablePathRef": "*mut c_void",
    "CGGradientRef": "*mut c_void",
    "CGFontRef": "*mut c_void",
    "CGFunctionRef": "*mut c_void",
    "CGPatternRef": "*mut c_void",
    "CGShadingRef": "*mut c_void",
    "CGLayerRef": "*mut c_void",
    "CGDataProviderRef": "*mut c_void",
    "CGDataConsumerRef": "*mut c_void",
    "CGPDFDocumentRef": "*mut c_void",
    "CGPDFPageRef": "*mut c_void",
    "CGPDFArrayRef": "*const c_void",
    "CGPDFDictionaryRef": "*const c_void",
    "CGPDFStringRef": "*const c_void",
    "CGPDFStreamRef": "*const c_void",
    "CGPDFObjectRef": "*const c_void",
    "CGPDFScannerRef": "*mut c_void",
    "CGPDFOperatorTableRef": "*mut c_void",
    "CGPDFContentStreamRef": "*mut c_void",
    "CFStringRef": "*const c_void",
    "CFArrayRef": "*const c_void",
    "CFMutableArrayRef": "*mut c_void",
    "CFDictionaryRef": "*const c_void",
    "CFMutableDictionaryRef": "*mut c_void",
    "CFDataRef": "*const c_void",
    "CFURLRef": "*const c_void",
    "CFAllocatorRef": "*const c_void",
    "CGLineCap": "i32",
    "CGLineJoin": "i32",
    "CGPathDrawingMode": "i32",
    "CGBlendMode": "i32",
    "CGInterpolationQuality": "i32",
    "CGTextDrawingMode": "i32",
    "CGTextEncoding": "i32",
    "CGColorRenderingIntent": "i32",
    "CGPathElementType": "i32",
    "CGPatternTiling": "i32",
    "CGGradientDrawingOptions": "u32",
    "CGColorSpaceModel": "i32",
    "CGImageAlphaInfo": "u32",
    "CGBitmapInfo": "u32",
    "CGImageByteOrderInfo": "u32",
    "CGFontIndex": "u16",
    "CGGlyph": "u16",
    "CGPDFObjectType": "i32",
    "CGPDFBoolean": "u8",
    "CGPDFInteger": "isize",
    "CGPDFReal": "f64",
    "CGPDFDataFormat": "i32",
    "CGDisplayCount": "u32",
    "CGDirectDisplayID": "u32",
}

def map_type(ctype):
    """Map a C type string to Rust."""
    ctype = ctype.strip()
    # Remove qualifiers and annotations
    for q in ["const ", "cg_nullable ", "cg_nonnull ", "__nullable ", "__nonnull ",
              "CF_RETURNS_RETAINED ", "CF_RETURNS_NOT_RETAINED ",
              "CG_PURE ", "CF_REFINED_FOR_SWIFT "]:
        ctype = ctype.replace(q, "")
    # Remove API_AVAILABLE(...), CG_AVAILABLE_STARTING(...), etc.
    ctype = re.sub(r'(?:API_AVAILABLE|API_DEPRECATED|CG_AVAILABLE_STARTING|CG_AVAILABLE_BUT_DEPRECATED)\([^)]*\)', '', ctype)
    # Remove everything after the first semicolon or CG_EXTERN (multi-decl on one line)
    ctype = ctype.split(';')[0].split('CG_EXTERN')[0]
    ctype = re.sub(r'\b(restrict|volatile)\b', '', ctype).strip()
    ctype = re.sub(r'\s+', ' ', ctype).strip()
    
    # Pointer types
    if ctype.endswith("*"):
        base = ctype[:-1].strip()
        if base == "char" or base == "const char":
            return "*const u8"
        if base == "void" or base == "const void":
            return "*const c_void"
        return f"*mut c_void"  # Generic pointer
    
    if ctype in TYPE_MAP:
        return TYPE_MAP[ctype]
    
    # Fallback
    return f"/* {ctype} */ *mut c_void"

def parse_functions(header_path):
    """Extract CG_EXTERN function declarations."""
    with open(header_path) as f:
        content = f.read()
    
    # Remove comments
    content = re.sub(r'/\*.*?\*/', '', content, flags=re.DOTALL)
    content = re.sub(r'//.*$', '', content, flags=re.MULTILINE)
    
    # Find CG_EXTERN declarations (may span multiple lines)
    # Join continuation lines
    content = re.sub(r'\n\s+', ' ', content)
    
    functions = []
    for m in re.finditer(r'CG_EXTERN\s+(.+?)\s+(CG\w+)\s*\(([^)]*)\)', content):
        ret_type = m.group(1).strip()
        name = m.group(2).strip()
        params_str = m.group(3).strip()
        
        # Parse parameters
        if params_str == "void" or params_str == "":
            params = []
        else:
            params = []
            for p in re.split(r',\s*', params_str):
                p = p.strip()
                if not p:
                    continue
                # Split into type and name
                parts = p.rsplit(None, 1)
                if len(parts) == 2:
                    ptype, pname = parts
                    # Clean up parameter name
                    pname = re.sub(r'[^a-zA-Z0-9_]', '', pname)
                    if pname in ('type', 'ref', 'match', 'move', 'fn', 'self', 'in', 'box'):
                        pname = pname + '_'
                    params.append((pname, map_type(ptype)))
                else:
                    params.append(("arg", map_type(p)))
        
        rust_ret = map_type(ret_type)
        functions.append((name, params, rust_ret))
    
    return functions

# Generate
headers = [
    "CGAffineTransform", "CGBitmapContext", "CGColor", "CGColorSpace",
    "CGContext", "CGFont", "CGGeometry", "CGGradient", "CGImage",
    "CGLayer", "CGPath", "CGPattern", "CGShading",
    "CGPDFDocument", "CGPDFPage", "CGPDFContext",
    "CGDataProvider", "CGDataConsumer", "CGFunction",
]

print("// Auto-generated CoreGraphics FFI bindings")
print("// Run: python3 tools/gen_cg_ffi.py > crates/coregraphics/src/ffi.rs")
print("")
print("#![allow(non_snake_case, dead_code, unused_variables)]")
print("")
print("use core::ffi::c_void;")
print("")
print("// Types re-exported from parent module")
print("use super::{CGFloat, Point as CGPoint, Size as CGSize, Rect as CGRect, AffineTransform as CGAffineTransform};")
print("")

for header in headers:
    path = os.path.join(HDIR, f"{header}.h")
    if not os.path.exists(path):
        continue
    
    fns = parse_functions(path)
    if not fns:
        continue
    
    print(f"// ── {header} ({len(fns)} functions) ──")
    print("")
    print("unsafe extern \"C\" {")
    
    for name, params, ret in fns:
        param_str = ", ".join(f"{pname}: {ptype}" for pname, ptype in params)
        if ret == "()":
            print(f"    pub fn {name}({param_str});")
        else:
            print(f"    pub fn {name}({param_str}) -> {ret};")
    
    print("}")
    print("")

print(f"// Total: {sum(len(parse_functions(os.path.join(HDIR, h + '.h'))) for h in headers if os.path.exists(os.path.join(HDIR, h + '.h')))} functions")
