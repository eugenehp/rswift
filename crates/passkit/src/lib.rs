#![allow(unsafe_op_in_unsafe_fn)]
//! Apple PassKit — Apple Pay and passes from Rust.
//!
//! **Platform:** macOS 11+, iOS 8+, watchOS 3+.
//!
//! ```ignore
//! if passkit::can_make_payments() {
//!     println!("Apple Pay available");
//!     let networks = passkit::available_networks();
//!     println!("Networks: {:?}", networks);
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

/// Whether the device can make Apple Pay payments.
pub fn can_make_payments() -> bool {
    unsafe { msg_send_t![bool; class!(b"PKPaymentAuthorizationController\0"), canMakePayments] }
}

/// Whether payments can be made with specific networks.
pub fn can_make_payments_with_networks(networks: &[&str]) -> bool {
    unsafe {
        let ns_arr = make_nsarray(networks);
        let r = msg_send_t![bool; class!(b"PKPaymentAuthorizationController\0"),
            canMakePaymentsUsingNetworks: ns_arr];
        r
    }
}

/// Well-known payment network identifiers.
pub mod networks {
    pub const VISA: &str = "PKPaymentNetworkVisa";
    pub const MASTERCARD: &str = "PKPaymentNetworkMasterCard";
    pub const AMEX: &str = "PKPaymentNetworkAmex";
    pub const DISCOVER: &str = "PKPaymentNetworkDiscover";
    pub const JCB: &str = "PKPaymentNetworkJCB";
    pub const CHINA_UNION_PAY: &str = "PKPaymentNetworkChinaUnionPay";
    pub const INTERAC: &str = "PKPaymentNetworkInterac";
    pub const EFTPOS: &str = "PKPaymentNetworkEftpos";
}

/// Merchant capability flags.
pub mod capabilities {
    pub const DEBIT: usize = 1;
    pub const CREDIT: usize = 2;
    pub const THREE_DS: usize = 4;
    pub const EMV: usize = 8;
}

/// A payment summary item (line item on the payment sheet).
#[derive(Debug, Clone)]
pub struct SummaryItem {
    pub label: String,
    /// Amount as a decimal string (e.g. "9.99").
    pub amount: String,
}

/// Build a PKPaymentRequest (returns raw pointer for use with PKPaymentAuthorizationController).
pub fn create_payment_request(
    merchant_id: &str,
    country_code: &str,
    currency_code: &str,
    supported_networks: &[&str],
    items: &[SummaryItem],
) -> Id {
    unsafe {
        let req: Id = msg_send![class!(b"PKPaymentRequest\0"), new];

        let mid = nsstring(merchant_id);
        msg_send_void![req, setMerchantIdentifier: mid];
        CFRelease(mid as CFTypeRef);

        let cc = nsstring(country_code);
        msg_send_void![req, setCountryCode: cc];
        CFRelease(cc as CFTypeRef);

        let cur = nsstring(currency_code);
        msg_send_void![req, setCurrencyCode: cur];
        CFRelease(cur as CFTypeRef);

        let nets = make_nsarray(supported_networks);
        msg_send_void![req, setSupportedNetworks: nets];

        // Set merchant capabilities (3DS)
        let sel = sel_registerName(b"setMerchantCapabilities:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, usize) =
            core::mem::transmute(objc_msgSend as *const ());
        f(req, sel, capabilities::THREE_DS);

        // Build summary items
        let mut item_ptrs = Vec::with_capacity(items.len());
        for item in items {
            let label = nsstring(&item.label);
            let amount_ns = nsstring(&item.amount);
            let decimal: Id = msg_send![class!(b"NSDecimalNumber\0"), decimalNumberWithString: amount_ns];
            let si: Id = msg_send![class!(b"PKPaymentSummaryItem\0"), summaryItemWithLabel: label, amount: decimal];
            item_ptrs.push(si);
            CFRelease(label as CFTypeRef);
            CFRelease(amount_ns as CFTypeRef);
        }
        let sel_arr = sel_registerName(b"arrayWithObjects:count:\0".as_ptr());
        let f_arr: unsafe extern "C" fn(Id, Sel, *const Id, usize) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let items_arr = f_arr(class!(b"NSArray\0") as Id, sel_arr, item_ptrs.as_ptr(), item_ptrs.len());
        msg_send_void![req, setPaymentSummaryItems: items_arr];

        req
    }
}

unsafe fn make_nsarray(strings: &[&str]) -> Id {
    let ns_strs: Vec<Id> = strings.iter().map(|s| nsstring(s)).collect();
    let sel = sel_registerName(b"arrayWithObjects:count:\0".as_ptr());
    let f: unsafe extern "C" fn(Id, Sel, *const Id, usize) -> Id =
        core::mem::transmute(objc_msgSend as *const ());
    let arr = f(class!(b"NSArray\0") as Id, sel, ns_strs.as_ptr(), ns_strs.len());
    for s in &ns_strs { CFRelease(*s as CFTypeRef); }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_make_payments() {
        let _ = can_make_payments();
    }

    #[test]
    fn test_summary_item() {
        let item = SummaryItem { label: "Widget".into(), amount: "9.99".into() };
        assert_eq!(item.amount, "9.99");
    }
}
