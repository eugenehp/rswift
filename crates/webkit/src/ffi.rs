//! ObjC selector constants for WebKit.
#![allow(dead_code)]

// ── WKWebView (41 methods, 15 properties) ──
pub mod w_k_web_view {
    pub const CLASS: &[u8] = b"WKWebView\0";
    pub const SEL_CONFIGURATION: &[u8] = b"configuration\0";
    pub const SEL_SET_CONFIGURATION: &[u8] = b"setConfiguration:\0";
    pub const SEL_NAVIGATION_DELEGATE: &[u8] = b"navigationDelegate\0";
    pub const SEL_SET_NAVIGATION_DELEGATE: &[u8] = b"setNavigationDelegate:\0";
    pub const SEL_U_I_DELEGATE: &[u8] = b"UIDelegate\0";
    pub const SEL_SET_U_I_DELEGATE: &[u8] = b"setUIDelegate:\0";
    pub const SEL_BACK_FORWARD_LIST: &[u8] = b"backForwardList\0";
    pub const SEL_SET_BACK_FORWARD_LIST: &[u8] = b"setBackForwardList:\0";
    pub const SEL_TITLE: &[u8] = b"title\0";
    pub const SEL_SET_TITLE: &[u8] = b"setTitle:\0";
    pub const SEL_U_R_L: &[u8] = b"URL\0";
    pub const SEL_SET_U_R_L: &[u8] = b"setURL:\0";
    pub const SEL_LOADING: &[u8] = b"loading\0";
    pub const SEL_SET_LOADING: &[u8] = b"setLoading:\0";
    pub const SEL_ESTIMATED_PROGRESS: &[u8] = b"estimatedProgress\0";
    pub const SEL_SET_ESTIMATED_PROGRESS: &[u8] = b"setEstimatedProgress:\0";
    pub const SEL_HAS_ONLY_SECURE_CONTENT: &[u8] = b"hasOnlySecureContent\0";
    pub const SEL_SET_HAS_ONLY_SECURE_CONTENT: &[u8] = b"setHasOnlySecureContent:\0";
    pub const SEL_CAN_GO_BACK: &[u8] = b"canGoBack\0";
    pub const SEL_SET_CAN_GO_BACK: &[u8] = b"setCanGoBack:\0";
    pub const SEL_CAN_GO_FORWARD: &[u8] = b"canGoForward\0";
    pub const SEL_SET_CAN_GO_FORWARD: &[u8] = b"setCanGoForward:\0";
    pub const SEL_ALLOWS_BACK_FORWARD_NAVIGATION_GESTURES: &[u8] = b"allowsBackForwardNavigationGestures\0";
    pub const SEL_SET_ALLOWS_BACK_FORWARD_NAVIGATION_GESTURES: &[u8] = b"setAllowsBackForwardNavigationGestures:\0";
    pub const SEL_SCROLL_VIEW: &[u8] = b"scrollView\0";
    pub const SEL_SET_SCROLL_VIEW: &[u8] = b"setScrollView:\0";
    pub const SEL_ALLOWS_MAGNIFICATION: &[u8] = b"allowsMagnification\0";
    pub const SEL_SET_ALLOWS_MAGNIFICATION: &[u8] = b"setAllowsMagnification:\0";
    pub const SEL_MAGNIFICATION: &[u8] = b"magnification\0";
    pub const SEL_SET_MAGNIFICATION: &[u8] = b"setMagnification:\0";
    pub const SEL_LOAD_REQUEST: &[u8] = b"loadRequest:\0";
    pub const SEL_LOAD_FILE_U_R_L: &[u8] = b"loadFileURL:allowingReadAccessToURL:\0";
    pub const SEL_LOAD_H_T_M_L_STRING: &[u8] = b"loadHTMLString:baseURL:\0";
    pub const SEL_LOAD_DATA: &[u8] = b"loadData:MIMEType:characterEncodingName:baseURL:\0";
    pub const SEL_GO_TO_BACK_FORWARD_LIST_ITEM: &[u8] = b"goToBackForwardListItem:\0";
    pub const SEL_GO_BACK: &[u8] = b"goBack\0";
    pub const SEL_GO_FORWARD: &[u8] = b"goForward\0";
    pub const SEL_RELOAD: &[u8] = b"reload\0";
    pub const SEL_RELOAD_FROM_ORIGIN: &[u8] = b"reloadFromOrigin\0";
    pub const SEL_STOP_LOADING: &[u8] = b"stopLoading\0";
    pub const SEL_CLOSE_ALL_MEDIA_PRESENTATIONS: &[u8] = b"closeAllMediaPresentations\0";
    pub const SEL_HANDLES_U_R_L_SCHEME: &[u8] = b"handlesURLScheme:\0";
    pub const SEL_LOAD_SIMULATED_REQUEST: &[u8] = b"loadSimulatedRequest:response:responseData:\0";
    pub const SEL_LOAD_FILE_REQUEST: &[u8] = b"loadFileRequest:allowingReadAccessToURL:\0";
    pub const SEL_PRINT_OPERATION_WITH_PRINT_INFO: &[u8] = b"printOperationWithPrintInfo:\0";
    pub const SEL_SET_MINIMUM_VIEWPORT_INSET: &[u8] = b"setMinimumViewportInset:maximumViewportInset:\0";
}

