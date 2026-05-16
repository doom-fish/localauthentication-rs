use core::ffi::{c_char, c_void};

pub const STATE_UNKNOWN: i32 = 0;
pub const STATE_AUTHORIZING: i32 = 1;
pub const STATE_AUTHORIZED: i32 = 2;
pub const STATE_NOT_AUTHORIZED: i32 = 3;

extern "C" {
    pub fn la_right_new(out_right: *mut *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn la_right_new_with_requirement(
        requirement: *mut c_void,
        out_right: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_right_release(right: *mut c_void);
    pub fn la_right_get_state(
        right: *mut c_void,
        out_state: *mut i32,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_right_get_tag(
        right: *mut c_void,
        out_tag: *mut i64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_right_set_tag(right: *mut c_void, tag: i64, error_out: *mut *mut c_char) -> i32;
    pub fn la_right_authorize(
        right: *mut c_void,
        localized_reason: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_right_check_can_authorize(right: *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn la_right_deauthorize(right: *mut c_void, error_out: *mut *mut c_char) -> i32;
}
