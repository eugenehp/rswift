//! ObjC selector constants for PassKit.
#![allow(dead_code)]

// ── PKPaymentRequest (7 methods, 9 properties) ──
pub mod p_k_payment_request {
    pub const CLASS: &[u8] = b"PKPaymentRequest\0";
    pub const SEL_MERCHANT_IDENTIFIER: &[u8] = b"merchantIdentifier\0";
    pub const SEL_SET_MERCHANT_IDENTIFIER: &[u8] = b"setMerchantIdentifier:\0";
    pub const SEL_COUNTRY_CODE: &[u8] = b"countryCode\0";
    pub const SEL_SET_COUNTRY_CODE: &[u8] = b"setCountryCode:\0";
    pub const SEL_SUPPORTED_NETWORKS: &[u8] = b"supportedNetworks\0";
    pub const SEL_SET_SUPPORTED_NETWORKS: &[u8] = b"setSupportedNetworks:\0";
    pub const SEL_MERCHANT_CAPABILITIES: &[u8] = b"merchantCapabilities\0";
    pub const SEL_SET_MERCHANT_CAPABILITIES: &[u8] = b"setMerchantCapabilities:\0";
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
    pub const SEL_PAYMENT_SUMMARY_ITEMS: &[u8] = b"paymentSummaryItems\0";
    pub const SEL_SET_PAYMENT_SUMMARY_ITEMS: &[u8] = b"setPaymentSummaryItems:\0";
    pub const SEL_CURRENCY_CODE: &[u8] = b"currencyCode\0";
    pub const SEL_SET_CURRENCY_CODE: &[u8] = b"setCurrencyCode:\0";
    pub const SEL_SHIPPING_METHODS: &[u8] = b"shippingMethods\0";
    pub const SEL_SET_SHIPPING_METHODS: &[u8] = b"setShippingMethods:\0";
    pub const SEL_APPLICATION_DATA: &[u8] = b"applicationData\0";
    pub const SEL_SET_APPLICATION_DATA: &[u8] = b"setApplicationData:\0";
    pub const SEL_AVAILABLE_NETWORKS: &[u8] = b"availableNetworks\0";
    pub const SEL_PAYMENT_CONTACT_INVALID_ERROR_WITH_CONTACT_FIELD: &[u8] = b"paymentContactInvalidErrorWithContactField:localizedDescription:\0";
    pub const SEL_PAYMENT_SHIPPING_ADDRESS_INVALID_ERROR_WITH_KEY: &[u8] = b"paymentShippingAddressInvalidErrorWithKey:localizedDescription:\0";
    pub const SEL_PAYMENT_BILLING_ADDRESS_INVALID_ERROR_WITH_KEY: &[u8] = b"paymentBillingAddressInvalidErrorWithKey:localizedDescription:\0";
    pub const SEL_PAYMENT_SHIPPING_ADDRESS_UNSERVICEABLE_ERROR_WITH_LOCALIZED_DESCRIPTION: &[u8] = b"paymentShippingAddressUnserviceableErrorWithLocalizedDescription:\0";
    pub const SEL_PAYMENT_COUPON_CODE_INVALID_ERROR_WITH_LOCALIZED_DESCRIPTION: &[u8] = b"paymentCouponCodeInvalidErrorWithLocalizedDescription:\0";
    pub const SEL_PAYMENT_COUPON_CODE_EXPIRED_ERROR_WITH_LOCALIZED_DESCRIPTION: &[u8] = b"paymentCouponCodeExpiredErrorWithLocalizedDescription:\0";
}

// ── PKPaymentAuthorizationController (8 methods, 1 properties) ──
pub mod p_k_payment_authorization_controller {
    pub const CLASS: &[u8] = b"PKPaymentAuthorizationController\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_CAN_MAKE_PAYMENTS: &[u8] = b"canMakePayments\0";
    pub const SEL_CAN_MAKE_PAYMENTS_USING_NETWORKS: &[u8] = b"canMakePaymentsUsingNetworks:\0";
    pub const SEL_SUPPORTS_DISBURSEMENTS: &[u8] = b"supportsDisbursements\0";
    pub const SEL_SUPPORTS_DISBURSEMENTS_USING_NETWORKS: &[u8] = b"supportsDisbursementsUsingNetworks:\0";
}

// ── PKPaymentSummaryItem (2 methods, 2 properties) ──
pub mod p_k_payment_summary_item {
    pub const CLASS: &[u8] = b"PKPaymentSummaryItem\0";
    pub const SEL_LABEL: &[u8] = b"label\0";
    pub const SEL_SET_LABEL: &[u8] = b"setLabel:\0";
    pub const SEL_AMOUNT: &[u8] = b"amount\0";
    pub const SEL_SET_AMOUNT: &[u8] = b"setAmount:\0";
    pub const SEL_SUMMARY_ITEM_WITH_LABEL: &[u8] = b"summaryItemWithLabel:amount:\0";
}

