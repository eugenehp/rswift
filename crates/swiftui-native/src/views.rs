//! Native view construction — pure Rust, no Swift code.
//!
//! Uses dlsym to resolve SwiftUI symbols and arm64 asm to call Swift CC
//! functions directly. No bridge dylib, no build-time Swift compilation.
//!
//! # What works natively
//!
//! | View | Status | Symbol |
//! |------|--------|--------|
//! | Text | ✅ | LocalizedStringKey.init + Text.init (exported) |
//! | EmptyView | ✅ | EmptyView.init() (exported) |
//! | Divider | ✅ | Divider.init() (exported) |
//! | Image (SF) | ✅ | Image.init(systemName:) (exported) |
//! | Color | ❌ | init inlined by compiler |
//! | Spacer | ❌ | init inlined by compiler |
//! | Modifiers | ❌ | protocol extensions not exported |
//! | Stacks | ❌ | generic inits not exported |

use crate::existential::ViewExistential;
use crate::resolve;
use core::ffi::c_void;
use swift_runtime_sys::SwiftABI::get_value_witness_table;

/// Create a Swift.String from a Rust &str using the runtime.
/// Returns the 16-byte String value.
unsafe fn make_swift_string(s: &str) -> [u8; 16] {
    swift_runtime_sys::SwiftUIBridge::create_swift_string(s)
        .expect("Failed to create Swift.String")
}

