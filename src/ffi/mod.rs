//! Raw FFI declarations matching the Swift `la_*` bridge exports.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::{c_char, c_void};

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -10_000;
    pub const TIMED_OUT: i32 = -10_001;
    pub const BRIDGE_FAILED: i32 = -10_002;
    pub const UNKNOWN: i32 = -10_099;
}

pub mod policy {
    pub const DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS: i32 = 1;
    pub const DEVICE_OWNER_AUTHENTICATION: i32 = 2;
    pub const DEVICE_OWNER_AUTHENTICATION_WITH_COMPANION: i32 = 3;
    pub const DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_COMPANION: i32 = 4;
}

pub mod biometry {
    pub const NONE: i32 = 0;
    pub const TOUCH_ID: i32 = 1;
    pub const FACE_ID: i32 = 2;
    pub const OPTIC_ID: i32 = 4;
}

pub mod la_error {
    pub const AUTHENTICATION_FAILED: i32 = -1;
    pub const USER_CANCEL: i32 = -2;
    pub const USER_FALLBACK: i32 = -3;
    pub const SYSTEM_CANCEL: i32 = -4;
    pub const PASSCODE_NOT_SET: i32 = -5;
    pub const BIOMETRY_NOT_AVAILABLE: i32 = -6;
    pub const BIOMETRY_NOT_ENROLLED: i32 = -7;
    pub const BIOMETRY_LOCKOUT: i32 = -8;
    pub const APP_CANCEL: i32 = -9;
    pub const INVALID_CONTEXT: i32 = -10;
    pub const COMPANION_NOT_AVAILABLE: i32 = -11;
    pub const BIOMETRY_NOT_PAIRED: i32 = -12;
    pub const BIOMETRY_DISCONNECTED: i32 = -13;
    pub const INVALID_DIMENSIONS: i32 = -14;
    pub const NOT_INTERACTIVE: i32 = -1004;
}

extern "C" {
    pub fn la_context_new(out_context: *mut *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn la_context_release(context: *mut c_void);
    pub fn la_context_invalidate(context: *mut c_void, error_out: *mut *mut c_char) -> i32;

    pub fn la_context_can_evaluate_policy(
        context: *mut c_void,
        policy: i32,
        out_can_evaluate: *mut u8,
        out_error_code: *mut i32,
        out_error_message: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_evaluate_policy(
        context: *mut c_void,
        policy: i32,
        localized_reason: *const c_char,
        out_success: *mut u8,
        out_error_code: *mut i32,
        out_error_message: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_context_get_localized_fallback_title(
        context: *mut c_void,
        out_title: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_set_localized_fallback_title(
        context: *mut c_void,
        title: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_get_localized_cancel_title(
        context: *mut c_void,
        out_title: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_set_localized_cancel_title(
        context: *mut c_void,
        title: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_context_get_allowable_reuse_duration(
        context: *mut c_void,
        out_duration: *mut f64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_set_allowable_reuse_duration(
        context: *mut c_void,
        duration: f64,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_context_get_interaction_not_allowed(
        context: *mut c_void,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_set_interaction_not_allowed(
        context: *mut c_void,
        value: u8,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_context_get_biometry_type(
        context: *mut c_void,
        out_biometry_type: *mut i32,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_get_evaluated_policy_domain_state(
        context: *mut c_void,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
}
