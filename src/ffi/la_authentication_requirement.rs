use core::ffi::{c_char, c_void};

extern "C" {
    pub fn la_authentication_requirement_default(
        out_requirement: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_authentication_requirement_biometry(
        out_requirement: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_authentication_requirement_biometry_current_set(
        out_requirement: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_authentication_requirement_biometry_with_fallback(
        fallback: *mut c_void,
        out_requirement: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_authentication_requirement_release(requirement: *mut c_void);

    pub fn la_biometry_fallback_requirement_default(
        out_requirement: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_biometry_fallback_requirement_device_passcode(
        out_requirement: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_biometry_fallback_requirement_release(requirement: *mut c_void);
}