// ── PKPass (1 methods, 9 properties) ──
pub mod p_k_pass {
    pub const CLASS: &[u8] = b"PKPass\0";
    pub const SEL_SERIAL_NUMBER: &[u8] = b"serialNumber\0";
    pub const SEL_SET_SERIAL_NUMBER: &[u8] = b"setSerialNumber:\0";
    pub const SEL_PASS_TYPE_IDENTIFIER: &[u8] = b"passTypeIdentifier\0";
    pub const SEL_SET_PASS_TYPE_IDENTIFIER: &[u8] = b"setPassTypeIdentifier:\0";
    pub const SEL_WEB_SERVICE_U_R_L: &[u8] = b"webServiceURL\0";
    pub const SEL_SET_WEB_SERVICE_U_R_L: &[u8] = b"setWebServiceURL:\0";
    pub const SEL_AUTHENTICATION_TOKEN: &[u8] = b"authenticationToken\0";
    pub const SEL_SET_AUTHENTICATION_TOKEN: &[u8] = b"setAuthenticationToken:\0";
    pub const SEL_W_A_T_C_H_O_S_P_R_O_H_I_B_I_T_E_D: &[u8] = b"__WATCHOS_PROHIBITED\0";
    pub const SEL_SET_W_A_T_C_H_O_S_P_R_O_H_I_B_I_T_E_D: &[u8] = b"set__WATCHOS_PROHIBITED:\0";
    pub const SEL_LOCALIZED_NAME: &[u8] = b"localizedName\0";
    pub const SEL_SET_LOCALIZED_NAME: &[u8] = b"setLocalizedName:\0";
    pub const SEL_LOCALIZED_DESCRIPTION: &[u8] = b"localizedDescription\0";
    pub const SEL_SET_LOCALIZED_DESCRIPTION: &[u8] = b"setLocalizedDescription:\0";
    pub const SEL_ORGANIZATION_NAME: &[u8] = b"organizationName\0";
    pub const SEL_SET_ORGANIZATION_NAME: &[u8] = b"setOrganizationName:\0";
    pub const SEL_PASS_U_R_L: &[u8] = b"passURL\0";
    pub const SEL_SET_PASS_U_R_L: &[u8] = b"setPassURL:\0";
    pub const SEL_LOCALIZED_VALUE_FOR_FIELD_KEY: &[u8] = b"localizedValueForFieldKey:\0";
}

// ── PKPassLibrary (28 methods, 0 properties) ──
pub mod p_k_pass_library {
    pub const CLASS: &[u8] = b"PKPassLibrary\0";
    pub const SEL_IS_PASS_LIBRARY_AVAILABLE: &[u8] = b"isPassLibraryAvailable\0";
    pub const SEL_END_AUTOMATIC_PASS_PRESENTATION_SUPPRESSION_WITH_REQUEST_TOKEN: &[u8] = b"endAutomaticPassPresentationSuppressionWithRequestToken:\0";
    pub const SEL_IS_SUPPRESSING_AUTOMATIC_PASS_PRESENTATION: &[u8] = b"isSuppressingAutomaticPassPresentation\0";
    pub const SEL_IS_PAYMENT_PASS_ACTIVATION_AVAILABLE: &[u8] = b"isPaymentPassActivationAvailable\0";
    pub const SEL_PASSES: &[u8] = b"passes\0";
    pub const SEL_PASS_WITH_PASS_TYPE_IDENTIFIER: &[u8] = b"passWithPassTypeIdentifier:serialNumber:\0";
    pub const SEL_PASSES_WITH_READER_IDENTIFIER: &[u8] = b"passesWithReaderIdentifier:\0";
    pub const SEL_PASSES_OF_TYPE: &[u8] = b"passesOfType:\0";
    pub const SEL_REMOTE_PAYMENT_PASSES: &[u8] = b"remotePaymentPasses\0";
    pub const SEL_REMOVE_PASS: &[u8] = b"removePass:\0";
    pub const SEL_CONTAINS_PASS: &[u8] = b"containsPass:\0";
    pub const SEL_REPLACE_PASS_WITH_PASS: &[u8] = b"replacePassWithPass:\0";
    pub const SEL_OPEN_PAYMENT_SETUP: &[u8] = b"openPaymentSetup\0";
    pub const SEL_PRESENT_PAYMENT_PASS: &[u8] = b"presentPaymentPass:\0";
    pub const SEL_PRESENT_SECURE_ELEMENT_PASS: &[u8] = b"presentSecureElementPass:\0";
    pub const SEL_CAN_ADD_PAYMENT_PASS_WITH_PRIMARY_ACCOUNT_IDENTIFIER: &[u8] = b"canAddPaymentPassWithPrimaryAccountIdentifier:\0";
    pub const SEL_CAN_ADD_SECURE_ELEMENT_PASS_WITH_PRIMARY_ACCOUNT_IDENTIFIER: &[u8] = b"canAddSecureElementPassWithPrimaryAccountIdentifier:\0";
    pub const SEL_CAN_ADD_FELICA_PASS: &[u8] = b"canAddFelicaPass\0";
    pub const SEL_AUTHORIZATION_STATUS_FOR_CAPABILITY: &[u8] = b"authorizationStatusForCapability:\0";
}

// Total: 84 selector constants
