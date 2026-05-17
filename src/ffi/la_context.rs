use core::ffi::{c_char, c_void};

pub const ACCESS_CONTROL_OPERATION_CREATE_ITEM: i32 = 0;
pub const ACCESS_CONTROL_OPERATION_USE_ITEM: i32 = 1;
pub const ACCESS_CONTROL_OPERATION_CREATE_KEY: i32 = 2;
pub const ACCESS_CONTROL_OPERATION_USE_KEY_SIGN: i32 = 3;
pub const ACCESS_CONTROL_OPERATION_USE_KEY_DECRYPT: i32 = 4;
pub const ACCESS_CONTROL_OPERATION_USE_KEY_KEY_EXCHANGE: i32 = 5;

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
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_evaluate_access_control(
        context: *mut c_void,
        access_control: *const c_void,
        operation: i32,
        localized_reason: *const c_char,
        out_success: *mut u8,
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
    pub fn la_context_get_localized_reason(
        context: *mut c_void,
        out_reason: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_set_localized_reason(
        context: *mut c_void,
        localized_reason: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_context_get_touch_id_authentication_allowable_reuse_duration(
        context: *mut c_void,
        out_duration: *mut f64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_set_touch_id_authentication_allowable_reuse_duration(
        context: *mut c_void,
        duration: f64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_get_touch_id_authentication_maximum_allowable_reuse_duration() -> f64;

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

    pub fn la_context_set_credential(
        context: *mut c_void,
        credential_bytes: *const u8,
        credential_len: usize,
        credential_type: i32,
        has_credential: u8,
        out_was_set: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_is_credential_set(
        context: *mut c_void,
        credential_type: i32,
        out_is_set: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_context_get_domain_state_hash(
        context: *mut c_void,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_get_domain_state_biometry_type(
        context: *mut c_void,
        out_biometry_type: *mut i32,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_get_domain_state_biometry_hash(
        context: *mut c_void,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_get_domain_state_companion_types(
        context: *mut c_void,
        out_values: *mut *mut i32,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_get_domain_state_companion_hash(
        context: *mut c_void,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_context_get_domain_state_companion_hash_for_type(
        context: *mut c_void,
        companion_type: i32,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;

    // Async APIs
    pub fn la_context_evaluate_policy_async(
        context: *mut c_void,
        policy: i32,
        localized_reason: *const c_char,
        cb: extern "C" fn(u8, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );

    pub fn la_context_evaluate_access_control_async(
        context: *mut c_void,
        access_control: *const c_void,
        operation: i32,
        localized_reason: *const c_char,
        cb: extern "C" fn(u8, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );
}
