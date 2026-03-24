#!/usr/bin/env python3
"""Generate Rust ObjC FFI wrappers from Apple framework headers.

Usage:
  python3 tools/gen_objc_ffi.py AVFoundation > crates/avfoundation/src/ffi.rs
  python3 tools/gen_objc_ffi.py CoreData > crates/coredata/src/ffi.rs
"""

import re, subprocess, os, sys

SDK = subprocess.check_output(
    ["xcrun", "--sdk", "macosx", "--show-sdk-path"], text=True
).strip()

def find_headers(fw):
    """Find the headers directory for a framework."""
    candidates = [
        f"{SDK}/System/Library/Frameworks/{fw}.framework/Headers",
        f"{SDK}/System/Library/Frameworks/{fw}.framework/Versions/A/Headers",
        f"{SDK}/System/Library/Frameworks/{fw}.framework/Versions/C/Headers",
    ]
    for c in candidates:
        if os.path.isdir(c):
            return c
    return None

def parse_interface(header_text):
    """Parse @interface declarations from a header."""
    classes = {}
    # Remove comments
    text = re.sub(r'/\*.*?\*/', '', header_text, flags=re.DOTALL)
    text = re.sub(r'//.*$', '', text, flags=re.MULTILINE)

    # Find @interface blocks
    for m in re.finditer(r'@interface\s+(\w+)\s*(?::\s*(\w+))?(?:\s*<[^>]*>)?\s*(.*?)@end', text, re.DOTALL):
        name = m.group(1)
        parent = m.group(2) or "NSObject"
        body = m.group(3)

        methods = []
        properties = []

        # Parse properties
        for pm in re.finditer(r'@property\s*\(([^)]*)\)\s*(.+?)\s+(\w+)\s*;', body):
            attrs = pm.group(1)
            ptype = pm.group(2).strip()
            pname = pm.group(3)
            readonly = 'readonly' in attrs
            properties.append({
                'name': pname,
                'type': ptype,
                'readonly': readonly,
                'class_prop': 'class' in attrs,
            })

        # Parse class methods (+)
        for mm in re.finditer(r'^\+\s*\(([^)]+)\)\s*(\w+(?::[^;]*)?)\s*;', body, re.MULTILINE):
            ret = mm.group(1).strip()
            sel = mm.group(2).strip()
            methods.append({'kind': '+', 'ret': ret, 'selector': sel})

        # Parse instance methods (-)
        for mm in re.finditer(r'^-\s*\(([^)]+)\)\s*(\w+(?::[^;]*)?)\s*;', body, re.MULTILINE):
            ret = mm.group(1).strip()
            sel = mm.group(2).strip()
            methods.append({'kind': '-', 'ret': ret, 'selector': sel})

        classes[name] = {
            'parent': parent,
            'methods': methods,
            'properties': properties,
        }

    return classes

def rust_sel_name(selector):
    """Convert ObjC selector to Rust-friendly name."""
    # "initWithURL:options:" -> "init_with_url_options"
    parts = selector.replace(':', '_').rstrip('_')
    # CamelCase to snake_case
    result = re.sub(r'([A-Z])', lambda m: '_' + m.group(1).lower(), parts)
    result = result.lstrip('_')
    # Fix double underscores
    result = re.sub(r'_+', '_', result)
    return result

def generate_class_bindings(name, info):
    """Generate selector constants and property accessors for a class."""
    lines = []
    lines.append(f"// ── {name} ({len(info['methods'])} methods, {len(info['properties'])} properties) ──")
    lines.append(f"pub mod {name.lower()} {{")
    lines.append(f"    pub const CLASS: &[u8] = b\"{name}\\0\";")

    # Properties as selector strings
    for p in info['properties']:
        sel_name = p['name']
        lines.append(f"    pub const SEL_{sel_name.upper()}: &[u8] = b\"{sel_name}\\0\";")
        if not p['readonly']:
            setter = f"set{sel_name[0].upper()}{sel_name[1:]}:"
            lines.append(f"    pub const SEL_SET_{sel_name.upper()}: &[u8] = b\"{setter}\\0\";")

    # Methods as selector strings
    seen_sels = set()
    for m in info['methods']:
        sel = m['selector']
        # Strip API annotations, NS macros, parameter types/names
        sel_clean = re.sub(r'API_\w+\([^)]*\)', '', sel)
        sel_clean = re.sub(r'NS_\w+(?:\([^)]*\))?', '', sel_clean)
        sel_clean = re.sub(r'CF_\w+(?:\([^)]*\))?', '', sel_clean)
        # Remove parameter types: (Type *)name -> just keep selector parts
        sel_clean = re.sub(r'\([^)]*\)\s*\w+', '', sel_clean)
        sel_clean = re.sub(r'\s+', '', sel_clean).strip().rstrip(';')
        if not sel_clean or sel_clean in seen_sels:
            continue
        seen_sels.add(sel_clean)
        first_word = sel_clean.split(':')[0]
        rust_name = rust_sel_name(first_word).upper()
        if rust_name in ('NEW', 'INIT', 'ALLOC', 'DEALLOC'):
            continue  # skip standard lifecycle
        lines.append(f"    pub const SEL_{rust_name}: &[u8] = b\"{sel_clean}\\0\";")

    lines.append("}")
    lines.append("")
    return "\n".join(lines)


def main():
    if len(sys.argv) < 2:
        print("Usage: gen_objc_ffi.py <FrameworkName> [--classes Class1,Class2,...]", file=sys.stderr)
        sys.exit(1)

    fw = sys.argv[1]
    filter_classes = None
    if len(sys.argv) > 3 and sys.argv[2] == '--classes':
        filter_classes = set(sys.argv[3].split(','))

    hdir = find_headers(fw)
    if not hdir:
        print(f"// Framework '{fw}' headers not found", file=sys.stderr)
        sys.exit(1)

    all_classes = {}
    for hfile in sorted(os.listdir(hdir)):
        if not hfile.endswith('.h'):
            continue
        with open(os.path.join(hdir, hfile)) as f:
            text = f.read()
        classes = parse_interface(text)
        all_classes.update(classes)

    if filter_classes:
        all_classes = {k: v for k, v in all_classes.items() if k in filter_classes}

    # Generate output
    print(f"//! Auto-generated ObjC selector constants for {fw}.")
    print(f"//! {len(all_classes)} classes, generated from SDK headers.")
    print(f"//!")
    print(f"//! Usage: `sel_registerName({fw.lower()}::avplayer::SEL_PLAY.as_ptr())`")
    print()
    print("#![allow(dead_code)]")
    print()

    total_methods = 0
    total_props = 0
    for name in sorted(all_classes.keys()):
        info = all_classes[name]
        total_methods += len(info['methods'])
        total_props += len(info['properties'])
        print(generate_class_bindings(name, info))

    print(f"// Total: {len(all_classes)} classes, {total_methods} methods, {total_props} properties")


if __name__ == '__main__':
    main()
