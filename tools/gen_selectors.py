#!/usr/bin/env python3
"""Generate clean ffi.rs ObjC selector constants from framework headers.

Usage: python3 tools/gen_selectors.py Framework Class1 Class2 ... > crates/x/src/ffi.rs
"""
import re, subprocess, os, sys, glob

SDK = subprocess.check_output(["xcrun","--sdk","macosx","--show-sdk-path"], text=True).strip()

def find_headers(fw):
    for d in [f"{SDK}/System/Library/Frameworks/{fw}.framework/Headers",
              f"{SDK}/System/Library/Frameworks/{fw}.framework/Versions/A/Headers",
              f"{SDK}/System/Library/Frameworks/{fw}.framework/Versions/C/Headers"]:
        if os.path.isdir(d): return d
    return None

def snake(name):
    s = re.sub(r'([A-Z])', r'_\1', name).upper().lstrip('_')
    return re.sub(r'_+', '_', s)

def extract_class(hdir, cls):
    """Extract properties and method selectors for a class from its header."""
    # Find the header file
    hfile = None
    for h in glob.glob(f"{hdir}/*.h"):
        with open(h) as f:
            if re.search(rf'^@interface\s+{cls}\b', f.read(), re.MULTILINE):
                hfile = h
                break
    if not hfile:
        return [], []

    with open(hfile) as f:
        text = f.read()
    # Remove comments
    text = re.sub(r'/\*.*?\*/', '', text, flags=re.DOTALL)
    text = re.sub(r'//.*$', '', text, flags=re.MULTILINE)

    # Find the @interface...@end block
    m = re.search(rf'@interface\s+{cls}\b.*?@end', text, re.DOTALL)
    if not m:
        return [], []
    body = m.group(0)

    # Properties
    props = []
    for pm in re.finditer(r'@property\s*\([^)]*\)[^;]*?(\b\w+)\s*(?:__attribute__\([^)]*\))?\s*;', body):
        pname = pm.group(1)
        if pname and pname not in ('void',):
            props.append(pname)

    # Methods - extract just the selector pattern
    methods = []
    for mm in re.finditer(r'^[-+]\s*\([^)]+\)\s*([^;{]+);', body, re.MULTILINE):
        raw = mm.group(1).strip()
        # Build selector: keep only word: patterns, strip types and param names
        # "doThing:(NSString *)foo bar:(int)baz" -> "doThing:bar:"
        parts = re.split(r'\s*\([^)]*\)\s*\w+\s*', raw)
        sel = ''.join(p.strip() for p in parts if p.strip())
        # Clean up remaining annotations
        sel = re.sub(r'\b(?:API_\w+|NS_\w+|CF_\w+|__attribute__)\b[^;]*', '', sel)
        sel = re.sub(r'\([^)]*\)', '', sel)
        sel = sel.strip().rstrip(';').strip()
        if sel and not sel.startswith('init') and sel not in ('new', 'alloc', 'dealloc'):
            methods.append(sel)

    return list(dict.fromkeys(props)), list(dict.fromkeys(methods))

def main():
    if len(sys.argv) < 3:
        print("Usage: gen_selectors.py Framework Class1 Class2 ...", file=sys.stderr)
        sys.exit(1)
    fw = sys.argv[1]
    classes = sys.argv[2:]
    hdir = find_headers(fw)
    if not hdir:
        print(f"// Headers for {fw} not found", file=sys.stderr)
        sys.exit(1)

    print(f"//! ObjC selector constants for {fw}.")
    print("#![allow(dead_code)]")
    print()

    total = 0
    for cls in classes:
        props, methods = extract_class(hdir, cls)
        modname = re.sub(r'([A-Z])', r'_\1', cls).lower().lstrip('_')

        print(f"// ── {cls} ({len(methods)} methods, {len(props)} properties) ──")
        print(f"pub mod {modname} {{")
        print(f'    pub const CLASS: &[u8] = b"{cls}\\0";')

        seen = set()
        for p in props:
            key = snake(p)
            set_key = f"SET_{key}"
            if key in seen: continue
            seen.add(key)
            print(f'    pub const SEL_{key}: &[u8] = b"{p}\\0";')
            if set_key not in seen:
                seen.add(set_key)
                setter = f"set{p[0].upper()}{p[1:]}:"
                print(f'    pub const SEL_{set_key}: &[u8] = b"{setter}\\0";')
                total += 1
            total += 1

        for m in methods:
            first = m.split(':')[0]
            key = snake(first)
            # Also check SET_ variant to avoid collisions with property setters
            if key in seen or f"SET_{key}" in seen: continue
            seen.add(key)
            print(f'    pub const SEL_{key}: &[u8] = b"{m}\\0";')
            total += 1

        print("}")
        print()

    print(f"// Total: {total} selector constants")

if __name__ == '__main__':
    main()
