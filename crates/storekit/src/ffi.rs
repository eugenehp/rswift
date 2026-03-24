//! ObjC selector constants for StoreKit.
#![allow(dead_code)]

// ── SKPaymentQueue (14 methods, 0 properties) ──
pub mod s_k_payment_queue {
    pub const CLASS: &[u8] = b"SKPaymentQueue\0";
    pub const SEL_DEFAULT_QUEUE: &[u8] = b"defaultQueue\0";
    pub const SEL_CAN_MAKE_PAYMENTS: &[u8] = b"canMakePayments\0";
    pub const SEL_ADD_PAYMENT: &[u8] = b"addPayment:\0";
    pub const SEL_RESTORE_COMPLETED_TRANSACTIONS: &[u8] = b"restoreCompletedTransactions\0";
    pub const SEL_RESTORE_COMPLETED_TRANSACTIONS_WITH_APPLICATION_USERNAME: &[u8] = b"restoreCompletedTransactionsWithApplicationUsername:\0";
    pub const SEL_FINISH_TRANSACTION: &[u8] = b"finishTransaction:\0";
    pub const SEL_START_DOWNLOADS: &[u8] = b"startDownloads:\0";
    pub const SEL_PAUSE_DOWNLOADS: &[u8] = b"pauseDownloads:\0";
    pub const SEL_RESUME_DOWNLOADS: &[u8] = b"resumeDownloads:\0";
    pub const SEL_CANCEL_DOWNLOADS: &[u8] = b"cancelDownloads:\0";
    pub const SEL_ADD_TRANSACTION_OBSERVER: &[u8] = b"addTransactionObserver:\0";
    pub const SEL_REMOVE_TRANSACTION_OBSERVER: &[u8] = b"removeTransactionObserver:\0";
    pub const SEL_SHOW_PRICE_CONSENT_IF_NEEDED: &[u8] = b"showPriceConsentIfNeeded\0";
    pub const SEL_PRESENT_CODE_REDEMPTION_SHEET: &[u8] = b"presentCodeRedemptionSheet\0";
}

// ── SKProduct (0 methods, 0 properties) ──
pub mod s_k_product {
}

// ── SKProductsRequest (0 methods, 1 properties) ──
pub mod s_k_products_request {
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
}

// ── SKPayment (2 methods, 1 properties) ──
pub mod s_k_payment {
    pub const SEL_PRODUCT_IDENTIFIER: &[u8] = b"productIdentifier\0";
    pub const SEL_SET_PRODUCT_IDENTIFIER: &[u8] = b"setProductIdentifier:\0";
    pub const SEL_PAYMENT_WITH_PRODUCT: &[u8] = b"paymentWithProduct:\0";
    pub const SEL_PAYMENT_WITH_PRODUCT_IDENTIFIER: &[u8] = b"paymentWithProductIdentifier:\0";
}

// ── SKTransaction (0 methods, 0 properties) ──
pub mod s_k_transaction {
}

// Total: 20 selector constants