// ── WKWebViewConfiguration (2 methods, 5 properties) ──
pub mod w_k_web_view_configuration {
    pub const SEL_PREFERENCES: &[u8] = b"preferences\0";
    pub const SEL_SET_PREFERENCES: &[u8] = b"setPreferences:\0";
    pub const SEL_USER_CONTENT_CONTROLLER: &[u8] = b"userContentController\0";
    pub const SEL_SET_USER_CONTENT_CONTROLLER: &[u8] = b"setUserContentController:\0";
    pub const SEL_SUPPRESSES_INCREMENTAL_RENDERING: &[u8] = b"suppressesIncrementalRendering\0";
    pub const SEL_SET_SUPPRESSES_INCREMENTAL_RENDERING: &[u8] = b"setSuppressesIncrementalRendering:\0";
    pub const SEL_ALLOWS_INLINE_MEDIA_PLAYBACK: &[u8] = b"allowsInlineMediaPlayback\0";
    pub const SEL_SET_ALLOWS_INLINE_MEDIA_PLAYBACK: &[u8] = b"setAllowsInlineMediaPlayback:\0";
    pub const SEL_IGNORED: &[u8] = b"ignored\0";
    pub const SEL_SET_IGNORED: &[u8] = b"setIgnored:\0";
    pub const SEL_SET_U_R_L_SCHEME_HANDLER: &[u8] = b"setURLSchemeHandler:forURLScheme:\0";
    pub const SEL_URL_SCHEME_HANDLER_FOR_U_R_L_SCHEME: &[u8] = b"urlSchemeHandlerForURLScheme:\0";
}

// ── WKWebsiteDataStore (11 methods, 1 properties) ──
pub mod w_k_website_data_store {
    pub const SEL_PERSISTENT: &[u8] = b"persistent\0";
    pub const SEL_SET_PERSISTENT: &[u8] = b"setPersistent:\0";
    pub const SEL_DEFAULT_DATA_STORE: &[u8] = b"defaultDataStore\0";
    pub const SEL_NON_PERSISTENT_DATA_STORE: &[u8] = b"nonPersistentDataStore\0";
    pub const SEL_ALL_WEBSITE_DATA_TYPES: &[u8] = b"allWebsiteDataTypes\0";
    pub const SEL_DATA_STORE_FOR_IDENTIFIER: &[u8] = b"dataStoreForIdentifier:\0";
}

// ── WKUserContentController (12 methods, 1 properties) ──
pub mod w_k_user_content_controller {
    pub const SEL_USER_SCRIPTS: &[u8] = b"userScripts\0";
    pub const SEL_SET_USER_SCRIPTS: &[u8] = b"setUserScripts:\0";
    pub const SEL_ADD_USER_SCRIPT: &[u8] = b"addUserScript:\0";
    pub const SEL_REMOVE_ALL_USER_SCRIPTS: &[u8] = b"removeAllUserScripts\0";
    pub const SEL_ADD_SCRIPT_MESSAGE_HANDLER: &[u8] = b"addScriptMessageHandler:contentWorld:name:\0";
    pub const SEL_ADD_SCRIPT_MESSAGE_HANDLER_WITH_REPLY: &[u8] = b"addScriptMessageHandlerWithReply:contentWorld:name:\0";
    pub const SEL_REMOVE_SCRIPT_MESSAGE_HANDLER_FOR_NAME: &[u8] = b"removeScriptMessageHandlerForName:contentWorld:\0";
    pub const SEL_REMOVE_ALL_SCRIPT_MESSAGE_HANDLERS_FROM_CONTENT_WORLD: &[u8] = b"removeAllScriptMessageHandlersFromContentWorld:\0";
    pub const SEL_REMOVE_ALL_SCRIPT_MESSAGE_HANDLERS: &[u8] = b"removeAllScriptMessageHandlers\0";
    pub const SEL_ADD_CONTENT_RULE_LIST: &[u8] = b"addContentRuleList:\0";
    pub const SEL_REMOVE_CONTENT_RULE_LIST: &[u8] = b"removeContentRuleList:\0";
    pub const SEL_REMOVE_ALL_CONTENT_RULE_LISTS: &[u8] = b"removeAllContentRuleLists\0";
}

// ── WKUserScript (0 methods, 3 properties) ──
pub mod w_k_user_script {
    pub const SEL_SOURCE: &[u8] = b"source\0";
    pub const SEL_SET_SOURCE: &[u8] = b"setSource:\0";
    pub const SEL_INJECTION_TIME: &[u8] = b"injectionTime\0";
    pub const SEL_SET_INJECTION_TIME: &[u8] = b"setInjectionTime:\0";
    pub const SEL_FOR_MAIN_FRAME_ONLY: &[u8] = b"forMainFrameOnly\0";
    pub const SEL_SET_FOR_MAIN_FRAME_ONLY: &[u8] = b"setForMainFrameOnly:\0";
}

// ── WKHTTPCookieStore (8 methods, 0 properties) ──
pub mod w_k_h_t_t_p_cookie_store {
    pub const SEL_ADD_OBSERVER: &[u8] = b"addObserver:\0";
    pub const SEL_REMOVE_OBSERVER: &[u8] = b"removeObserver:\0";
}

// ── WKPreferences (0 methods, 2 properties) ──
pub mod w_k_preferences {
    pub const SEL_MINIMUM_FONT_SIZE: &[u8] = b"minimumFontSize\0";
    pub const SEL_SET_MINIMUM_FONT_SIZE: &[u8] = b"setMinimumFontSize:\0";
    pub const SEL_JAVA_SCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY: &[u8] = b"javaScriptCanOpenWindowsAutomatically\0";
    pub const SEL_SET_JAVA_SCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY: &[u8] = b"setJavaScriptCanOpenWindowsAutomatically:\0";
}

// Total: 121 selector constants