/// Create a LocalizedStringKey from a Swift.String.
/// LSK.init(stringLiteral:) takes a String and returns an LSK.
unsafe fn make_lsk(swift_string: &[u8; 16]) -> Vec<u8> {
    let init_fn = resolve::lsk_string_literal_init();
    let lsk_meta = resolve::sym(c"$s7SwiftUI18LocalizedStringKeyVN");

    // LSK size from VWT
    let vwt = &*get_value_witness_table(lsk_meta);
    let mut result = vec![0u8; vwt.size];

    // Swift CC: LSK.init(stringLiteral: String)
    // String is 16 bytes passed in x0,x1. LSK metadata in x5 (self metatype).
    // Result returned via registers or indirect (x8) if > 32 bytes.
    #[cfg(target_arch = "aarch64")]
    {
        let s0 = u64::from_le_bytes(swift_string[..8].try_into().unwrap());
        let s1 = u64::from_le_bytes(swift_string[8..].try_into().unwrap());

        if vwt.size <= 32 {
            let r0: u64;
            let r1: u64;
            let r2: u64;
            let r3: u64;
            core::arch::asm!(
                "blr {func}",
                func = in(reg) init_fn,
                in("x0") s0,
                in("x1") s1,
                in("x20") lsk_meta,
                lateout("x0") r0, lateout("x1") r1,
                lateout("x2") r2, lateout("x3") r3,
                lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
                lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
                lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
                lateout("x16") _, lateout("x17") _, lateout("lr") _,
                clobber_abi("C"),
            );
            result[..8].copy_from_slice(&r0.to_le_bytes());
            if vwt.size > 8 { result[8..16].copy_from_slice(&r1.to_le_bytes()); }
            if vwt.size > 16 { result[16..24].copy_from_slice(&r2.to_le_bytes()); }
            if vwt.size > 24 { result[24..32].copy_from_slice(&r3.to_le_bytes()); }
        } else {
            core::arch::asm!(
                "blr {func}",
                func = in(reg) init_fn,
                in("x0") s0,
                in("x1") s1,
                in("x8") result.as_mut_ptr(),
                in("x20") lsk_meta,
                lateout("x0") _, lateout("x1") _,
                lateout("x2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _,
                lateout("x6") _, lateout("x7") _,
                lateout("x9") _, lateout("x10") _, lateout("x11") _,
                lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
                lateout("x16") _, lateout("x17") _, lateout("lr") _,
                clobber_abi("C"),
            );
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        panic!("swiftui-native only supports aarch64");
    }

    result
}

/// Create a Text from a LocalizedStringKey.
/// Text.init(_:tableName:bundle:comment:)
unsafe fn make_text_from_lsk(lsk: &[u8]) -> Vec<u8> {
    let init_fn = resolve::text_lsk_init();
    let text_meta = resolve::text_metadata();
    let vwt = &*get_value_witness_table(text_meta);
    let mut result = vec![0u8; vwt.size];

    #[cfg(target_arch = "aarch64")]
    {
        // LSK value in registers, then nil,nil,nil for tableName,bundle,comment
        // Text is 32 bytes — returned in x0-x3
        let l0 = u64::from_le_bytes(lsk[..8].try_into().unwrap());
        let l1 = if lsk.len() > 8 { u64::from_le_bytes(lsk[8..16].try_into().unwrap()) } else { 0 };
        let l2 = if lsk.len() > 16 { u64::from_le_bytes(lsk[16..24].try_into().unwrap()) } else { 0 };

        let r0: u64;
        let r1: u64;
        let r2: u64;
        let r3: u64;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) init_fn,
            in("x0") l0,         // LSK word 0
            in("x1") l1,         // LSK word 1
            in("x2") l2,         // LSK word 2
            in("x3") 0u64,       // tableName: nil
            in("x4") 0u64,       // bundle: nil
            in("x5") 0u64,       // comment: nil (StaticString?)
            in("x6") 0u64,
            in("x20") text_meta, // Self metatype
            lateout("x0") r0, lateout("x1") r1,
            lateout("x2") r2, lateout("x3") r3,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        result[..8].copy_from_slice(&r0.to_le_bytes());
        if vwt.size > 8 { result[8..16].copy_from_slice(&r1.to_le_bytes()); }
        if vwt.size > 16 { result[16..24].copy_from_slice(&r2.to_le_bytes()); }
        if vwt.size > 24 { result[24..32].copy_from_slice(&r3.to_le_bytes()); }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        panic!("swiftui-native only supports aarch64");
    }

    result
}

// ═══════════════════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════════════════

/// Create a Text view from a Rust string. Pure Rust → Swift runtime.
pub fn text(s: &str) -> ViewExistential {
    unsafe {
        let swift_str = make_swift_string(s);
        let lsk = make_lsk(&swift_str);
        let text_bytes = make_text_from_lsk(&lsk);
        let meta = resolve::text_metadata();
        let wt = resolve::text_view_wt();
        let vwt = &*get_value_witness_table(meta);
        ViewExistential::new(
            text_bytes.as_ptr() as *const c_void,
            vwt.size,
            meta,
            wt,
        )
    }
}

/// Create an EmptyView. Pure Rust.
pub fn empty_view() -> ViewExistential {
    unsafe {
        let meta = resolve::empty_view_metadata();
        let wt = resolve::empty_view_view_wt();
        // EmptyView is zero-size
        ViewExistential::from_zero_size(meta, wt)
    }
}

/// Create a Divider. Pure Rust.
pub fn divider() -> ViewExistential {
    unsafe {
        let init_fn = resolve::divider_init();
        let meta = resolve::divider_metadata();
        let wt = resolve::divider_view_wt();
        let vwt = &*get_value_witness_table(meta);

        if vwt.size == 0 {
            // Zero-size type — just call init (no return value) and use from_zero_size
            #[cfg(target_arch = "aarch64")]
            {
                core::arch::asm!(
                    "blr {func}",
                    func = in(reg) init_fn,
                    in("x20") meta,
                    lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
                    lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
                    lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
                    lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
                    lateout("x16") _, lateout("x17") _, lateout("lr") _,
                    clobber_abi("C"),
                );
            }
            return ViewExistential::from_zero_size(meta, wt);
        }

        let mut buf = vec![0u8; vwt.size];
        #[cfg(target_arch = "aarch64")]
        {
            let r0: u64;
            core::arch::asm!(
                "blr {func}",
                func = in(reg) init_fn,
                in("x20") meta,
                lateout("x0") r0,
                lateout("x1") _, lateout("x2") _, lateout("x3") _,
                lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
                lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
                lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
                lateout("x16") _, lateout("x17") _, lateout("lr") _,
                clobber_abi("C"),
            );
            buf[..8.min(vwt.size)].copy_from_slice(&r0.to_le_bytes()[..8.min(vwt.size)]);
        }

        ViewExistential::new(buf.as_ptr() as *const c_void, vwt.size, meta, wt)
    }
}

/// Create an Image from an SF Symbol name. Pure Rust.
pub fn system_image(name: &str) -> ViewExistential {
    unsafe {
        let swift_str = make_swift_string(name);
        let init_fn = resolve::image_systemname_init();
        let meta = resolve::image_metadata();
        let wt = resolve::image_view_wt();
        let vwt = &*get_value_witness_table(meta);
        let mut buf = vec![0u8; vwt.size];

        #[cfg(target_arch = "aarch64")]
        {
            let s0 = u64::from_le_bytes(swift_str[..8].try_into().unwrap());
            let s1 = u64::from_le_bytes(swift_str[8..].try_into().unwrap());
            let r0: u64;
            core::arch::asm!(
                "blr {func}",
                func = in(reg) init_fn,
                in("x0") s0,
                in("x1") s1,
                in("x20") meta,
                lateout("x0") r0,
                lateout("x1") _, lateout("x2") _, lateout("x3") _,
                lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
                lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
                lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
                lateout("x16") _, lateout("x17") _, lateout("lr") _,
                clobber_abi("C"),
            );
            buf[..8].copy_from_slice(&r0.to_le_bytes());
        }

        ViewExistential::new(buf.as_ptr() as *const c_void, vwt.size, meta, wt)
    }
}
